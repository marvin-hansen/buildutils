#
# SPDX-License-Identifier: MIT
# Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Runs the whole workspace under Cargo, with the binary staging Bazel does for free.
#
# service_utils starts real binaries, so its example test needs one on disk before it runs.
# Bazel stages it declaratively: the test target has a `copy_file` in its `data`, and Bazel
# materialises it into the runfiles tree before the test starts. Cargo has no equivalent.
# A build script cannot do it either, because build scripts run before the workspace's other
# binaries are built and have no dependency edge to them.
#
# So it is done here, ahead of the test run. This is a plain Cargo limitation, not a defect
# in the crate or the example: under Bazel, `make test` needs none of this.

# Must match BAZEL_ROOT_PATH / CARGO_ROOT_PATH and PROGRAM in
# examples/service_utils_example/tests/service_utils_tests.rs
SERVICE_BIN_DIR="examples/service_utils_example/tests"
SERVICE_BIN_NAME="service"

command cargo build -p service_example
command mkdir -p "${SERVICE_BIN_DIR}"
command cp "target/debug/service_example" "${SERVICE_BIN_DIR}/${SERVICE_BIN_NAME}"
command chmod +x "${SERVICE_BIN_DIR}/${SERVICE_BIN_NAME}"

echo "Staged ${SERVICE_BIN_DIR}/${SERVICE_BIN_NAME}"

echo ""
echo "=============="
echo "Run tests"
echo "=============="

# One test binary at a time. The Docker integration tests share a single daemon, and cargo
# runs different packages' test binaries concurrently, so without this they collide on
# container names and published ports. Bazel does not need it: it isolates each test target.
command cargo test --workspace -- --test-threads=1

echo ""
echo "====================="
echo "All Tests Passed"
echo "====================="
echo ""
