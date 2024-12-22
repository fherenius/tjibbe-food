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

# Copy the built static files
COPY --from=builder /usr/src/app/target/dx/nutrient_calculator/release/web/public /usr/share/nginx/html

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
