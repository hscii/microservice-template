# Build
FROM rust:latest AS builder
WORKDIR /usr/src/service
COPY . .
RUN apt-get update && apt-get install -y cmake clang && cargo install --path .

# Environment Initialization
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/service /usr/local/bin/service
CMD ["service"]