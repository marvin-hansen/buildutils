# 🐳 Docker Utils

A friendly Rust crate that simplifies Docker container management with a clean and intuitive API.

## Features

- 🐳 **Easy Docker API**
  - Only uses the standard library thus easy to cross compile. 
  - Only uses the robust Docker CLI.
  - Tested and documented [public API](src/api.rs).

- 🚀 **Container Lifecycle Management**
  - Start and stop containers with ease.
  - Check container status.
  - Pull container images.
  - Prune unused containers.

- ⚙️ **Flexible Configuration**
  - Environment variable support.
  - Platform-specific configurations.
  - Container reuse options.
  - Easy builder pattern support.


## Quick Start

```rust
use docker_utils::*;

// Create a new Docker utility instance
let docker_util = DockerUtil::new().expect("Failed to create DockerUtil");

// Configure your container using the provided sample configuration for postgres
let container_config = postgres_config::postgres_db_container_config();

// Start the container or get it if it already running. Returns (container_name, port)
let (container_name, port) = docker_util
    .setup_container(&container_config)
    .expect("Failed to start container");

// Stop the container when done
docker_util.stop_container(&container_name)
    .expect("Failed to stop container");
```

## Container Configuration

Creating a new configuration using the builder pattern:

```rust
use docker_utils::*;
 
    let config = ContainerConfig::builder()
        .name("test_container")
        .image("test_image")
        .tag("latest")
        .url("0.0.0.0")
        .connection_port(8080)
        .reuse_container(true)
        .keep_configuration(true)
        .wait_strategy(WaitStrategy::NoWait)
        .build();
```  

Setting optional fields (opening additional ports, environment variables, or platform) using the builder:

```rust
use docker_utils::*;
 
    let config = ContainerConfig::builder()
        .name("test_container")
        .image("test_image")
        .tag("latest")
        .url("0.0.0.0")
        .connection_port(8080)
        .additional_ports(&[8081, 8082])
        .additional_env_vars( &["ENV_VAR=VALUE", "DEBUG=true"])
        .platform("linux/amd64")
        .reuse_container(true)
        .keep_configuration(true)
        .wait_strategy(WaitStrategy::NoWait)
        .build();
```  

Creating a new configuration using the conventional constructor:

```rust
use docker_utils::*;
 
    let config =  ContainerConfig::new(
        "test_container",
        "test_image",
        "latest",
        "0.0.0.0",
        8080,
        Some(&[8081, 8082]),
        Some(&["ENV_VAR=VALUE", "DEBUG=true"]),
        Some("linux/amd64"),
        true,
        false,
        WaitStrategy::default(), // NoWait is the default wait strategy
    );
```  

## Error Handling

The crate uses a custom `DockerError` type for comprehensive error handling, making it easy to identify 
and handle Docker-related issues in your application.

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
