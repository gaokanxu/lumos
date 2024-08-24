#!/usr/bin/env bash
#
# Starts an instance of lumos-faucet
#
here=$(dirname "$0")

# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

[[ -f "$LUMOS_CONFIG_DIR"/faucet.json ]] || {
  echo "$LUMOS_CONFIG_DIR/faucet.json not found, create it by running:"
  echo
  echo "  ${here}/setup.sh"
  exit 1
}

set -x
# shellcheck disable=SC2086 # Don't want to double quote $lumos_faucet
exec $lumos_faucet --keypair "$LUMOS_CONFIG_DIR"/faucet.json "$@"
