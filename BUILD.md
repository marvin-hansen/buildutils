# Build information

Bazel is configured as primary build system for this project. See the information below for more details.

A few important differences between Cargo and Bazel:

* Bazel builds in parallel and is therefore significantly faster than Cargo especially for large projects.
* Bazel tests uses tagging thus allowing to query and run only selected tests i.e. only runs tests tagged with "foo".
  Cargo does not support this.
* Bazel caches build artifacts reliably and never rebuilds anything without code change. Cargo, however, has a long history of incorrect cache invalidation and often rebuilds the entire worskpace after only a minor change.
* Bazel supports building polyglot targets, Cargo does not.
* Bazel supports remote builds i.e. compiling on a remote cluster. Cargo does not. 

## Aliasing

This project uses target aliases for Cargo and Bazel mainly to make build targets location independent
and therefore easy to refactor. When you move a crate from one folder to another, just update the alias,
and run make build again. This works because all dependencies are declared using aliases.

For Cargo, crate aliases are defined in the workspace [Cargo.toml](Cargo.toml) file.
Please recognize that Cargo neither supports nested alias namespaces nor path dependent aliases
with the implications that, across the entire workspace, all crates must have exactly one unique names and aliases.
For simplicity, the alias is the same as the crate name.

All Bazel aliases are defined in the [BUILD.bazel](BUILD.bazel) file inside the [alias folder](alias).
For simplicity, Bazel aliases are also sticking to the convention that the alias is the same as the crate name to
replicate the Cargo convention. However, Bazel supports nested alias namespaces and path dependent target aliases
and everything in between. Please refer to the official documentation for more information.

## Build commands

Cargo build works as expected. However, Bazel is configured as primary build system for this project.
Because not everyone is familiar with Bazel, I made a makefile to simplify all build related tasks.

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



## Selective target build

To build only a single target, run `make current` and specify the target in the current.txt file.
The command in the underlying shell script is the Bazel equivalent to Cargo build -p target.

## Vendoring

To vendor all Bazel managed dependencies, run `make vendor`. Bazel then resolves all dependencies,
stores them in thirdparty, and makes them available to the project under the //thirdparty/crates namespace.
If your build target depends, for example, on rustls and Tokio, then you add the following to your target dependency list:

```text
    deps = [
        # External crates
        "//thirdparty/crates:rustls",
        "//thirdparty/crates:tokio",
        #...  
    ],
```    

Notice, only vendored dependencies are available to build targets. When you add new Rust dependencies
to `thirdparty/BUILD.bazel`, you have to call `make vendor `first before you can use them.

No vendoring has been configured for Cargo although this should be possible. Please refer
to the official Cargo documentation for more information.

