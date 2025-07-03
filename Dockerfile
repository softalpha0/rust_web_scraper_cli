# Stage 1: Build the Rust binary
FROM rust:1.77 AS builder

WORKDIR /app

# Copy manifests and lockfile
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the release binary
RUN cargo build --release --bin bot_main

# Stage 2: Create minimal runtime image
FROM debian:bullseye-slim

# Install needed system dependencies (if any)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the build stage
COPY --from=builder /app/target/release/bot_main /usr/local/bin/bot_main

# Set binary as entrypoint and give default URL as argument
ENTRYPOINT ["/usr/local/bin/bot_main"]
CMD ["https://news.ycombinator.com"]