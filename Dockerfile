# Build stage
FROM rust:1.82-slim AS builder

# Install build dependencies
RUN rustup update nightly && \
    rustup default nightly && \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy only the files needed for build
COPY --link Cargo.toml Cargo.lock ./

# Fetch dependencies and create a dummy build to cache dependencies
RUN cargo fetch && \
    mkdir src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release --features tui && \
    rm -rf src/*.rs target/release/deps/nutrient_calculator*

COPY --link src ./src

# Build with specified features (default to tui if not specified)
ARG FEATURES=tui
RUN cargo build --release --features ${FEATURES}

FROM debian:bookworm-slim

# Install required shared libraries
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Create a non-root user and group named nonroot
RUN groupadd -r nonroot && useradd --no-log-init -r -g nonroot nonroot

# Use nonroot user
USER nonroot:nonroot

# Copy only the binary using --link for better layer caching
COPY --link --chmod=555 --chown=nonroot:nonroot --from=builder /usr/src/app/target/release/nutrient_calculator /usr/local/bin/

# Specify that this is a tui app that needs a terminal
ENV TERM=xterm-256color

# Set the entrypoint
CMD ["/usr/local/bin/nutrient_calculator"]
