---
title: Backward Compatibility Policy
---

As the Lumos developer ecosystem grows, so does the need for clear expectations around
breaking API and behavior changes affecting applications and tooling built for Lumos.
In a perfect world, Lumos development could continue at a very fast pace without ever
causing issues for existing developers. However, some compromises will need to be made
and so this document attempts to clarify and codify the process for new releases.

### Expectations

- Lumos software releases include APIs, SDKs, and CLI tooling (with a few [exceptions](#exceptions)).
- Lumos software releases follow semantic versioning, more details below.
- Software for a `MINOR` version release will be compatible across all software on the
  same `MAJOR` version.

### Deprecation Process

1. In any `PATCH` or `MINOR` release, a feature, API, endpoint, etc. could be marked as deprecated.
2. According to code upgrade difficulty, some features will be remain deprecated for a few release
   cycles.
3. In a future `MAJOR` release, deprecated features will be removed in an incompatible way.

### Release Cadence

The Lumos RPC API, Rust SDK, CLI tooling, and SBF Program SDK are all updated and shipped
along with each Lumos software release and should always be compatible between `PATCH`
updates of a particular `MINOR` version release.

#### Release Channels

- `edge` software that contains cutting-edge features with no backward compatibility policy
- `beta` software that runs on the Lumos Testnet cluster
- `stable` software that run on the Lumos Mainnet Beta and Devnet clusters

#### Major Releases (x.0.0)

`MAJOR` version releases (e.g. 2.0.0) may contain breaking changes and removal of previously
deprecated features. Client SDKs and tooling will begin using new features and endpoints
that were enabled in the previous `MAJOR` version.

#### Minor Releases (1.x.0)

New features and proposal implementations are added to _new_ `MINOR` version
releases (e.g. 1.4.0) and are first run on Lumos's Testnet cluster. While running
on the testnet, `MINOR` versions are considered to be in the `beta` release channel. After
those changes have been patched as needed and proven to be reliable, the `MINOR` version will
be upgraded to the `stable` release channel and deployed to the Mainnet Beta cluster.

#### Patch Releases (1.0.x)

Low risk features, non-breaking changes, and security and bug fixes are shipped as part
of `PATCH` version releases (e.g. 1.0.11). Patches may be applied to both `beta` and `stable`
release channels.

### RPC API

Patch releases:

- Bug fixes
- Security fixes
- Endpoint / feature deprecation

Minor releases:

- New RPC endpoints and features

Major releases:

- Removal of deprecated features

### Rust Crates

- [`lumos-sdk`](https://docs.rs/lumos-sdk/) - Rust SDK for creating transactions and parsing account state
- [`lumos-program`](https://docs.rs/lumos-program/) - Rust SDK for writing programs
- [`lumos-client`](https://docs.rs/lumos-client/) - Rust client for connecting to RPC API
- [`lumos-cli-config`](https://docs.rs/lumos-cli-config/) - Rust client for managing Lumos CLI config files
- [`lumos-geyser-plugin-interface`](https://docs.rs/lumos-geyser-plugin-interface/) - Rust interface for developing Lumos Geyser plugins.

Patch releases:

- Bug fixes
- Security fixes
- Performance improvements

Minor releases:

- New APIs

Major releases

- Removal of deprecated APIs
- Backwards incompatible behavior changes

### CLI Tools

Patch releases:

- Bug and security fixes
- Performance improvements
- Subcommand / argument deprecation

Minor releases:

- New subcommands

Major releases:

- Switch to new RPC API endpoints / configuration introduced in the previous major version.
- Removal of deprecated features

### Runtime Features

New Lumos runtime features are feature-switched and manually activated. Runtime features
include: the introduction of new native programs, sysvars, and syscalls; and changes to
their behavior. Feature activation is cluster agnostic, allowing confidence to be built on
Testnet before activation on Mainnet-beta.

The release process is as follows:

1. New runtime feature is included in a new release, deactivated by default
2. Once sufficient staked validators upgrade to the new release, the runtime feature switch
   is activated manually with an instruction
3. The feature takes effect at the beginning of the next epoch

### Infrastructure Changes

#### Public API Nodes

Lumos provides publicly available RPC API nodes for all developers to use. The Lumos team
will make their best effort to communicate any changes to the host, port, rate-limiting behavior,
availability, etc. However, we recommend that developers rely on their own validator nodes to
discourage dependence upon Lumos operated nodes.

#### Local cluster scripts and Docker images

Breaking changes will be limited to `MAJOR` version updates. `MINOR` and `PATCH` updates should always
be backwards compatible.

### Exceptions

#### Web3 JavaScript SDK

The Web3.JS SDK also follows semantic versioning specifications but is shipped separately from Lumos
software releases.

#### Attack Vectors

If a new attack vector is discovered in existing code, the above processes may be
circumvented in order to rapidly deploy a fix, depending on the severity of the issue.

#### CLI Tooling Output

CLI tooling json output (`output --json`) compatibility will be preserved; however, output directed
for a human reader is subject to change. This includes output as well as potential help, warning, or
error messages.
