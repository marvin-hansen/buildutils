# Service Utils 🛠️

A Rust utility crate for testing service binaries with ease.

## Features ✨

- **Wait Strategies**: Flexible waiting mechanisms to ensure services are ready
- **Debug Mode**: Enhanced logging and debugging capabilities
- **Environment Control**: Configure service environment variables
- **Error Handling**: Comprehensive error handling for service operations

## Usage 🚀

Add this to your `Cargo.toml`:

```toml
[dependencies]
service_utils = "0.1.0"
```

### Basic Example

```rust
use service_utils::{ServiceUtil, WaitStrategy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ServiceUtil with the path to your binaries
    let service_util = ServiceUtil::new(
        "/path/to/binaries",
        vec!["service1", "service2"]
    ).await?;

    // Start a service with a wait strategy
    service_util.start_service(
        "service1",
        WaitStrategy::WaitForSeconds(5),
        None,
    ).await?;

    Ok(())
}
```

### With Configuration

```rust
use service_utils::{ServiceUtil, ServiceStartConfig, WaitStrategy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_util = ServiceUtil::new(
        "/path/to/binaries",
        vec!["my_service"]
    ).await?;

    let config = ServiceStartConfig {
        program: "my_service",
        wait_strategy: WaitStrategy::WaitUntilConsoleOutputContains(
            "Service started successfully".to_string(),
            15,
        ),
        env_var: Some(("DEBUG", "1")),
    };

    service_util.start_service_from_config(config).await?;

    Ok(())
}
```

## Wait Strategies 🕒

The crate provides several wait strategies through the `wait_utils` dependency:

- `WaitForSeconds(u64)`: Wait for a specified number of seconds
- `WaitUntilConsoleOutputContains(String, u64)`: Wait until specific output appears
- `WaitUntilPortIsAvailable(u16, u64)`: Wait for a port to become available
- Custom strategies can be implemented as needed

## Error Handling 🚨

The crate uses a dedicated `ServiceUtilError` type that covers various failure scenarios:

- Binary not found
- Service start failure
- Wait strategy timeout
- Environment configuration errors

## Debug Mode 🔍

Enable debug mode for additional logging and information:

```rust
let service_util = ServiceUtil::with_debug(
    "/path/to/binaries",
    vec!["service1"]
).await?;
```

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request.

## License 📄

This project is licensed under the terms specified in the workspace configuration.

## Related Crates 📦

- `wait_utils`: Provides wait strategies used by this crate
- `docker_utils`: Docker container management utilities
