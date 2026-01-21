# Build stage
FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSL - often required for Rust web services
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/css_workbench_backend /usr/local/bin/css_workbench_backend

WORKDIR /usr/local/bin
EXPOSE 3000
CMD ["css_workbench_backend"]