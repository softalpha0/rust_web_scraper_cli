# Use Rust image as build AND runtime (no glibc issues)
FROM rust:1.77

# Set working directory
WORKDIR /app

# Copy manifests and source
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

# Build the binary
RUN cargo build --release --bin bot_main

# Set entrypoint
ENTRYPOINT ["./target/release/bot_main"]
CMD ["https://news.ycombinator.com"]