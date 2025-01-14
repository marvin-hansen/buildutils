# Build Utils

Utils for CI tests with Bazel. Mostly used on BuildBuddy, but can easily be used locally or on any 
other Bazel CI system. 

## Build commands

Cargo build works as expected. However, Bazel is configured as primary build system for this project.
Because not everyone is familiar with Bazel, I made a makefile to simplify all bazel and build related tasks.

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