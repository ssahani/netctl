<div align="center">

<p align="center">
  <a href="https://zyvor.dev/?utm_source=github&utm_medium=netctl">
    <img src="docs/img/zyvor-logo.webp" alt="Zyvor AI Labs — HyperSDK Platform" width="220">
  </a>
</p>

# netctl

### Modern Network Configuration Tool for Linux

<sub>Part of the HyperSDK networking stack · by Zyvor AI Labs</sub>

[![CI](https://github.com/hypersdk/netctl/actions/workflows/ci.yml/badge.svg)](https://github.com/hypersdk/netctl/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/hypersdk/netctl)](https://github.com/hypersdk/netctl/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux-lightgrey.svg)](https://www.linux.org)

**Async-first network configuration manager written in Rust**

[Quick start](QUICKSTART.md) · [Documentation](docs/) · [Releases](https://github.com/hypersdk/netctl/releases)

</div>

---

## Overview

**netctl** is a network configuration CLI for Linux with a `systemctl`-style interface. It integrates with **systemd-networkd**, **systemd-resolved**, and **systemd-hostnamed** over D-Bus, and uses **netlink** for link, address, and route operations.

> **Enterprise:** [zyvor.dev](https://zyvor.dev/?utm_source=github&utm_medium=netctl) · [Contact sales](https://zyvor.dev/contact?utm_source=github&utm_medium=netctl)

## Features

- Async Tokio runtime with structured logging and JSON output
- Link, address, route, DNS, and hostname management
- Profiles, declarative YAML/TOML apply, backup/restore, diff, and `doctor` diagnostics
- Real-time TUI, watch mode, shell completions, and dry-run previews

## Installation

```bash
git clone https://github.com/hypersdk/netctl.git
cd netctl
cargo build --release
sudo cp target/release/netctl /usr/local/bin/
```

Pre-built binaries: [GitHub Releases](https://github.com/hypersdk/netctl/releases).  
Full install options (Docker, musl cross-build): [QUICKSTART.md](QUICKSTART.md).

## Quick start

```bash
sudo netctl show
sudo netctl link set eth0 state up
sudo netctl addr add eth0 192.168.1.100/24
netctl show --json
```

More examples: [QUICKSTART.md](QUICKSTART.md).

## Architecture

Workspace crates: `netctl` (CLI), `netctl-core`, `netctl-netlink`, `netctl-dbus`, `netctl-config`, `netctl-types`.

## Documentation

| Document | Description |
|----------|-------------|
| [QUICKSTART.md](QUICKSTART.md) | Install and first commands |
| [docs/](docs/) | Doc hub, enterprise guides |
| [CHANGELOG.md](CHANGELOG.md) | Release history |

## Development

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

## Troubleshooting

- Network changes require root or `CAP_NET_ADMIN`.
- D-Bus features need `systemd-networkd`, `systemd-resolved`, and `systemd-hostnamed` running.
- Build deps: `libdbus-1-dev`, `pkg-config` (Debian/Ubuntu: `sudo apt install libdbus-1-dev pkg-config`).

## Contributing

Issues and PRs welcome at [github.com/hypersdk/netctl](https://github.com/hypersdk/netctl). See [CONTRIBUTING.md](CONTRIBUTING.md) if present.

## License

Apache License 2.0 — see [LICENSE](LICENSE).

## Support

<p align="center">
  <a href="https://zyvor.dev/">
    <img src="docs/img/zyvor-logo.webp" alt="Zyvor AI Labs" width="220">
  </a>
</p>

**netctl** is part of the [HyperSDK Platform](https://zyvor.dev/) by [Zyvor AI Labs](https://zyvor.dev/).

| | |
|---|---|
| **Issues (OSS)** | [github.com/hypersdk/netctl/issues](https://github.com/hypersdk/netctl/issues) |
| **Enterprise** | [zyvor.dev/contact](https://zyvor.dev/contact) · [sales@zyvor.dev](mailto:sales@zyvor.dev) |

Related: [netevd](https://github.com/hypersdk/netevd) · [cloud-netconfig](https://github.com/hypersdk/cloud-netconfig) · [HyperSDK](https://github.com/hypersdk/hypersdk)

[Open source vs Enterprise](docs/ce-vs-enterprise.md) · [Enterprise guide](docs/zyvor-enterprise.md)
