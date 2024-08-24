#!/usr/bin/env bash
set -ex

[[ $(uname) = Linux ]] || exit 1
[[ $USER = root ]] || exit 1

if grep -q lumos /etc/passwd ; then
  echo "User lumos already exists"
else
  adduser lumos --gecos "" --disabled-password --quiet
  adduser lumos sudo
  adduser lumos adm
  echo "lumos ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
  id lumos

  [[ -r /lumos-scratch/id_ecdsa ]] || exit 1
  [[ -r /lumos-scratch/id_ecdsa.pub ]] || exit 1

  sudo -u lumos bash -c "
    echo 'PATH=\"/home/lumos/.cargo/bin:$PATH\"' > /home/lumos/.profile
    mkdir -p /home/lumos/.ssh/
    cd /home/lumos/.ssh/
    cp /lumos-scratch/id_ecdsa.pub authorized_keys
    umask 377
    cp /lumos-scratch/id_ecdsa id_ecdsa
    echo \"
      Host *
      BatchMode yes
      IdentityFile ~/.ssh/id_ecdsa
      StrictHostKeyChecking no
    \" > config
  "
fi
