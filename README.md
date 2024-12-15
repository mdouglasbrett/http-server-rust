# Codecrafters HTTP Server in Rust

This project is an HTTP server written in Rust, created as part of the Codecrafters "Build your own HTTP server" challenge. The server supports basic HTTP functionalities such as handling GET and POST requests, serving static files, and echoing request bodies.

## Features

- **GET /echo/:message**: Echoes the message provided in the URL.
- **GET /user-agent**: Returns the `User-Agent` header from the request.
- **GET /files/:filename**: Serves static files from a specified directory.
- **POST /files/:filename**: Saves the request body as a file in the specified directory.
- **Gzip Compression**: Supports gzip compression for responses if requested by the client.

## Project Structure

- `src/config.rs`: Configuration handling for the server.
- `src/constants.rs`: Constants used throughout the server.
- `src/errors.rs`: Custom error types for the server.
- `src/handlers.rs`: Request handlers for different routes.
- `src/http/request.rs`: HTTP request parsing.
- `src/http/response.rs`: HTTP response generation.
- `src/main.rs`: Entry point of the application.
- `src/router.rs`: Request routing logic.
- `src/routes.rs`: Route definitions.
- `src/server.rs`: Server setup and connection handling.
- `src/utils.rs`: Utility functions.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)

### Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/codecrafters-http-server-rust.git
    cd codecrafters-http-server-rust
    ```

2. Build the project:

    ```sh
    cargo build
    ```

### Running the Server

To run the server, use the following command:

```sh
cargo run -- [-t | --target_dir=TARGET_DIR] [-a | --address=ADDRESS]
```

- `TARGET_DIR`: Directory to serve and save files (default: `/tmp/`).
- `ADDRESS`: Address to bind the server to (default: `127.0.0.1:4221`).

Example:

```sh
cargo run -- --target_dir=/path/to/dir --address=127.0.0.1:8080
```

### Testing

The project includes unit tests for various components. To run the tests, use the following command:

```sh
cargo test
```

This will execute all the tests and display the results.

## Usage

### Endpoints

- **GET /echo/:message**

    Echoes the message provided in the URL.

    ```sh
    curl http://127.0.0.1:4221/echo/hello
    ```

- **GET /user-agent**

    Returns the `User-Agent` header from the request.

    ```sh
    curl -H "User-Agent: MyTestAgent" http://127.0.0.1:4221/user-agent
    ```

- **GET /files/:filename**

    Serves static files from the specified directory.

    ```sh
    curl http://127.0.0.1:4221/files/test.txt
    ```

- **POST /files/:filename**

    Saves the request body as a file in the specified directory.

    ```sh
    curl -X POST -d "File content" http://127.0.0.1:4221/files/test.txt
    ```


TODO:

- [ ] handle subdir creation issue and add tests
- [ ] add more test coverage
- [ ] flesh out error handling further
- [ ] add documentation
- [ ] add graceful shutdown
- [ ] add logging
- [ ] possibly revisit module structure
- [ ] add benchmarks
- [ ] fork and refactor to use async/await
