# Installation Guide

## Prerequisites

- Rust 1.70 or later
- For system clipboard integration: platform-specific dependencies
  - Linux: X11 or Wayland libraries
  - macOS: No additional dependencies
  - Windows: No additional dependencies

## Installation Methods

### Cargo Install

```bash
cargo install morsel
```

This installs the `morsel` CLI, `morseld` daemon, and all required components.

### From Source

```bash
git clone https://github.com/pomagrenate/morsel.git
cd morsel
cargo install --path .
```

### Prebuilt Binaries

Download the appropriate binary for your platform from the [releases page](https://github.com/pomagrenate/morsel/releases).

#### Linux

```bash
wget https://github.com/pomagrenate/morsel/releases/download/v0.1.0/morsel-0.1.0-linux-x86_64.tar.gz
tar xzf morsel-0.1.0-linux-x86_64.tar.gz
sudo cp morsel morseld /usr/local/bin/
```

#### macOS

```bash
curl -L https://github.com/pomagrenate/morsel/releases/download/v0.1.0/morsel-0.1.0-darwin-x86_64.tar.gz -o morsel.tar.gz
tar xzf morsel.tar.gz
sudo cp morsel morseld /usr/local/bin/
```

#### Windows

Download the `.zip` file from the releases page, extract it, and add the extracted directory to your PATH.

### Install Script

```bash
curl -sSL https://install.morsel.sh | sh
```

## Verification

Verify the installation:

```bash
morsel --version
morseld --version
```

## Platform-Specific Setup

### Linux

For X11:
```bash
sudo apt install libx11-dev libxext-dev  # Debian/Ubuntu
sudo dnf install libX11-devel libXext-devel  # Fedora
```

For Wayland:
```bash
sudo apt install libwayland-dev  # Debian/Ubuntu
sudo dnf install wayland-devel  # Fedora
```

### macOS

No additional setup required.

### Windows

No additional setup required.

## Configuration

After installation, create a configuration file:

```bash
morsel config init
```

This creates `~/.config/morsel/config.toml` with default settings.

## Starting the Daemon

Start the morsel daemon:

```bash
morseld start
```

Or run it in the foreground:

```bash
morseld run
```

## Uninstallation

### Cargo Install

```bash
cargo uninstall morsel
```

### From Source/Prebuilt

Remove the binaries:

```bash
sudo rm /usr/local/bin/morsel /usr/local/bin/morseld
```

Remove configuration:

```bash
rm -rf ~/.config/morsel
```

Remove data:

```bash
rm -rf ~/.local/share/morsel
```
