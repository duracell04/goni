# syntax=docker/dockerfile:1.6

# Builder
FROM rust:1.80 as builder
WORKDIR /workspace

# Cache dependencies first
COPY Cargo.toml ./
COPY goni-* ./goni-*/
RUN cargo metadata --no-deps --format-version 1 >/dev/null

# Build release binary for goni-http
RUN cargo build --release -p goni-http

# Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /workspace/target/release/goni-http /app/goni-http

ENV RUST_LOG=info
EXPOSE 7000

CMD ["/app/goni-http"]
