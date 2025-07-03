FROM rust:1.82

WORKDIR /app

COPY . .

RUN cargo build --release --bin bot_main

CMD ["./target/release/bot_main", "https://news.ycombinator.com"]