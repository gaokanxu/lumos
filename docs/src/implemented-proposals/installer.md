---
title: Cluster Software Installation and Updates
---

Currently users are required to build the lumos cluster software themselves from the git repository and manually update it, which is error prone and inconvenient.

This document proposes an easy to use software install and updater that can be used to deploy pre-built binaries for supported platforms. Users may elect to use binaries supplied by Lumos or any other party provider. Deployment of updates is managed using an on-chain update manifest program.

## Motivating Examples

### Fetch and run a pre-built installer using a bootstrap curl/shell script

The easiest install method for supported platforms:

```bash
$ curl -sSf https://raw.githubusercontent.com/lumos-labs/lumos/v1.0.0/install/lumos-install-init.sh | sh
```

This script will check github for the latest tagged release and download and run the `lumos-install-init` binary from there.

If additional arguments need to be specified during the installation, the following shell syntax is used:

```bash
$ init_args=.... # arguments for `lumos-install-init ...`
$ curl -sSf https://raw.githubusercontent.com/lumos-labs/lumos/v1.0.0/install/lumos-install-init.sh | sh -s - ${init_args}
```

### Fetch and run a pre-built installer from a Github release

With a well-known release URL, a pre-built binary can be obtained for supported platforms:

```bash
$ curl -o lumos-install-init https://github.com/lumos-labs/lumos/releases/download/v1.0.0/lumos-install-init-x86_64-apple-darwin
$ chmod +x ./lumos-install-init
$ ./lumos-install-init --help
```

### Build and run the installer from source

If a pre-built binary is not available for a given platform, building the installer from source is always an option:

```bash
$ git clone https://github.com/lumos-labs/lumos.git
$ cd lumos/install
$ cargo run -- --help
```

### Deploy a new update to a cluster

Given a lumos release tarball \(as created by `ci/publish-tarball.sh`\) that has already been uploaded to a publicly accessible URL, the following commands will deploy the update:

```bash
$ lumos-keygen new -o update-manifest.json  # <-- only generated once, the public key is shared with users
$ lumos-install deploy http://example.com/path/to/lumos-release.tar.bz2 update-manifest.json
```

### Run a validator node that auto updates itself

```bash
$ lumos-install init --pubkey 92DMonmBYXwEMHJ99c9ceRSpAmk9v6i3RdvDdXaVcrfj  # <-- pubkey is obtained from whoever is deploying the updates
$ export PATH=~/.local/share/lumos-install/bin:$PATH
$ lumos-keygen ...  # <-- runs the latest lumos-keygen
$ lumos-install run lumos-validator ...  # <-- runs a validator, restarting it as necessary when an update is applied
```

## On-chain Update Manifest

An update manifest is used to advertise the deployment of new release tarballs on a lumos cluster. The update manifest is stored using the `config` program, and each update manifest account describes a logical update channel for a given target triple \(eg, `x86_64-apple-darwin`\). The account public key is well-known between the entity deploying new updates and users consuming those updates.

The update tarball itself is hosted elsewhere, off-chain and can be fetched from the specified `download_url`.

```text
use lumos_sdk::signature::Signature;

/// Information required to download and apply a given update
pub struct UpdateManifest {
    pub timestamp_secs: u64, // When the release was deployed in seconds since UNIX EPOCH
    pub download_url: String, // Download URL to the release tar.bz2
    pub download_sha256: String, // SHA256 digest of the release tar.bz2 file
}

/// Data of an Update Manifest program Account.
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct SignedUpdateManifest {
    pub manifest: UpdateManifest,
    pub manifest_signature: Signature,
}
```

Note that the `manifest` field itself contains a corresponding signature \(`manifest_signature`\) to guard against man-in-the-middle attacks between the `lumos-install` tool and the lumos cluster RPC API.

To guard against rollback attacks, `lumos-install` will refuse to install an update with an older `timestamp_secs` than what is currently installed.

## Release Archive Contents

A release archive is expected to be a tar file compressed with bzip2 with the following internal structure:

- `/version.yml` - a simple YAML file containing the field `"target"` - the

  target tuple. Any additional fields are ignored.

- `/bin/` -- directory containing available programs in the release.

  `lumos-install` will symlink this directory to

  `~/.local/share/lumos-install/bin` for use by the `PATH` environment

  variable.

- `...` -- any additional files and directories are permitted

## lumos-install Tool

The `lumos-install` tool is used by the user to install and update their cluster software.

It manages the following files and directories in the user's home directory:

- `~/.config/lumos/install/config.yml` - user configuration and information about currently installed software version
- `~/.local/share/lumos/install/bin` - a symlink to the current release. eg, `~/.local/share/lumos-update/<update-pubkey>-<manifest_signature>/bin`
- `~/.local/share/lumos/install/releases/<download_sha256>/` - contents of a release

### Command-line Interface

```text
lumos-install 0.16.0
The lumos cluster software installer

USAGE:
    lumos-install [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <PATH>    Configuration file to use [default: .../Library/Preferences/lumos/install.yml]

SUBCOMMANDS:
    deploy    deploys a new update
    help      Prints this message or the help of the given subcommand(s)
    info      displays information about the current installation
    init      initializes a new installation
    run       Runs a program while periodically checking and applying software updates
    update    checks for an update, and if available downloads and applies it
```

```text
lumos-install-init
initializes a new installation

USAGE:
    lumos-install init [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -d, --data_dir <PATH>    Directory to store install data [default: .../Library/Application Support/lumos]
    -u, --url <URL>          JSON RPC URL for the lumos cluster [default: http://api.devnet.lumos.com]
    -p, --pubkey <PUBKEY>    Public key of the update manifest [default: 9XX329sPuskWhH4DQh6k16c87dHKhXLBZTL3Gxmve8Gp]
```

```text
lumos-install info
displays information about the current installation

USAGE:
    lumos-install info [FLAGS]

FLAGS:
    -h, --help     Prints help information
    -l, --local    only display local information, don't check the cluster for new updates
```

```text
lumos-install deploy
deploys a new update

USAGE:
    lumos-install deploy <download_url> <update_manifest_keypair>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <download_url>               URL to the lumos release archive
    <update_manifest_keypair>    Keypair file for the update manifest (/path/to/keypair.json)
```

```text
lumos-install update
checks for an update, and if available downloads and applies it

USAGE:
    lumos-install update

FLAGS:
    -h, --help    Prints help information
```

```text
lumos-install run
Runs a program while periodically checking and applying software updates

USAGE:
    lumos-install run <program_name> [program_arguments]...

FLAGS:
    -h, --help    Prints help information

ARGS:
    <program_name>            program to run
    <program_arguments>...    arguments to supply to the program

The program will be restarted upon a successful software update
```
