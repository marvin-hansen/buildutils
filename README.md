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

## Build commands

Cargo build commands for all crates work as expected. However, Bazel is configured 
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