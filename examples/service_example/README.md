# Service Example 🌐

A sample HTTP service implementation that demonstrates a typical service that might be tested using the `service_utils` crate.

## Features

- HTTP endpoints:
  - Health check endpoint (`/health`)
  - Greeting endpoint (`/hello`)
- Warp web framework integration
- Simple service configuration


## Running the Example

1. Start the service:
   ```bash
   cargo run
   ```
2. Test the endpoints:
   ```bash
   curl http://localhost:4242/health
   curl http://localhost:4242/hello
   ```

## Dependencies

- warp = "0.3"
- tokio = { version = "1", features = ["full"] }
