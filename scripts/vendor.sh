# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Updates all vendored crates
command bazel run //thirdparty:crates_vendor