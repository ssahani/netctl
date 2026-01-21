# netctl - Modern Network Configuration Tool

> **Async-first network configuration manager written in Rust**

[![CI](https://github.com/vmware/netctl/actions/workflows/ci.yml/badge.svg)](https://github.com/vmware/netctl/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE.txt)

**netctl** is a modern, async-first network configuration tool for Linux, designed as a complete rewrite of network-config-manager in Rust. It provides a clean CLI interface following the systemd naming convention (like `systemctl`, `hostnamectl`, `timedatectl`).

## Features

- 🚀 **Async/Await** - Built on Tokio for non-blocking operations
- 🔒 **Type-Safe** - Leverages Rust's type system for correctness
- 🎯 **Modern CLI** - Clean, intuitive interface with rich error messages
- 📊 **Structured Logging** - Production-grade observability with tracing
- 📦 **Static Binary** - Single executable with no dependencies
- 🔧 **systemd Integration** - Full D-Bus support for networkd, resolved, and hostnamed
- 📋 **JSON Output** - Machine-readable output for automation
- ✅ **Well-Tested** - 21 unit tests with comprehensive coverage

## Quick Start

### Build

```bash
# Development build
cargo build

# Release build
cargo build --release

# Static binary
cross build --release --target x86_64-unknown-linux-musl
```

### Usage

```bash
# Show all interfaces (table format)
netctl show

# Show specific interface details
netctl show eth0

# Show in JSON format (for automation)
netctl show --json
netctl show eth0 --json

# Set link up/down
netctl link set eth0 --state up
netctl link set eth0 --state down

# Set MTU
netctl link set eth0 --mtu 9000

# Add IP address
netctl addr add eth0 192.168.1.10/24
```

## Architecture

```
crates/
├── netctl/          # CLI binary
├── netctl-core/     # Business logic
├── netctl-netlink/  # Async netlink operations
├── netctl-dbus/     # D-Bus integration
├── netctl-config/   # Configuration management
└── netctl-types/    # Core types and traits
```

## Commands

### Link Management

```bash
# Set link state
netctl link set INTERFACE --state {up|down}

# Set MTU
netctl link set INTERFACE --mtu MTU

# Show link info
netctl show [INTERFACE]
```

### Address Management

```bash
# Add IP address
netctl addr add INTERFACE ADDRESS/PREFIX

# Remove IP address
netctl addr del INTERFACE ADDRESS/PREFIX

# List addresses
netctl addr list INTERFACE
```

### systemd Integration

**systemd-networkd:**
```bash
# Reload networkd configuration
netctl networkd reload

# Reconfigure specific interface
netctl networkd reconfigure eth0
```

**systemd-resolved (DNS):**
```bash
# Set DNS servers
netctl dns set eth0 8.8.8.8 8.8.4.4

# Set search domains
netctl dns domains eth0 example.com

# Revert DNS configuration
netctl dns revert eth0

# Flush DNS caches
netctl dns flush
```

**systemd-hostnamed:**
```bash
# Set hostname
netctl hostname set myhost

# Get hostname
netctl hostname get
```

### JSON Output

All show commands support JSON output for automation:

```bash
# Get all interfaces as JSON
netctl show --json | jq '.[].name'

# Get specific interface as JSON
netctl show eth0 --json | jq '.state'
```

## Development

### Prerequisites

- Rust 1.75+
- `libdbus-1-dev` (for D-Bus support)
- Linux kernel 3.16+ (for netlink)

### Commands

```bash
# Check code
cargo check --workspace

# Run tests
cargo test --workspace

# Format
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings
```

## Design Principles

1. **Async Throughout** - All I/O operations are async
2. **Type Safety** - Compile-time guarantees
3. **Rich Errors** - Helpful error messages with miette
4. **Zero Panics** - Explicit error handling
5. **Idiomatic Rust** - Follow Rust best practices

## Comparison with C Version

| Feature | C Version | Rust Version |
|---------|-----------|--------------|
| Safety | Manual | Compile-time |
| Concurrency | Sync | Async/await |
| Errors | Codes | Rich types |
| CLI | Basic | Modern |
| Binary | ~500KB | ~5MB |

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Apache-2.0

## Links

- [Issue Tracker](https://github.com/vmware/netctl/issues)
- [Discussions](https://github.com/vmware/netctl/discussions)
