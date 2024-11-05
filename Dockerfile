# Build
FROM rust:latest AS builder
WORKDIR /usr/src/template
COPY . .
RUN apt-get update && apt-get install -y cmake clang && cargo install --path .

# Environment Initialization
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/template /usr/local/bin/template
CMD ["template"]