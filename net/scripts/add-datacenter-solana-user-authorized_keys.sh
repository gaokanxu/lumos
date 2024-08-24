#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"

# shellcheck source=net/scripts/lumos-user-authorized_keys.sh
source lumos-user-authorized_keys.sh

# lumos-user-authorized_keys.sh defines the public keys for users that should
# automatically be granted access to ALL datacenter nodes.
for i in "${!LUMOS_USERS[@]}"; do
  echo "environment=\"LUMOS_USER=${LUMOS_USERS[i]}\" ${LUMOS_PUBKEYS[i]}"
done

