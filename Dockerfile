# Build stage
FROM rust:1.80 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM alpine:latest

WORKDIR /app
RUN apk add --no-cache libgcc
COPY --from=builder /app/target/release/main_http_api .

CMD ["./tanukeys"]
