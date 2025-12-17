# syntax=docker/dockerfile:1.6

FROM rust:1.88-slim AS builder
WORKDIR /app

# Install build dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev build-essential ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests and source
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY certs ./certs

# Build release binary
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates openssl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/wentu_backend /usr/local/bin/wentu_backend
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/certs ./certs

ENV RUST_LOG=info \
    SERVER_HOST=0.0.0.0 \
    SERVER_PORT=3000

EXPOSE 3000

CMD ["wentu_backend"]
