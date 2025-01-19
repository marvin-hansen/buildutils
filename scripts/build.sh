#
# SPDX-License-Identifier: MIT
# Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail


command bazel build //... --test_env=ENV=LOCAL