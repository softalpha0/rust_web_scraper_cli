# Stage 1: Build the Rust binary
FROM rust:1.82 AS builder

WORKDIR /app

# Copy manifests and dependencies
COPY Cargo.toml Cargo.lock ./
COPY . ./

# Build the binary
RUN cargo build --release --bin bot_main

# Stage 2: Slim runtime
FROM debian:bullseye-slim

# Copy compiled binary
COPY --from=builder /app/target/release/bot_main /usr/local/bin/bot_main

# Set entrypoint and default args
ENTRYPOINT ["/usr/local/bin/bot_main"]
CMD ["https://news.ycombinator.com"]