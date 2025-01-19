# Docker Utils Example 🐳

This example demonstrates how to use the `docker_utils` crate to manage Docker containers in your Rust integration tests.

## Features

- Container lifecycle management (start, stop, check status)
- Postgres container configuration and setup
- Error handling and status verification
- Container port mapping


## Running the Example


Make sure Docker is installed and running on your system 
 
Run the example with Cargo:
```bash
cargo test -p docker_utils_example
```

Run the example with Bazel:
```bash
bazel  test //... --test_tag_filters=docker_utils_example
```

The example will:
1. Start a Postgres container
2. Verify it's running
3. Stop the container
4. Verify the container has been stopped and deleted

## Dependencies

- docker_utils = "0.1.0"
