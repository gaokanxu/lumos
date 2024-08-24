#!/usr/bin/env bash

HERE="$(dirname "$0")"

# shellcheck source=ci/setup-new-buildkite-agent/utils.sh
source "$HERE"/utils.sh

ensure_env || exit 1

# Setup kernel constants
cat > /etc/sysctl.d/20-lumos-node.conf <<EOF

# Lumos networking requirements
net.core.rmem_default=134217728
net.core.rmem_max=134217728
net.core.wmem_default=134217728
net.core.wmem_max=134217728

# Lumos earlyoom setup
kernel.sysrq=$(( $(cat /proc/sys/kernel/sysrq) | 64 ))

# Allow kernel and CPU perf events
kernel.perf_event_paranoid=0
EOF

