# Build Utils

Utils for CI tests with Bazel. Mostly used on BuildBuddy, but can easily be used locally or on any 
other Bazel CI system. 

## Crates

### 🐳 Docker Utils

A friendly Rust crate that makes Docker container management a breeze! With docker_utils, you can:
- Start and manage Docker containers with just a few lines of code
- Configure containers flexibly with custom ports and environment variables
- Handle container lifecycle with ease (start, stop, check status)

Check out the [docker_utils documentation](crates/docker_utils/README.md) for detailed usage examples and API reference.

### 🔧 Service Utils

A lightweight Rust crate for testing service binaries efficiently. With service_utils, you can:
- Start service binaries for easyintegration testing
- Use flexible wait strategies to ensure services are ready
- Configure environment variables for testing different scenarios

Check out the [service_utils documentation](crates/service_utils/README.md) for detailed usage examples and API reference.

## Examples 📚

The repository includes several examples to help you get started:

### 🐳 Docker Utils Example
Demonstrates container lifecycle management using docker_utils:
- Start and manage a Postgres container
- Check container status and health
- Handle container cleanup
[View Example](examples/docker_utils_example)

### 🌐 Service Example
A sample HTTP service implementation that is used to test the service utils crate:
- Health check endpoint
- Greeting endpoint
- Warp web framework integration
[View Example](examples/service_example)

### 🔧 Service Utils Example
Shows how to test service binaries using service_utils:
- Service binary management
- Wait strategy implementation
- Integration test setup
[View Example](examples/service_utils_example)

## Users

* Docker and Service Utils power thousands of CI test runs each month at Emet-labs.


## Build commands

Cargo build work as expected for all crates. However, Bazel is configured 
as primary build system for this project. Because not everyone is familiar with Bazel, 
I made a makefile to simplify all bazel and build related tasks.

```text
    make build          Builds the code base incrementally (fast) for dev.
    make current        Builds the current target incrementally (fast) defined in current.txt.
    make doc            Builds documentation for the project.
    make format         Formats call code according to cargo fmt style.
    make lint           Lints and formats the code of the project.
    make fix            Fixes linting issues as reported by clippy.
    make test           Tests across all crates.
    make vendor         Vendors all Bazel managed Rust dependencies to folder thirdparty.
```

For more details on the project build configuration, please read the [BUILD.md file](BUILD.md).

## Licence
This project is licensed under the [MIT license](LICENSE).

## Author
* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/
* Github GPG key ID: 369D5A0B210D39BC