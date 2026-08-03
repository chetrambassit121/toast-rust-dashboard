# Build stage
FROM rust:1-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release


# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend

EXPOSE 8080

CMD ["./backend"]