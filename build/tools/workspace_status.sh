#!/usr/bin/env bash
#
# SPDX-License-Identifier: MIT
# Copyright (c) "2025" . The buildutils Authors and Contributors. All Rights Reserved.
#

set -o errexit
set -o nounset
set -o pipefail

echo "STABLE_GIT_COMMIT $(git rev-parse --short HEAD)"
