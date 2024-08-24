#!/usr/bin/env bash
set -e

cd "$(dirname "$0")"/../..

set -x
deployMethod="$1"
nodeType="$2"
entrypointIp="$3"
numNodes="$4"
if [[ -n $5 ]]; then
  export RUST_LOG="$5"
fi
skipSetup="$6"
failOnValidatorBootupFailure="$7"
externalPrimordialAccountsFile="$8"
maybeDisableAirdrops="$9"
internalNodesStakeLamports="${10}"
internalNodesLamports="${11}"
nodeIndex="${12}"
numBenchTpsClients="${13}"
benchTpsExtraArgs="${14}"
genesisOptions="${15}"
extraNodeArgs="${16}"
gpuMode="${17:-auto}"
maybeWarpSlot="${18}"
maybeFullRpc="${19}"
waitForNodeInit="${20}"
extraPrimordialStakes="${21:=0}"
tmpfsAccounts="${22:false}"
disableQuic="${23}"
enableUdp="${24}"

set +x

missing() {
  echo "Error: $1 not specified"
  exit 1
}

[[ -n $deployMethod ]]  || missing deployMethod
[[ -n $nodeType ]]      || missing nodeType
[[ -n $entrypointIp ]]  || missing entrypointIp
[[ -n $numNodes ]]      || missing numNodes
[[ -n $skipSetup ]]     || missing skipSetup
[[ -n $failOnValidatorBootupFailure ]] || missing failOnValidatorBootupFailure

airdropsEnabled=true
if [[ -n $maybeDisableAirdrops ]]; then
  airdropsEnabled=false
fi
cat > deployConfig <<EOF
deployMethod="$deployMethod"
entrypointIp="$entrypointIp"
numNodes="$numNodes"
failOnValidatorBootupFailure=$failOnValidatorBootupFailure
genesisOptions="$genesisOptions"
airdropsEnabled=$airdropsEnabled
EOF

source net/common.sh
source multinode-demo/common.sh
loadConfigFile

initCompleteFile=init-complete-node.log

cat > ~/lumos/on-reboot <<EOF
#!/usr/bin/env bash
cd ~/lumos
source scripts/oom-score-adj.sh

now=\$(date -u +"%Y-%m-%dT%H:%M:%SZ")
ln -sfT validator.log.\$now validator.log
EOF
chmod +x ~/lumos/on-reboot

GPU_CUDA_OK=false
GPU_FAIL_IF_NONE=false
case "$gpuMode" in
  on) # GPU *required*, any vendor
    GPU_CUDA_OK=true
    GPU_FAIL_IF_NONE=true
    ;;
  off) # CPU-only
    ;;
  auto) # Use GPU if installed, any vendor
    GPU_CUDA_OK=true
    ;;
  cuda) # GPU *required*, CUDA-only
    GPU_CUDA_OK=true
    GPU_FAIL_IF_NONE=true
    ;;
  *)
    echo "Unexpected gpuMode: \"$gpuMode\""
    exit 1
    ;;
esac

case $deployMethod in
local|tar|skip)
  PATH="$HOME"/.cargo/bin:"$PATH"
  export USE_INSTALL=1

  ./fetch-perf-libs.sh

cat >> ~/lumos/on-reboot <<EOF
  PATH="$HOME"/.cargo/bin:"$PATH"
  export USE_INSTALL=1

  (
    sudo LUMOS_METRICS_CONFIG="$LUMOS_METRICS_CONFIG" scripts/oom-monitor.sh
  ) > oom-monitor.log 2>&1 &
  echo \$! > oom-monitor.pid
  scripts/fd-monitor.sh > fd-monitor.log 2>&1 &
  echo \$! > fd-monitor.pid
  scripts/net-stats.sh  > net-stats.log 2>&1 &
  echo \$! > net-stats.pid
  scripts/iftop.sh  > iftop.log 2>&1 &
  echo \$! > iftop.pid
  scripts/system-stats.sh  > system-stats.log 2>&1 &
  echo \$! > system-stats.pid

  if ${GPU_CUDA_OK} && [[ -e /dev/nvidia0 ]]; then
    echo Selecting lumos-validator-cuda
    export LUMOS_CUDA=1
  elif ${GPU_FAIL_IF_NONE} ; then
    echo "Expected GPU, found none!"
    export LUMOS_GPU_MISSING=1
  fi
EOF

  case $nodeType in
  bootstrap-validator)
    set -x
    if [[ $skipSetup != true ]]; then
      clear_config_dir "$LUMOS_CONFIG_DIR"

      if [[ -n $internalNodesLamports ]]; then
        echo "---" >> config/validator-balances.yml
      fi

      setupValidatorKeypair() {
        declare name=$1
        if [[ -f net/keypairs/"$name".json ]]; then
          cp net/keypairs/"$name".json config/"$name".json
          if [[ "$name" =~ ^validator-identity- ]]; then
            name="${name//-identity-/-vote-}"
            cp net/keypairs/"$name".json config/"$name".json
            name="${name//-vote-/-stake-}"
            cp net/keypairs/"$name".json config/"$name".json
          fi
        else
          lumos-keygen new --no-passphrase -so config/"$name".json
          if [[ "$name" =~ ^validator-identity- ]]; then
            name="${name//-identity-/-vote-}"
            lumos-keygen new --no-passphrase -so config/"$name".json
            name="${name//-vote-/-stake-}"
            lumos-keygen new --no-passphrase -so config/"$name".json
          fi
        fi
        if [[ -n $internalNodesLamports ]]; then
          declare pubkey
          pubkey="$(lumos-keygen pubkey config/"$name".json)"
          cat >> config/validator-balances.yml <<EOF
$pubkey:
  balance: $internalNodesLamports
  owner: 11111111111111111111111111111111
  data:
  executable: false
EOF
        fi
      }
      for i in $(seq 1 "$numNodes"); do
        setupValidatorKeypair validator-identity-"$i"
      done
      setupValidatorKeypair blockstreamer-identity

      lamports_per_signature="42"
      # shellcheck disable=SC2206 # Do not want to quote $genesisOptions
      genesis_args=($genesisOptions)
      for i in "${!genesis_args[@]}"; do
        if [[ "${genesis_args[$i]}" = --target-lamports-per-signature ]]; then
          lamports_per_signature="${genesis_args[$((i+1))]}"
          break
        fi
      done

      for i in $(seq 0 $((numBenchTpsClients-1))); do
        # shellcheck disable=SC2086 # Do not want to quote $benchTpsExtraArgs
        lumos-bench-tps --write-client-keys config/bench-tps"$i".yml \
          --target-lamports-per-signature "$lamports_per_signature" $benchTpsExtraArgs
        # Skip first line, as it contains header
        tail -n +2 -q config/bench-tps"$i".yml >> config/client-accounts.yml
        echo "" >> config/client-accounts.yml
      done
      if [[ -f $externalPrimordialAccountsFile ]]; then
        cat "$externalPrimordialAccountsFile" >> config/validator-balances.yml
      fi
      if [[ -f config/validator-balances.yml ]]; then
        genesisOptions+=" --primordial-accounts-file config/validator-balances.yml"
      fi
      if [[ -f config/client-accounts.yml ]]; then
        genesisOptions+=" --primordial-accounts-file config/client-accounts.yml"
      fi

      if [[ -n $internalNodesStakeLamports ]]; then
        args+=(--bootstrap-validator-stake-lamports "$internalNodesStakeLamports")
      fi
      if [[ -n $internalNodesLamports ]]; then
        args+=(--bootstrap-validator-lamports "$internalNodesLamports")
      fi
      # shellcheck disable=SC2206 # Do not want to quote $genesisOptions
      args+=($genesisOptions)

      if [[ -f net/keypairs/faucet.json ]]; then
        export FAUCET_KEYPAIR=net/keypairs/faucet.json
      fi
      if [[ -f net/keypairs/bootstrap-validator-identity.json ]]; then
        export BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR=net/keypairs/bootstrap-validator-identity.json
      fi
      if [[ -f net/keypairs/bootstrap-validator-stake.json ]]; then
        export BOOTSTRAP_VALIDATOR_STAKE_KEYPAIR=net/keypairs/bootstrap-validator-stake.json
      fi
      if [[ -f net/keypairs/bootstrap-validator-vote.json ]]; then
        export BOOTSTRAP_VALIDATOR_VOTE_KEYPAIR=net/keypairs/bootstrap-validator-vote.json
      fi
      echo "remote-node.sh: Primordial stakes: $extraPrimordialStakes"
      if [[ "$extraPrimordialStakes" -gt 0 ]]; then
        if [[ "$extraPrimordialStakes" -gt "$numNodes" ]]; then
          echo "warning: extraPrimordialStakes($extraPrimordialStakes) clamped to numNodes($numNodes)"
          extraPrimordialStakes=$numNodes
        fi
        for i in $(seq "$extraPrimordialStakes"); do
          args+=(--bootstrap-validator "$(lumos-keygen pubkey "config/validator-identity-$i.json")"
                                       "$(lumos-keygen pubkey "config/validator-vote-$i.json")"
                                       "$(lumos-keygen pubkey "config/validator-stake-$i.json")"
          )
        done
      fi

      multinode-demo/setup.sh "${args[@]}"

      maybeWaitForSupermajority=
      # shellcheck disable=SC2086 # Do not want to quote $extraNodeArgs
      set -- $extraNodeArgs
      while [[ -n $1 ]]; do
        if [[ $1 = "--wait-for-supermajority" ]]; then
          maybeWaitForSupermajority=$2
          break
        fi
        shift
      done

      if [[ -z "$maybeWarpSlot" && -n "$maybeWaitForSupermajority" ]]; then
        maybeWarpSlot="--warp-slot $maybeWaitForSupermajority"
      fi

      if [[ -n "$maybeWarpSlot" ]]; then
        # shellcheck disable=SC2086 # Do not want to quote $maybeWarSlot
        lumos-ledger-tool -l config/bootstrap-validator create-snapshot 0 config/bootstrap-validator $maybeWarpSlot
      fi

      lumos-ledger-tool -l config/bootstrap-validator shred-version --max-genesis-archive-unpacked-size 1073741824 | tee config/shred-version

      if [[ -n "$maybeWaitForSupermajority" ]]; then
        bankHash=$(lumos-ledger-tool -l config/bootstrap-validator bank-hash --halt-at-slot 0)
        shredVersion="$(cat "$LUMOS_CONFIG_DIR"/shred-version)"
        extraNodeArgs="$extraNodeArgs --expected-bank-hash $bankHash --expected-shred-version $shredVersion"
        echo "$bankHash" > config/bank-hash
      fi
    fi
    args=(
      --gossip-host "$entrypointIp"
      --gossip-port 8001
      --init-complete-file "$initCompleteFile"
    )

    if [[ "$tmpfsAccounts" = "true" ]]; then
      args+=(--accounts /mnt/lumos-accounts)
    fi

    if $maybeFullRpc; then
      args+=(--enable-rpc-transaction-history)
      args+=(--enable-extended-tx-metadata-storage)
    fi


    if $disableQuic; then
      args+=(--tpu-disable-quic)
    fi

    if $enableUdp; then
      args+=(--tpu-enable-udp)
    fi

    if [[ $airdropsEnabled = true ]]; then
cat >> ~/lumos/on-reboot <<EOF
      ./multinode-demo/faucet.sh > faucet.log 2>&1 &
EOF
    fi
    # shellcheck disable=SC2206 # Don't want to double quote $extraNodeArgs
    args+=($extraNodeArgs)

cat >> ~/lumos/on-reboot <<EOF
    nohup ./multinode-demo/bootstrap-validator.sh ${args[@]} > validator.log.\$now 2>&1 &
    pid=\$!
    oom_score_adj "\$pid" 1000
    disown
EOF
    ~/lumos/on-reboot

    if $waitForNodeInit; then
      net/remote/remote-node-wait-init.sh 600
    fi

    ;;
  validator|blockstreamer)
    if [[ $deployMethod != skip ]]; then
      net/scripts/rsync-retry.sh -vPrc "$entrypointIp":~/.cargo/bin/ ~/.cargo/bin/
      net/scripts/rsync-retry.sh -vPrc "$entrypointIp":~/version.yml ~/version.yml
    fi
    if [[ $skipSetup != true ]]; then
      clear_config_dir "$LUMOS_CONFIG_DIR"

      if [[ $nodeType = blockstreamer ]]; then
        net/scripts/rsync-retry.sh -vPrc \
          "$entrypointIp":~/lumos/config/blockstreamer-identity.json "$LUMOS_CONFIG_DIR"/validator-identity.json
      else
        net/scripts/rsync-retry.sh -vPrc \
          "$entrypointIp":~/lumos/config/validator-identity-"$nodeIndex".json "$LUMOS_CONFIG_DIR"/validator-identity.json
        net/scripts/rsync-retry.sh -vPrc \
          "$entrypointIp":~/lumos/config/validator-stake-"$nodeIndex".json "$LUMOS_CONFIG_DIR"/stake-account.json
        net/scripts/rsync-retry.sh -vPrc \
          "$entrypointIp":~/lumos/config/validator-vote-"$nodeIndex".json "$LUMOS_CONFIG_DIR"/vote-account.json
      fi
      net/scripts/rsync-retry.sh -vPrc \
        "$entrypointIp":~/lumos/config/shred-version "$LUMOS_CONFIG_DIR"/shred-version

      net/scripts/rsync-retry.sh -vPrc \
        "$entrypointIp":~/lumos/config/bank-hash "$LUMOS_CONFIG_DIR"/bank-hash || true

      net/scripts/rsync-retry.sh -vPrc \
        "$entrypointIp":~/lumos/config/faucet.json "$LUMOS_CONFIG_DIR"/faucet.json
    fi

    args=(
      --entrypoint "$entrypointIp:8001"
      --gossip-port 8001
      --rpc-port 8899
      --expected-shred-version "$(cat "$LUMOS_CONFIG_DIR"/shred-version)"
    )
    if [[ $nodeType = blockstreamer ]]; then
      args+=(
        --blockstream /tmp/lumos-blockstream.sock
        --no-voting
        --dev-no-sigverify
        --enable-rpc-transaction-history
      )
    else
      if [[ -n $internalNodesLamports ]]; then
        args+=(--node-lamports "$internalNodesLamports")
      fi
    fi

    if [[ ! -f "$LUMOS_CONFIG_DIR"/validator-identity.json ]]; then
      lumos-keygen new --no-passphrase -so "$LUMOS_CONFIG_DIR"/validator-identity.json
    fi
    args+=(--identity "$LUMOS_CONFIG_DIR"/validator-identity.json)
    if [[ ! -f "$LUMOS_CONFIG_DIR"/vote-account.json ]]; then
      lumos-keygen new --no-passphrase -so "$LUMOS_CONFIG_DIR"/vote-account.json
    fi
    args+=(--vote-account "$LUMOS_CONFIG_DIR"/vote-account.json)

    if [[ $airdropsEnabled != true ]]; then
      args+=(--no-airdrop)
    else
      args+=(--rpc-faucet-address "$entrypointIp:9900")
    fi

    if [[ -r "$LUMOS_CONFIG_DIR"/bank-hash ]]; then
      args+=(--expected-bank-hash "$(cat "$LUMOS_CONFIG_DIR"/bank-hash)")
    fi

    set -x
    # Add the faucet keypair to validators for convenient access from tools
    # like bench-tps and add to blocktreamers to run a faucet
    scp "$entrypointIp":~/lumos/config/faucet.json "$LUMOS_CONFIG_DIR"/
    if [[ $nodeType = blockstreamer ]]; then
      # Run another faucet with the same keypair on the blockstreamer node.
      # Typically the blockstreamer node has a static IP/DNS name for hosting
      # the blockexplorer web app, and is a location that somebody would expect
      # to be able to airdrop from
      if [[ $airdropsEnabled = true ]]; then
cat >> ~/lumos/on-reboot <<EOF
        multinode-demo/faucet.sh > faucet.log 2>&1 &
EOF
      fi

      # Grab the TLS cert generated by /certbot-restore.sh
      if [[ -f /.cert.pem ]]; then
        sudo install -o $UID -m 400 /.cert.pem /.key.pem .
        ls -l .cert.pem .key.pem
      fi
    fi

    args+=(--init-complete-file "$initCompleteFile")
    # shellcheck disable=SC2206 # Don't want to double quote $extraNodeArgs
    args+=($extraNodeArgs)

    maybeSkipAccountsCreation=
    if [[ $nodeIndex -le $extraPrimordialStakes ]]; then
      maybeSkipAccountsCreation="export SKIP_ACCOUNTS_CREATION=1"
    fi

    if [[ "$tmpfsAccounts" = "true" ]]; then
      args+=(--accounts /mnt/lumos-accounts)
    fi

    if $maybeFullRpc; then
      args+=(--enable-rpc-transaction-history)
      args+=(--enable-extended-tx-metadata-storage)
    fi

    if $disableQuic; then
      args+=(--tpu-disable-quic)
    fi

    if $enableUdp; then
      args+=(--tpu-enable-udp)
    fi

cat >> ~/lumos/on-reboot <<EOF
    $maybeSkipAccountsCreation
    nohup multinode-demo/validator.sh ${args[@]} > validator.log.\$now 2>&1 &
    pid=\$!
    oom_score_adj "\$pid" 1000
    disown
EOF
    ~/lumos/on-reboot

    if $waitForNodeInit; then
      net/remote/remote-node-wait-init.sh 600
    fi

    if [[ $skipSetup != true && $nodeType != blockstreamer && -z $maybeSkipAccountsCreation ]]; then
      # Wait for the validator to catch up to the bootstrap validator before
      # delegating stake to it
      lumos --url http://"$entrypointIp":8899 catchup config/validator-identity.json

      args=(
        --url http://"$entrypointIp":8899
      )
      if [[ $airdropsEnabled != true ]]; then
        args+=(--no-airdrop)
      fi
      if [[ -f config/validator-identity.json ]]; then
        args+=(--keypair config/validator-identity.json)
      fi

      if [[ ${extraPrimordialStakes} -eq 0 ]]; then
        echo "0 Primordial stakes, staking with $internalNodesStakeLamports"
        multinode-demo/delegate-stake.sh --vote-account "$LUMOS_CONFIG_DIR"/vote-account.json \
                                         --stake-account "$LUMOS_CONFIG_DIR"/stake-account.json \
                                         --force \
                                         "${args[@]}" "$internalNodesStakeLamports"
      else
        echo "Skipping staking with extra stakes: ${extraPrimordialStakes}"
      fi
    fi
    ;;
  *)
    echo "Error: unknown node type: $nodeType"
    exit 1
    ;;
  esac
  ;;
*)
  echo "Unknown deployment method: $deployMethod"
  exit 1
esac
