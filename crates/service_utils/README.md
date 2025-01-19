[//]: # (---)

[//]: # (SPDX-License-Identifier: MIT)

[//]: # (---)

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/badge/Crates.io-Latest-blue

[crates-url]: https://crates.io/crates/service_utils

[docs-badge]: https://img.shields.io/badge/Docs.rs-Latest-blue

[docs-url]: https://docs.rs/service_utils/latest/service_utils/

[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg

[mit-url]: https://github.com/deepcausality-rs/deep_causality/blob/main/LICENSE


# Service Utils 🛠️

A simple utility for testing service binaries with ease and flexibility mainly for usage with BAZEL during CI BUILDS.

## Why?

Integration and acceptance testing in bulk quite often requires dockerized applications but as
the number of tests increase, so does the overhead from setting up and tearing down docker containers.
With Service Utils, you can easily test your services without of docker and therefore reducing test time substantially.

## Why Bazel?

While cargo is great, it reaches its limitations when a project grows large. Specifically when you need to test
a large number of services, you need to group services, start processes, set environment variables, and ensure
everything has been cleaned up after tests. While this can be done to some degree with Cargo, Bazel offers a
comprehensive solution that can be used to test a large number of services in a fast and efficient way without the need
for complex configuration.

Specifically, the service_util does not have a service stop or shutdown method because Bazel runs all tests in an
isolated sandbox environment that shutdowns and cleans up automatically. In contrast, if you were to start a service
with Cargo test, you would have to kill that process manually to prevent zombie processes on the CI server. The service
util does not offer a way to copy binaries back and forth because Bazel already has a build in mechanism to do that. The
service util is stateless as to facilitate massively parallel integration testing with Bazel Remote Build Execution (RBE) .

That said, the util can be used with Cargo,
the [service util example demonstrates](../../examples/service_utils_example)
this and documents the additional steps required.

## Features ✨

- **Wait Strategies**: Flexible waiting mechanisms to ensure services are ready
- **Environment Control**: Configure service environment variables
- **Error Handling**: Comprehensive error handling for service operations

## Install 🚀

Add this to your `Cargo.toml`:

```toml
[dependencies]
service_utils = "0.1.0"
```

### Basic Example

```rust
use service_utils::*;
 
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

## Service Start Configuration

Creating a new start configuration using the builder pattern:

```rust
use service_utils::*;

    let config = ServiceStartConfig::builder()
        .program("service1")
        .wait_strategy(WaitStrategy::WaitForSeconds(5))
        .env_vars(vec!["DEBUG=1".into(), "ENV=CI".into()])
        .build();

    service_util.start_service_from_config(config).await?;

    Ok(())
}
```

## Bazel Test configuration

A Rust test configuration for testing with the service util and Bazel requires three segments:
1) Imports i.e. the Bazel rules to load 
2) Test suite
3) Copy the binary to the target directory

For example, the configuration below is taken from the [service_utils_example](../../examples/service_utils_example):

```python
# 1) Imports   
load("@aspect_bazel_lib//lib:copy_file.bzl", "copy_file")
load("@rules_rust//rust:defs.bzl", "rust_test_suite")

# 2) Test suite 
rust_test_suite(
    name = "tests",
    srcs = glob([
        "*_tests.rs",
    ]),
    data = [
        ":copy_service",  # Copies the service binary into the test folder
    ],
    tags = [ # Tags are used to filter and select tests to run 
        "integration-test",
        "service_utils_example",
    ],
    visibility = ["//visibility:public"],
    deps = [
        # Crate under test
        "//alias:service_example",
        # Internal crates
        "//alias:service_utils",
        # External crates
        "//thirdparty/crates:reqwest",
        "//thirdparty/crates:serde",
        "//thirdparty/crates:tokio",
    ],
)

# 3) Copy the binary   
copy_file(
    name = "copy_service",  # label to this rule. Used in the data attribute
    src = "//alias:service_example",  # Alias is defined in file: alias/BUILD.bazel
    out = "service",  # Name of the output binary
    is_executable = True,  # Must always set to true otherwise the service cannot be started.
)
```  

## Wait Strategies 🕒

The crate provides several wait strategies through the `wait_utils` dependency:

- `WaitForDuration(u64)`: Wait for a specified number of seconds
- `WaitForHttpHealthCheck(String, u64)`: Wait until an HTTP request to the given URL or until a timeout occurs.
- `WaitForGrpcHealthCheck(u16, u64)`: Wait until an gRPC health request to the given URL or until a timeout occurs.

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

## Related Crates 📦

- `wait_utils`: Provides wait strategies used by this crate
- `docker_utils`: Docker container management utilities

## Contributing

Contributions are welcome! Feel free to:

- Report issues
- Submit pull requests
- Suggest new features
- Improve documentation

## Licence

This project is licensed under the [MIT license](LICENSE).

## Author

* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/
* Github GPG key ID: 369D5A0B210D39BC
