# Build stage
FROM rust:1.80 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM rust:1.80

WORKDIR /app
COPY --from=builder /app/target/release/tanukeys /app/

ENV PORT=3030 \
    RUST_LOG=main_http_api=trace,kernel=trace \
    DATABASE_URL=postgres://root:root@postgres:5432/tanukeys

CMD ["./tanukeys"]
