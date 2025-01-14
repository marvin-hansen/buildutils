# 🐳 Docker Utils

A friendly Rust crate that simplifies Docker container management with a clean and intuitive API.

## Features

- 🚀 **Container Lifecycle Management**
  - Start and stop containers with ease
  - Check container status
  - Pull container images
  - Prune unused containers

- ⚙️ **Flexible Configuration**
  - Custom port mapping
  - Environment variable support
  - Platform-specific configurations
  - Container reuse options

- 🔍 **Container Monitoring**
  - Check running containers
  - Verify container tags

## Quick Start

```rust
use docker_utils::DockerUtil;
use common_container::ContainerConfig;

// Create a new Docker utility instance
let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

// Configure your container using the provided sample configuration for postgres
let container_config = postgres_config::postgres_db_container_config();

// Start the container or get it if it already running. Returns (container_name, port)
let (container_name, port) = docker_util
    .get_or_start_container_config(&container_config)
    .expect("Failed to start container");

// Stop the container when done
docker_util.stop_container(&container_name)
    .expect("Failed to stop container");
```

## Error Handling

The crate uses a custom `DockerError` type for comprehensive error handling, making it easy to identify and handle Docker-related issues in your application.

## Examples

Check out the `example` directory for complete working examples, including:
- Postgres container setup and management
- Container lifecycle management
- Status monitoring

## Requirements

- Docker daemon running on your system
- Rust 2021 edition or later

## Contributing

Contributions are welcome! Feel free to:
- Report issues
- Submit pull requests
- Suggest new features
- Improve documentation

## Licence
This project is licensed under the [MIT license](../../LICENSE).

## Author
* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/
* Github GPG key ID: 369D5A0B210D39BC
