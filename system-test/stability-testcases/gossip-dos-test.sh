#!/usr/bin/env bash

set -e
cd "$(dirname "$0")"
LUMOS_ROOT="$(cd ../..; pwd)"

logDir="$PWD"/logs
rm -rf "$logDir"
mkdir "$logDir"

lumosInstallDataDir=$PWD/releases
lumosInstallGlobalOpts=(
  --data-dir "$lumosInstallDataDir"
  --config "$lumosInstallDataDir"/config.yml
  --no-modify-path
)

# Install all the lumos versions
bootstrapInstall() {
  declare v=$1
  if [[ ! -h $lumosInstallDataDir/active_release ]]; then
    sh "$LUMOS_ROOT"/install/lumos-install-init.sh "$v" "${lumosInstallGlobalOpts[@]}"
  fi
  export PATH="$lumosInstallDataDir/active_release/bin/:$PATH"
}

bootstrapInstall "edge"
lumos-install-init --version
lumos-install-init edge
lumos-gossip --version
lumos-dos --version

killall lumos-gossip || true
lumos-gossip spy --gossip-port 8001 > "$logDir"/gossip.log 2>&1 &
lumosGossipPid=$!
echo "lumos-gossip pid: $lumosGossipPid"
sleep 5
lumos-dos --mode gossip --data-type random --data-size 1232 &
dosPid=$!
echo "lumos-dos pid: $dosPid"

pass=true

SECONDS=
while ((SECONDS < 600)); do
  if ! kill -0 $lumosGossipPid; then
    echo "lumos-gossip is no longer running after $SECONDS seconds"
    pass=false
    break
  fi
  if ! kill -0 $dosPid; then
    echo "lumos-dos is no longer running after $SECONDS seconds"
    pass=false
    break
  fi
  sleep 1
done

kill $lumosGossipPid || true
kill $dosPid || true
wait || true

$pass && echo Pass
