# Stage 1: Build the Rust binary
FROM rust:1.77 AS builder

WORKDIR /app

# Copy manifests and dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the bot_main binary
RUN cargo build --release --bin bot_main

# Stage 2: Lightweight runtime image
FROM debian:bullseye-slim

# Copy binary from builder
COPY --from=builder /app/target/release/bot_main /usr/local/bin/bot_main

# Set the binary as entrypoint and provide default arg
ENTRYPOINT ["/usr/local/bin/bot_main"]
CMD ["https://news.ycombinator.com"]