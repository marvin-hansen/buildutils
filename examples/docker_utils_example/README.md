# Docker Utils Example 🐳

This example demonstrates how to use the `docker_utils` crate to manage Docker containers in your Rust applications.

## Features

- Container lifecycle management (start, stop, check status)
- Postgres container configuration and setup
- Error handling and status verification
- Container port mapping

## Example Code

```rust
use docker_utils::*;

fn main() -> Result<(), Error> {
    // Initialize Docker utilities
    let docker_util = DockerUtil::new()?;
    
    // Configure and start a Postgres container
    let container_config = postgres_db_container_config();
    let (container_id, port) = docker_util.get_or_start_container(&container_config)?;
    
    // Verify container is running
    assert!(docker_util.check_if_container_is_running(&container_id)?);
    
    // Stop and clean up
    docker_util.stop_container(&container_id)?;
    Ok(())
}
```

## Running the Example

1. Make sure Docker is installed and running on your system
2. Run the example:
   ```bash
   cargo run
   ```

The example will:
1. Start a Postgres container
2. Verify it's running
3. Stop the container
4. Verify it's been cleaned up

## Dependencies

- docker_utils = "0.1.0"
- Postgres Docker image
