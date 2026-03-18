# Build stage
FROM rust:1.75-slim as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/Trab-SD-Back-end /usr/local/bin/Trab-SD-Back-end

EXPOSE 9000

CMD ["Trab-SD-Back-end"]