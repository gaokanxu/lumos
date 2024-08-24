#!/usr/bin/env bash
set -ex

[[ $(uname) = Linux ]] || exit 1
[[ $USER = root ]] || exit 1

[[ -d /home/lumos/.ssh ]] || exit 1

if [[ ${#LUMOS_PUBKEYS[@]} -eq 0 ]]; then
  echo "Warning: source lumos-user-authorized_keys.sh first"
fi

# lumos-user-authorized_keys.sh defines the public keys for users that should
# automatically be granted access to ALL testnets
for key in "${LUMOS_PUBKEYS[@]}"; do
  echo "$key" >> /lumos-scratch/authorized_keys
done

sudo -u lumos bash -c "
  cat /lumos-scratch/authorized_keys >> /home/lumos/.ssh/authorized_keys
"
