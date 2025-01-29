# Build
FROM rust:latest AS builder
WORKDIR /usr/src/service
COPY . .
RUN apt-get update && apt-get install -y cmake clang && cargo install --path .

# Environment Initialization
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/service /usr/local/bin/service

# Set default binding address for containerized service
ENV BIND_ADDRESS=0.0.0.0:8080

EXPOSE 8080
CMD ["service"]
