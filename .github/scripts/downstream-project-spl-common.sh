#!/usr/bin/env bash
set -e

here="$(dirname "${BASH_SOURCE[0]}")"

#shellcheck source=ci/downstream-projects/common.sh
source "$here"/../../ci/downstream-projects/common.sh

set -x
rm -rf spl
git clone https://github.com/lumos-labs/lumos-program-library.git spl

# copy toolchain file to use lumos's rust version
cp "$LUMOS_DIR"/rust-toolchain.toml spl/
cd spl || exit 1

project_used_lumos_version=$(sed -nE 's/lumos-sdk = \"[>=<~]*(.*)\"/\1/p' <"token/program/Cargo.toml")
echo "used lumos version: $project_used_lumos_version"
if semverGT "$project_used_lumos_version" "$LUMOS_VER"; then
  echo "skip"
  return
fi

./patch.crates-io.sh "$LUMOS_DIR"
