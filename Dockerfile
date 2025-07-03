# Use the official Rust image
FROM rust:1.72

# Set the working directory
WORKDIR /app

# Copy the manifest and lock files first
COPY Cargo.toml Cargo.lock ./

# Create an empty src dir and copy bin target to avoid cache busting
RUN mkdir src
COPY src/bot_main.rs src/

# Build the binary (caches this layer if code hasn't changed)
RUN cargo build --release --bin bot_main

# Copy the actual full source
COPY . .

# Rebuild to make sure all source is included
RUN cargo build --release --bin bot_main

# Set the binary as the entrypoint (pass default URL here if desired)
CMD ["./target/release/bot_main", "https://news.ycombinator.com"]