---
title: Install the Lumos CLI
pagination_label: Install the Lumos CLI
sidebar_label: Installation
sidebar_position: 1
---

There are multiple ways to install the Lumos tools on your computer depending
on your preferred workflow:

- [Use Lumos's Install Tool (Simplest option)](#use-lumoss-install-tool)
- [Download Prebuilt Binaries](#download-prebuilt-binaries)
- [Build from Source](#build-from-source)
- [Use Homebrew](#use-homebrew)

## Use Lumos's Install Tool

### MacOS & Linux

- Open your favorite Terminal application

- Install the Lumos release
  [LATEST_LUMOS_RELEASE_VERSION](https://github.com/lumos-labs/lumos/releases/tag/LATEST_LUMOS_RELEASE_VERSION)
  on your machine by running:

```bash
sh -c "$(curl -sSfL https://release.lumos.com/LATEST_LUMOS_RELEASE_VERSION/install)"
```

- You can replace `LATEST_LUMOS_RELEASE_VERSION` with the release tag matching
  the software version of your desired release, or use one of the three symbolic
  channel names: `stable`, `beta`, or `edge`.

- The following output indicates a successful update:

```text
downloading LATEST_LUMOS_RELEASE_VERSION installer
Configuration: /home/lumos/.config/lumos/install/config.yml
Active release directory: /home/lumos/.local/share/lumos/install/active_release
* Release version: LATEST_LUMOS_RELEASE_VERSION
* Release URL: https://github.com/lumos-labs/lumos/releases/download/LATEST_LUMOS_RELEASE_VERSION/lumos-release-x86_64-unknown-linux-gnu.tar.bz2
Update successful
```

- Depending on your system, the end of the installer messaging may prompt you to

```bash
Please update your PATH environment variable to include the lumos programs:
```

- If you get the above message, copy and paste the recommended command below it
  to update `PATH`
- Confirm you have the desired version of `lumos` installed by running:

```bash
lumos --version
```

- After a successful install, `lumos-install update` may be used to easily
  update the Lumos software to a newer version at any time.

---

### Windows

- Open a Command Prompt (`cmd.exe`) as an Administrator

  - Search for Command Prompt in the Windows search bar. When the Command Prompt
    app appears, right-click and select “Open as Administrator”. If you are
    prompted by a pop-up window asking “Do you want to allow this app to make
    changes to your device?”, click Yes.

- Copy and paste the following command, then press Enter to download the Lumos
  installer into a temporary directory:

```bash
cmd /c "curl https://release.lumos.com/LATEST_LUMOS_RELEASE_VERSION/lumos-install-init-x86_64-pc-windows-msvc.exe --output C:\lumos-install-tmp\lumos-install-init.exe --create-dirs"
```

- Copy and paste the following command, then press Enter to install the latest
  version of Lumos. If you see a security pop-up by your system, please select
  to allow the program to run.

```bash
C:\lumos-install-tmp\lumos-install-init.exe LATEST_LUMOS_RELEASE_VERSION
```

- When the installer is finished, press Enter.

- Close the command prompt window and re-open a new command prompt window as a
  normal user
  - Search for "Command Prompt" in the search bar, then left click on the
    Command Prompt app icon, no need to run as Administrator)
- Confirm you have the desired version of `lumos` installed by entering:

```bash
lumos --version
```

- After a successful install, `lumos-install update` may be used to easily
  update the Lumos software to a newer version at any time.

## Download Prebuilt Binaries

If you would rather not use `lumos-install` to manage the install, you can
manually download and install the binaries.

### Linux

Download the binaries by navigating to
[https://github.com/lumos-labs/lumos/releases/latest](https://github.com/lumos-labs/lumos/releases/latest),
download **lumos-release-x86_64-unknown-linux-gnu.tar.bz2**, then extract the
archive:

```bash
tar jxf lumos-release-x86_64-unknown-linux-gnu.tar.bz2
cd lumos-release/
export PATH=$PWD/bin:$PATH
```

### MacOS

Download the binaries by navigating to
[https://github.com/lumos-labs/lumos/releases/latest](https://github.com/lumos-labs/lumos/releases/latest),
download **lumos-release-x86_64-apple-darwin.tar.bz2**, then extract the
archive:

```bash
tar jxf lumos-release-x86_64-apple-darwin.tar.bz2
cd lumos-release/
export PATH=$PWD/bin:$PATH
```

### Windows

- Download the binaries by navigating to
  [https://github.com/lumos-labs/lumos/releases/latest](https://github.com/lumos-labs/lumos/releases/latest),
  download **lumos-release-x86_64-pc-windows-msvc.tar.bz2**, then extract the
  archive using WinZip or similar.

- Open a Command Prompt and navigate to the directory into which you extracted
  the binaries and run:

```bash
cd lumos-release/
set PATH=%cd%/bin;%PATH%
```

## Build From Source

If you are unable to use the prebuilt binaries or prefer to build it yourself
from source, follow these steps, ensuring you have the necessary prerequisites
installed on your system.

### Prerequisites

Before building from source, make sure to install the following prerequisites:

#### For Debian and Other Linux Distributions:

Rust Programming Language: Check "Install Rust" at
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install),
which recommends the following command.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install build dependencies:

- Build essential
- Package config
- Udev & LLM & libclang
- Protocol buffers

```bash
apt-get install \
    build-essential \
    pkg-config \
    libudev-dev llvm libclang-dev \
    protobuf-compiler
```

#### For Other Linux Distributions:

Replace `apt` with your distribution's package manager (e.g., `yum`, `dnf`,
`pacman`) and adjust package names as needed.

#### For macOS:

Install Homebrew (if not already installed), check "Install Hombrew" at
[https://brew.sh/](https://brew.sh/), which recommends the following command:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Install the necessary tools and libraries using Homebrew:

```bash
brew install rust pkg-config libudev protobuf llvm coreutils
```

Follow the instructions given at the end of the brew install command about
`PATH` configurations.

#### For Windows:

Rust Programming Language: Check "Install Rust" at
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install),
which recommends the following command.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Download and install the Build Tools for Visual Studio (2019 or later) from
  the
  [Visual Studio downloads page](https://visualstudio.microsoft.com/downloads/).
  Make sure to include the C++ build tools in the installation.
- Install LLVM: Download and install LLVM from the
  [official LLVM download page](https://releases.llvm.org/download.html).
- Install Protocol Buffers Compiler (protoc): Download `protoc` from the
  [GitHub releases page of Protocol Buffers](https://github.com/protocolbuffers/protobuf/releases),
  and add it to your `PATH`.

:::info

Users on Windows 10 or 11 may need to install
[Windows Subsystem for Linux](https://learn.microsoft.com/en-us/windows/wsl/install)
(WSL) in order to be able to build from source. WSL provides a Linux environment
that runs inside your existing Windows installation. You can then run regular
Linux software, including the Linux versions of Lumos CLI.

After installed, run `wsl` from your Windows terminal, then continue through the
[Debian and Other Linux Distributions](#for-debian-and-other-linux-distributions)
above.

:::

### Building from Source

After installing the prerequisites, proceed with building Lumos from source,
navigate to
[Lumos's GitHub releases page](https://github.com/lumos-labs/lumos/releases/latest),
and download the **Source Code** archive. Extract the code and build the
binaries with:

```bash
./scripts/cargo-install-all.sh .
export PATH=$PWD/bin:$PATH
```

You can then run the following command to obtain the same result as with
prebuilt binaries:

```bash
lumos-install init
```

## Use Homebrew

This option requires you to have [Homebrew](https://brew.sh/) package manager on
your MacOS or Linux machine.

### MacOS & Linux

- Follow instructions at: https://formulae.brew.sh/formula/lumos

[Homebrew formulae](https://github.com/Homebrew/homebrew-core/blob/HEAD/Formula/lumos.rb)
is updated after each `lumos` release, however it is possible that the Homebrew
version is outdated.

- Confirm you have the desired version of `lumos` installed by entering:

```bash
lumos --version
```
