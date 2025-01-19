#
# SPDX-License-Identifier: MIT
# Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
#

# bin/sh
set -o errexit
set -o nounset
set -o pipefail

command bazel build //... --build_tag_filters=doc,doc-test --test_env=ENV=LOCAL
command bazel test  //... --test_tag_filters=doc-test --test_env=ENV=LOCAL