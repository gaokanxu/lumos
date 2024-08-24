#!/usr/bin/env bash

set -x
! tmux list-sessions || tmux kill-session
declare sudo=
if sudo true; then
  sudo="sudo -n"
fi

echo "pwd: $(pwd)"
for pid in lumos/*.pid; do
  pgid=$(ps opgid= "$(cat "$pid")" | tr -d '[:space:]')
  if [[ -n $pgid ]]; then
    $sudo kill -- -"$pgid"
  fi
done
if [[ -f lumos/netem.cfg ]]; then
  lumos/scripts/netem.sh delete < lumos/netem.cfg
  rm -f lumos/netem.cfg
fi
lumos/scripts/net-shaper.sh cleanup
for pattern in validator.sh boostrap-leader.sh lumos- remote- iftop validator client node; do
  echo "killing $pattern"
  pkill -f $pattern
done
