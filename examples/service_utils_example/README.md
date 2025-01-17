# Service Utils Example 🔧

This example demonstrates how to use the `service_utils` crate to test service binaries in your integration tests.

## Features

- Service binary management
- Wait strategy implementation
- Environment variable configuration
- Integration test setup

## Example Structure

```
service_utils_example/
├── src/           # Service implementation
└── tests/         # Integration tests using service_utils
```

## Running the Example

1. Build the service:
   ```bash
   cargo build
   ```

2. Run the tests:
   ```bash
   cargo test
   ```

The example demonstrates:
- Starting a service binary
- Waiting for the service to be ready
- Testing service endpoints
- Proper cleanup after tests

## Key Concepts

1. **Service Configuration**
   ```rust
   let config = ServiceStartConfig::new(
       "my_service",
       WaitStrategy::WaitUntilConsoleOutputContains(
           "Service started successfully".to_string(),
           15,
       ),
       Some(vec!["DEBUG=1".to_string()]),
   );
   ```

2. **Wait Strategies**
   - Console output matching
   - Port availability checking
   - Custom timeout configuration

3. **Environment Control**
   - Set test-specific environment variables
   - Control service behavior in tests

## Dependencies

- service_utils = "0.1.0"
- wait_utils = "0.1.0"
