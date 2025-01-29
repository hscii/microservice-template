# rust microservice-template

Simple asynchronous microservice using [`Rust ⇩`](https://www.rust-lang.org/tools/install), [Actix Web](https://actix.rs/docs/getting-started), and [`Docker ⇩`](https://docs.docker.com/engine/install/).

JWT auth middleware: https://github.com/hscii/jwt-middleware-crate

## Running

### > Run Locally (Cargo)

To run the service locally without using Docker, use the following command:

```
cargo run
```

By default, this will serve your application at [http://127.0.0.1:8080/](http://127.0.0.1:8080/)

You should get a `pong` response from: [http://127.0.0.1:8080/ping](http://127.0.0.1:8080/ping)

### > Run Containerized (Docker)

To run this service in a Docker container, use the following commands:

```
docker build -t microservice-template .
docker run --rm -p 8080:8080 microservice-template
```

The `BIND_ADDRESS` environment variable is already set in the Dockerfile to `0.0.0.0:8080`, so no additional configuration is needed for the container.

Use `-e RUST_LOG=debug` to enable debug logging.

Replace the following:

- `8080:8080` with your desired port mapping
- `microservice-template` with your service name

## Testing

To run all tests in the `/tests` directory:

```
cargo test
```

_Note, the default test initializes the server without the JWT auth middleware._
