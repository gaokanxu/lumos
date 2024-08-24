#!/usr/bin/env bash

here=$(dirname "$0")
# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

set -e

rm -rf "$LUMOS_CONFIG_DIR"/latest-mainnet-beta-snapshot
mkdir -p "$LUMOS_CONFIG_DIR"/latest-mainnet-beta-snapshot
(
  cd "$LUMOS_CONFIG_DIR"/latest-mainnet-beta-snapshot || exit 1
  set -x
  wget http://api.mainnet-beta.lumos.com/genesis.tar.bz2
  wget --trust-server-names http://api.mainnet-beta.lumos.com/snapshot.tar.bz2
)

snapshot=$(ls "$LUMOS_CONFIG_DIR"/latest-mainnet-beta-snapshot/snapshot-[0-9]*-*.tar.zst)
if [[ -z $snapshot ]]; then
  echo Error: Unable to find latest snapshot
  exit 1
fi

if [[ ! $snapshot =~ snapshot-([0-9]*)-.*.tar.zst ]]; then
  echo Error: Unable to determine snapshot slot for "$snapshot"
  exit 1
fi

snapshot_slot="${BASH_REMATCH[1]}"

rm -rf "$LUMOS_CONFIG_DIR"/bootstrap-validator
mkdir -p "$LUMOS_CONFIG_DIR"/bootstrap-validator


# Create genesis ledger
if [[ -r $FAUCET_KEYPAIR ]]; then
  cp -f "$FAUCET_KEYPAIR" "$LUMOS_CONFIG_DIR"/faucet.json
else
  $lumos_keygen new --no-passphrase -fso "$LUMOS_CONFIG_DIR"/faucet.json
fi

if [[ -f $BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR ]]; then
  cp -f "$BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR" "$LUMOS_CONFIG_DIR"/bootstrap-validator/identity.json
else
  $lumos_keygen new --no-passphrase -so "$LUMOS_CONFIG_DIR"/bootstrap-validator/identity.json
fi

$lumos_keygen new --no-passphrase -so "$LUMOS_CONFIG_DIR"/bootstrap-validator/vote-account.json
$lumos_keygen new --no-passphrase -so "$LUMOS_CONFIG_DIR"/bootstrap-validator/stake-account.json

$lumos_ledger_tool create-snapshot \
  --ledger "$LUMOS_CONFIG_DIR"/latest-mainnet-beta-snapshot \
  --faucet-pubkey "$LUMOS_CONFIG_DIR"/faucet.json \
  --faucet-lamports 500000000000000000 \
  --bootstrap-validator "$LUMOS_CONFIG_DIR"/bootstrap-validator/identity.json \
                        "$LUMOS_CONFIG_DIR"/bootstrap-validator/vote-account.json \
                        "$LUMOS_CONFIG_DIR"/bootstrap-validator/stake-account.json \
  --hashes-per-tick sleep \
  "$snapshot_slot" "$LUMOS_CONFIG_DIR"/bootstrap-validator

$lumos_ledger_tool modify-genesis \
  --ledger "$LUMOS_CONFIG_DIR"/latest-mainnet-beta-snapshot \
  --hashes-per-tick sleep \
  "$LUMOS_CONFIG_DIR"/bootstrap-validator
