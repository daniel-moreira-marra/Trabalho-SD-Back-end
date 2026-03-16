# Build stage
FROM rust:1.75-slim as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/lamport-backend /usr/local/bin/lamport-backend

EXPOSE 9000

CMD ["lamport-backend"]
