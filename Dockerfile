# Add new stage for asset compilation
FROM node:20-slim AS asset-builder

WORKDIR /usr/src/app

# Copy package files first for better caching
COPY --link package.json package-lock.json ./

# Install dependencies using the lock file
RUN npm ci

# Copy all source files for Tailwind to scan
COPY --link tailwind.config.js ./
COPY --link src ./src
COPY --link assets ./assets

# Build assets using npx directly
RUN npx tailwindcss -i ./assets/tailwind.css -o ./assets/output.css --minify

# Build stage
FROM rust:1.82-slim AS builder

# Install build dependencies
RUN rustup update nightly && \
    rustup default nightly && \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/* && \
    # Install WebAssembly target
    rustup target add wasm32-unknown-unknown && \
    # Install Dioxus CLI instead of trunk
    cargo install dioxus-cli

# Create working directory
WORKDIR /usr/src/app

# Copy only dependency files first
COPY --link Cargo.toml Cargo.lock ./

# Create a dummy src/main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    dx build --release && \
    rm -rf src

# Now copy the real source code and assets
COPY --link src ./src
COPY --from=asset-builder /usr/src/app/assets ./assets
COPY --link Dioxus.toml ./Dioxus.toml

# Build the actual application
RUN dx build --release

# Use official non-root nginx image
FROM nginxinc/nginx-unprivileged:alpine-slim

# Create nginx.conf first
COPY <<EOF /tmp/nginx.conf
worker_processes auto;
pid /tmp/nginx.pid;

events {
    worker_connections 1024;
    multi_accept on;
}

http {
    include       /etc/nginx/mime.types;
    default_type  application/octet-stream;

    # Optimization settings
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    keepalive_timeout 65;
    types_hash_max_size 2048;

    # Gzip settings
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types text/plain text/css text/xml application/json application/javascript application/xml+rss application/atom+xml image/svg+xml application/wasm;

    # Logging configuration
    access_log /dev/stdout;
    error_log /dev/stderr;

    server {
        listen 8080 default_server;
        server_name _;  # Catch-all server name
        root   /usr/share/nginx/html;

        # Security headers
        add_header X-Content-Type-Options nosniff;
        add_header X-Frame-Options "SAMEORIGIN";
        add_header X-XSS-Protection "1; mode=block";

        location / {
            try_files $uri $uri/ /index.html;
            expires 1h;
        }

        # Cache static assets
        location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg)$ {
            expires max;
            add_header Cache-Control "public, no-transform";
        }
    }
}
EOF

# Copy the nginx config
RUN cp /tmp/nginx.conf /etc/nginx/nginx.conf

# Copy the built static files
COPY --from=builder /usr/src/app/target/dx/nutrient_calculator/release/web/public /usr/share/nginx/html

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
