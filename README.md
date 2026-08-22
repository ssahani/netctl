<div align="center">

# netctl

**Network configuration CLI for Linux**

[![CI](https://github.com/hypersdk/netctl/actions/workflows/ci.yml/badge.svg)](https://github.com/hypersdk/netctl/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/hypersdk/netctl)](https://github.com/hypersdk/netctl/releases)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

[Quick start](#quick-start) · [Enterprise](#enterprise) · [Support the project](#support-the-project) · [Releases](https://github.com/hypersdk/netctl/releases)

<p align="center">
  <a href="https://zyvor.dev/demo?utm_source=github&utm_medium=netctl"><img src="https://img.shields.io/badge/Demo-F97316?style=flat-square" alt="Demo"/></a>
  <a href="https://zyvor.dev/docs?utm_source=github&utm_medium=netctl"><img src="https://img.shields.io/badge/Docs-2563EB?style=flat-square" alt="Docs"/></a>
  <a href="https://zyvor.dev/blog?utm_source=github&utm_medium=netctl"><img src="https://img.shields.io/badge/Blog-71717A?style=flat-square" alt="Blog"/></a>
  <a href="https://zyvor.dev/contact?utm_source=github&utm_medium=netctl"><img src="https://img.shields.io/badge/Contact_sales-22C55E?style=flat-square" alt="Contact"/></a>
</p>

</div>

---

Async-first network configuration manager with a `systemctl`-style interface. Integrates with **systemd-networkd**, **systemd-resolved**, and **systemd-hostnamed** over D-Bus, and uses **netlink** for link, address, and route operations.

## Table of contents

- [Features](#features)
- [Installation](#installation)
- [Quick start](#quick-start)
- [Configuration samples](#configuration-samples)
- [Architecture](#architecture)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [Enterprise](#enterprise)
- [Support the project](#support-the-project)
- [License](#license)
- [Related](#related)

## Features

- Link, address, route, DNS, and hostname management
- Declarative YAML/TOML apply, profiles, backup/restore, diff, and `doctor` diagnostics
- JSON output, watch mode, TUI, shell completions, and dry-run previews

## Installation

```bash
git clone https://github.com/hypersdk/netctl.git
cd netctl
cargo build --release
sudo cp target/release/netctl /usr/local/bin/
```

Pre-built binaries: [GitHub Releases](https://github.com/hypersdk/netctl/releases).

Docker:

```bash
docker build -t netctl:latest .
docker run --rm netctl:latest --help
```

Static musl build:

```bash
cargo install cross
cross build --release --target x86_64-unknown-linux-musl
```

## Quick start

```bash
sudo netctl show
sudo netctl link set eth0 state up
sudo netctl addr add eth0 192.168.1.100/24
netctl show --json
```

Declarative apply:

```bash
sudo netctl apply config/network-config.example.yaml
```

## Configuration samples

| File | Description |
|------|-------------|
| [config/network-config.example.yaml](config/network-config.example.yaml) | YAML network profile |
| [config/network-config.example.toml](config/network-config.example.toml) | TOML network profile |

## Architecture

Workspace crates: `netctl` (CLI), `netctl-core`, `netctl-netlink`, `netctl-dbus`, `netctl-config`, `netctl-types`.

## Development

```bash
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

## Troubleshooting

- Network changes require root or `CAP_NET_ADMIN`.
- D-Bus features need `systemd-networkd`, `systemd-resolved`, and `systemd-hostnamed` running.
- Debian/Ubuntu build deps: `sudo apt install libdbus-1-dev pkg-config`

## Contributing

Issues and PRs: [github.com/hypersdk/netctl](https://github.com/hypersdk/netctl/issues).

## Enterprise

| | Community Edition (this repo) | Enterprise ([zyvor.dev](https://zyvor.dev/?utm_source=github&utm_medium=netctl)) |
|---|------------------------------|-------------------------------------------------------------------------------------|
| **Support** | [GitHub Issues](https://github.com/hypersdk/netctl/issues) | SLA, [sales@zyvor.dev](mailto:sales@zyvor.dev), professional services |
| **Scope** | CLI and declarative apply | Supported rollouts with netevd and cloud-netconfig |
| **Platform** | netctl | Full HyperSDK networking and migration stack |
| **Features** | Link/address/route/DNS/hostname management, declarative YAML/TOML apply, `doctor` diagnostics, TUI, shell completions | Same feature set, operated at fleet scale with SLA-backed support |

| | |
|---|---|
| **Demo** | [zyvor.dev/demo](https://zyvor.dev/demo?utm_source=github&utm_medium=netctl) |
| **ROI** | [zyvor.dev/roi](https://zyvor.dev/roi?utm_source=github&utm_medium=netctl) |
| **Pricing** | [zyvor.dev/pricing](https://zyvor.dev/pricing?utm_source=github&utm_medium=netctl) |
| **Contact** | [zyvor.dev/contact](https://zyvor.dev/contact?utm_source=github&utm_medium=netctl) · [sales@zyvor.dev](mailto:sales@zyvor.dev) |

Community Edition covers CLI usage and declarative apply. Enterprise SLAs and the full stack with [netevd](https://github.com/hypersdk/netevd) and [cloud-netconfig](https://github.com/hypersdk/cloud-netconfig) → contact Zyvor (not GitHub Issues). Details: [docs/enterprise.md](docs/enterprise.md).

## Support the project

netctl Community Edition is free and open source, maintained by **Susant Sahani** · [Zyvor AI Labs](https://zyvor.dev?utm_source=github&utm_medium=netctl)

- **Enterprise / production:** [zyvor.dev/contact](https://zyvor.dev/contact?utm_source=github&utm_medium=netctl) · [sales@zyvor.dev](mailto:sales@zyvor.dev)
- **Community help:** [GitHub Issues](https://github.com/hypersdk/netctl/issues)

## License

netctl is licensed under the Apache License, Version 2.0.

Copyright © 2026 Zyvor AI Labs Private Limited.

This repository contains only the netctl Community Edition source code.

Other Zyvor products, platforms, services, and commercial offerings are separate works and may be governed by different licenses and terms.

Enterprise: [sales@zyvor.dev](mailto:sales@zyvor.dev) · General: [info@zyvor.dev](mailto:info@zyvor.dev).

## Related

[netevd](https://github.com/hypersdk/netevd) · [cloud-netconfig](https://github.com/hypersdk/cloud-netconfig) · [hypersdk](https://github.com/hypersdk/hypersdk)
