#!/usr/bin/env bash
set -e

source ci/_
source ci/semver_bash/semver.sh
source scripts/patch-crates.sh
source scripts/read-cargo-variable.sh

LUMOS_VER=$(readCargoVariable version Cargo.toml)
export LUMOS_VER
export LUMOS_DIR=$PWD
export CARGO="$LUMOS_DIR"/cargo
export CARGO_BUILD_SBF="$LUMOS_DIR"/cargo-build-sbf
export CARGO_TEST_SBF="$LUMOS_DIR"/cargo-test-sbf

mkdir -p target/downstream-projects
cd target/downstream-projects
