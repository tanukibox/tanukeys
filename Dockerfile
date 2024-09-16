# Build stage
FROM rust:1.80

WORKDIR /app
COPY . .
RUN cargo build --release

CMD ["./target/release/main_http_api"]
