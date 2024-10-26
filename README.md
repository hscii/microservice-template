# rust-microservice-template

Simple, asynchronous, and efficient microservice using [`Rust ⇩`](https://www.rust-lang.org/tools/install), [Actix Web](https://actix.rs/docs/getting-started), and [`Docker ⇩`](https://docs.docker.com/engine/install/).

JWT auth middleware: https://github.com/hscii/jwt-middleware-crate

## Running

### > Run Locally (Cargo)

To run the service locally without using Docker, use the following command:

```
cargo run
```

### > Run Containerized (Docker)

To run this service in a Docker container, use the following commmands:

```
docker build -t rust-microservice-skeleton .
docker run --rm -p 8080:8080 rust-microservice-skeleton
```

Replace the follwing:

- `8080:8080` with your desired port mapping
- `rust-microservice-skeleton` with your service name

## Testing

To run all tests in the `/tests` directory:

```
cargo test
```
_Note, the default test initializes the server without the JWT auth middleware._
