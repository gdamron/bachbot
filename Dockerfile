# ---- Build stage ----
FROM rust:latest AS builder

WORKDIR /app
COPY . .

# Install openssl dev for rumqttc TLS support
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim

# Install minimal deps for OpenSSL
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy release binary
COPY --from=builder /app/target/release/bachbot /app/bachbot

ENTRYPOINT ["./bachbot"]
