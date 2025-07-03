# --- Stage 1: Build the Rust binary ---
FROM rust:1.72 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY bot.rs .  # if you use it
COPY lib.rs ./lib.rs  # if you use it

RUN cargo build --release --bin bot_main

# --- Stage 2: Run the compiled binary ---
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/bot_main /usr/local/bin/bot_main

# Set the entrypoint with default argument
CMD ["bot_main", "https://news.ycombinator.com"]