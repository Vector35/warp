#!/usr/bin/env bash
set -Eeuo pipefail

mkdir -p ~/.binaryninja
cp "$BN_LICENSE_FILE" ~/.binaryninja/license.dat
exec sigem "$@"