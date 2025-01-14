# bin/sh
set -o errexit
set -o nounset
set -o pipefail


# Bazel file formatting (Installed via homebrew)
# https://github.com/bazelbuild/buildtools
buildifier -r MODULE.bazel BUILD.bazel thirdparty/BUILD.bazel
buildifier -r build wait_utils/

# Rust code formatting
# https://github.com/rust-lang/rustfmt
command cargo fmt --all