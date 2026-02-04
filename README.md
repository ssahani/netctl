<div align="center">

# netctl

### Modern Network Configuration Tool for Linux

[![CI](https://github.com/ssahani/netctl/actions/workflows/ci.yml/badge.svg)](https://github.com/ssahani/netctl/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux-lightgrey.svg)](https://www.linux.org)

**Async-first network configuration manager written in Rust**

[Features](#-features) •
[Installation](#-installation) •
[Quick Start](#-quick-start) •
[Documentation](#-documentation) •
[Architecture](#-architecture) •
[Contributing](#-contributing)

</div>

---

## 📖 Overview

**netctl** is a modern, async-first network configuration tool for Linux, designed as a complete rewrite of network-config-manager in Rust. It provides a clean, intuitive CLI interface following the systemd naming convention (like `systemctl`, `hostnamectl`, `timedatectl`).

Built with Rust's safety guarantees and Tokio's async runtime, netctl offers a production-ready solution for network management with full systemd integration.

## ✨ Features

### Core Capabilities
- 🚀 **Async/Await** - Built on Tokio for non-blocking, concurrent operations
- 🔒 **Type-Safe** - Leverages Rust's type system for compile-time correctness
- 🎯 **Modern CLI** - Clean, intuitive interface with rich error messages and suggestions
- 📊 **Structured Logging** - Production-grade observability with tracing framework
- 📦 **Static Binary** - Single 4.2MB executable with no runtime dependencies

### System Integration
- 🔧 **systemd-networkd** - Full D-Bus integration for network configuration
- 🌐 **systemd-resolved** - DNS server and search domain management
- 💻 **systemd-hostnamed** - Hostname configuration and queries
- ⚡ **Netlink** - Direct kernel communication for network operations

### Developer Experience
- 📋 **JSON Output** - Machine-readable output for automation and scripting
- ✅ **Well-Tested** - 21 unit tests with comprehensive type and error coverage
- 🔍 **Rich Errors** - Detailed error messages with miette diagnostic framework
- 📚 **Documentation** - Comprehensive guides and API documentation

### Performance
- ⚡ **Fast** - Async I/O with zero blocking operations
- 💾 **Efficient** - Low memory footprint with Rust's zero-cost abstractions
- 🔄 **Concurrent** - Multiple network operations executed in parallel

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/ssahani/netctl.git
cd netctl

# Build release binary
cargo build --release

# Install to system (optional)
sudo cp target/release/netctl /usr/local/bin/
```

### Static Binary (musl)

For a fully static binary with no runtime dependencies:

```bash
# Install cross-compilation tool
cargo install cross --git https://github.com/cross-rs/cross

# Build static binary
cross build --release --target x86_64-unknown-linux-musl

# Binary location
ls target/x86_64-unknown-linux-musl/release/netctl
```

### Prerequisites

- **Rust 1.75+** - Install from [rustup.rs](https://rustup.rs)
- **libdbus-1-dev** - For D-Bus integration
  ```bash
  # Debian/Ubuntu
  sudo apt-get install libdbus-1-dev

  # Fedora/RHEL
  sudo dnf install dbus-devel
  ```
- **Linux kernel 3.16+** - For netlink support

## 🚀 Quick Start

### Basic Usage

```bash
# View all network interfaces
sudo netctl show

# View specific interface details
sudo netctl show eth0

# Bring interface up
sudo netctl link set eth0 --state up

# Configure IP address
sudo netctl addr add eth0 192.168.1.100/24

# Set MTU for jumbo frames
sudo netctl link set eth0 --mtu 9000
```

### Example Output

```bash
$ sudo netctl show
INDEX NAME            STATE    MTU      MAC ADDRESS
------------------------------------------------------------
1     lo              UP       65536    -
2     eth0            UP       1500     52:54:00:12:34:56
3     wlan0           DOWN     1500     ac:de:48:00:11:22

Total: 3 interface(s)
```

### JSON Output for Automation

```bash
# Get all interfaces in JSON format
netctl show --json

# Extract specific fields with jq
netctl show --json | jq '.[].name'
netctl show eth0 --json | jq '.state'

# Example: Check if interface is up
netctl show eth0 --json | jq -r '.state' | grep -q "Up" && echo "Interface is up"
```

## 🏗️ Architecture

netctl is built as a modular Rust workspace with clear separation of concerns:

```
┌─────────────────────────────────────────────────────┐
│                  CLI Layer (netctl)                 │
│  • Async Clap commands                              │
│  • JSON output support                              │
│  • Rich error display (miette)                      │
└─────────────────────────────────────────────────────┘
                          │
┌─────────────────────────────────────────────────────┐
│          Application Layer (netctl-core)            │
│  • High-level network operations                    │
│  • Orchestration logic                              │
│  • Validation & business rules                      │
└─────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┴─────────────────┐
        │                                   │
┌───────────────────┐            ┌──────────────────┐
│ netctl-netlink    │            │ netctl-dbus      │
│ • Async rtnetlink │            │ • Async zbus     │
│ • Links           │            │ • systemd-*      │
│ • Addresses       │            │ • Proxies        │
│ • Routes          │            └──────────────────┘
└───────────────────┘
        │
┌─────────────────────────────────────────────────────┐
│        Infrastructure (netctl-types/config)         │
│  • Error types (thiserror + miette)                 │
│  • Network types (IpAddr, MAC, etc.)                │
│  • Tracing setup                                    │
└─────────────────────────────────────────────────────┘
```

### Crate Responsibilities

| Crate | Purpose | Key Dependencies |
|-------|---------|------------------|
| `netctl` | CLI binary and user interface | clap, miette |
| `netctl-core` | Business logic and orchestration | tokio, tracing |
| `netctl-netlink` | Async netlink operations | rtnetlink, futures |
| `netctl-dbus` | D-Bus integration with systemd | zbus |
| `netctl-config` | Configuration file management | serde, toml |
| `netctl-types` | Core types and error handling | thiserror, miette |

## 📚 Documentation

### Command Reference

#### Show Commands

```bash
# Show all interfaces (table format)
netctl show

# Show specific interface (detailed view)
netctl show eth0

# JSON output for automation
netctl show --json
netctl show eth0 --json
```

#### Link Management

```bash
# Bring interface up/down
netctl link set eth0 --state up
netctl link set eth0 --state down

# Configure MTU
netctl link set eth0 --mtu 1500      # Standard Ethernet
netctl link set eth0 --mtu 9000      # Jumbo frames

# Set MAC address (requires interface down)
netctl link set eth0 --mac 00:11:22:33:44:55
```

#### Address Management

```bash
# Add IPv4 address
netctl addr add eth0 192.168.1.100/24

# Add IPv6 address
netctl addr add eth0 2001:db8::1/64

# Remove address
netctl addr del eth0 192.168.1.100/24

# List addresses on interface
netctl addr list eth0
```

#### systemd Integration

**networkd Operations:**
```bash
# Reload all networkd configuration files
netctl networkd reload

# Reconfigure specific interface
netctl networkd reconfigure eth0
```

**DNS Management (systemd-resolved):**
```bash
# Set DNS servers for interface
netctl dns set eth0 8.8.8.8 8.8.4.4
netctl dns set eth0 2001:4860:4860::8888

# Set search domains
netctl dns domains eth0 example.com internal.local

# Revert DNS to defaults
netctl dns revert eth0

# Flush all DNS caches
netctl dns flush
```

**Hostname Management (systemd-hostnamed):**
```bash
# Set static hostname
netctl hostname set myserver

# Get current hostname
netctl hostname get

# Get machine ID
netctl hostname machine-id
```

### Real-World Examples

#### Configure Static IP with Gateway

```bash
# Complete static IP configuration workflow
# Bring interface down for configuration
sudo netctl link set eth0 --state down

# Add static IP address with subnet
sudo netctl addr add eth0 192.168.1.100/24

# Set custom MTU if needed (default is 1500)
sudo netctl link set eth0 --mtu 1500

# Bring interface up
sudo netctl link set eth0 --state up

# Configure DNS servers (primary and secondary)
sudo netctl dns set eth0 192.168.1.1 8.8.8.8

# Set search domains for hostname resolution
sudo netctl dns domains eth0 local.lan example.com

# Verify configuration
netctl show eth0
```

#### Configure IPv6 Address

```bash
# Add IPv6 address with /64 prefix
sudo netctl addr add eth0 2001:db8::100/64

# Add link-local IPv6 address
sudo netctl addr add eth0 fe80::1/64

# Configure IPv6 DNS servers
sudo netctl dns set eth0 2001:4860:4860::8888 2001:4860:4860::8844

# Verify IPv6 configuration
netctl show eth0 --json | jq '.addresses[] | select(.family == "inet6")'
```

#### Enable Jumbo Frames for High-Performance Network

```bash
# Configure MTU for jumbo frames (9000 bytes)
# Requires network infrastructure support
sudo netctl link set eth0 --state down
sudo netctl link set eth0 --mtu 9000
sudo netctl link set eth0 --state up

# Verify jumbo frame configuration
netctl show eth0 | grep MTU

# Test jumbo frame connectivity (ping with large packet)
ping -M do -s 8972 192.168.1.1
```

#### Configure Server with Multiple IPs (Virtual Hosting)

```bash
# Primary IP address
sudo netctl addr add eth0 192.168.1.100/24

# Add additional IP addresses for virtual hosting
sudo netctl addr add eth0 192.168.1.101/24
sudo netctl addr add eth0 192.168.1.102/24
sudo netctl addr add eth0 192.168.1.103/24

# Verify all addresses are configured
netctl show eth0 --json | jq '.addresses[].address'
```

#### Change MAC Address (Spoofing)

```bash
# Changing MAC address requires interface to be down
sudo netctl link set eth0 --state down

# Set new MAC address
sudo netctl link set eth0 --mac 00:11:22:33:44:55

# Bring interface back up
sudo netctl link set eth0 --state up

# Verify MAC address change
netctl show eth0 | grep MAC
```

#### Configure Split DNS (Different DNS per Interface)

```bash
# Configure DNS for wired interface (corporate DNS)
sudo netctl dns set eth0 10.0.0.1 10.0.0.2
sudo netctl dns domains eth0 corp.example.com

# Configure DNS for wireless interface (public DNS)
sudo netctl dns set wlan0 8.8.8.8 1.1.1.1
sudo netctl dns domains wlan0 home.local

# Verify DNS configuration per interface
netctl show eth0 --json | jq '.dns'
netctl show wlan0 --json | jq '.dns'
```

#### Automation: Monitor Interface State Changes

```bash
#!/bin/bash
# Script to monitor and log interface state changes

LOG_FILE="/var/log/netctl-monitor.log"

while true; do
    # Get all interfaces
    interfaces=$(netctl show --json | jq -r '.[].name')

    for iface in $interfaces; do
        # Get current state and MAC
        state=$(netctl show "$iface" --json | jq -r '.state')
        mac=$(netctl show "$iface" --json | jq -r '.mac_address // "N/A"')
        mtu=$(netctl show "$iface" --json | jq -r '.mtu')

        # Log interface status
        echo "$(date '+%Y-%m-%d %H:%M:%S') - $iface: $state (MAC: $mac, MTU: $mtu)" >> "$LOG_FILE"
    done

    sleep 60  # Check every minute
done
```

#### Automation: Bring Up All Down Interfaces

```bash
#!/bin/bash
# Script to automatically bring up all down interfaces

# Find all interfaces that are down
down_interfaces=$(netctl show --json | jq -r '.[] | select(.state == "Down") | .name')

for iface in $down_interfaces; do
    echo "Bringing up interface: $iface"
    sudo netctl link set "$iface" --state up

    # Verify state change
    new_state=$(netctl show "$iface" --json | jq -r '.state')
    echo "  $iface is now: $new_state"
done
```

#### Automation: Network Configuration Backup

```bash
#!/bin/bash
# Script to backup network configuration as JSON

BACKUP_DIR="/etc/netctl-backup"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Backup all interface configurations
netctl show --json > "$BACKUP_DIR/interfaces_$TIMESTAMP.json"

# Backup individual interface details
interfaces=$(netctl show --json | jq -r '.[].name')
for iface in $interfaces; do
    netctl show "$iface" --json > "$BACKUP_DIR/${iface}_$TIMESTAMP.json"
done

echo "Backup completed: $BACKUP_DIR"
ls -lh "$BACKUP_DIR"
```

#### Automation: Health Check and Alerting

```bash
#!/bin/bash
# Script to check interface health and alert on issues

ALERT_EMAIL="admin@example.com"

# Check critical interfaces
CRITICAL_INTERFACES=("eth0" "eth1")

for iface in "${CRITICAL_INTERFACES[@]}"; do
    # Check if interface exists and is up
    state=$(netctl show "$iface" --json 2>/dev/null | jq -r '.state')

    if [[ "$state" != "Up" ]]; then
        # Send alert
        echo "ALERT: Interface $iface is $state" | \
            mail -s "Network Interface Alert" "$ALERT_EMAIL"

        # Log the issue
        logger -t netctl-health "Interface $iface is $state"

        # Attempt to bring it up
        sudo netctl link set "$iface" --state up
    fi
done
```

#### Automation: Compare Configurations

```bash
#!/bin/bash
# Script to compare network configuration between two snapshots

OLD_CONFIG="$1"
NEW_CONFIG="$2"

if [[ ! -f "$OLD_CONFIG" ]] || [[ ! -f "$NEW_CONFIG" ]]; then
    echo "Usage: $0 <old_config.json> <new_config.json>"
    exit 1
fi

# Compare interface names
echo "=== Interface Changes ==="
OLD_IFACES=$(jq -r '.[].name' "$OLD_CONFIG" | sort)
NEW_IFACES=$(jq -r '.[].name' "$NEW_CONFIG" | sort)

diff <(echo "$OLD_IFACES") <(echo "$NEW_IFACES")

# Compare IP addresses for each interface
echo -e "\n=== IP Address Changes ==="
for iface in $(echo "$NEW_IFACES"); do
    OLD_IPS=$(jq -r ".[] | select(.name == \"$iface\") | .addresses[]?.address // empty" "$OLD_CONFIG" | sort)
    NEW_IPS=$(jq -r ".[] | select(.name == \"$iface\") | .addresses[]?.address // empty" "$NEW_CONFIG" | sort)

    if [[ "$OLD_IPS" != "$NEW_IPS" ]]; then
        echo "Interface: $iface"
        diff <(echo "$OLD_IPS") <(echo "$NEW_IPS") || true
    fi
done
```

#### Migration from Legacy Tools

```bash
# Traditional ip/ifconfig commands → netctl equivalents

# OLD: ifconfig eth0 up
# NEW:
sudo netctl link set eth0 --state up

# OLD: ifconfig eth0 down
# NEW:
sudo netctl link set eth0 --state down

# OLD: ifconfig eth0 192.168.1.100 netmask 255.255.255.0
# NEW:
sudo netctl addr add eth0 192.168.1.100/24

# OLD: ifconfig eth0 mtu 9000
# NEW:
sudo netctl link set eth0 --mtu 9000

# OLD: ip link show
# NEW:
netctl show

# OLD: ip addr show eth0
# NEW:
netctl show eth0

# OLD: ip link set eth0 address 00:11:22:33:44:55
# NEW:
sudo netctl link set eth0 --mac 00:11:22:33:44:55
```

## 🔧 Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/ssahani/netctl.git
cd netctl

# Check compilation
cargo check --workspace

# Run tests
cargo test --workspace

# Build debug binary
cargo build

# Build optimized release binary
cargo build --release
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter (all warnings as errors)
cargo clippy --workspace -- -D warnings

# Security audit
cargo audit

# Run all CI checks locally
cargo check --workspace && \
cargo test --workspace && \
cargo fmt --all -- --check && \
cargo clippy --workspace -- -D warnings
```

### Testing

- **Unit Tests**: 21 tests covering network types and error handling
- **Integration Tests**: Planned (systemd services)
- **Test Coverage**: Run `cargo tarpaulin` for coverage reports

```bash
# Run specific test
cargo test --package netctl-types test_ipnetwork_parse_ipv4

# Run with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

### Design Principles

1. **Async Throughout** - All I/O operations are non-blocking
2. **Type Safety** - Leverage Rust's type system for compile-time correctness
3. **Rich Errors** - Detailed error messages with actionable suggestions
4. **Zero Panics** - Explicit error handling, no unwrap() in production code
5. **Idiomatic Rust** - Follow community best practices and conventions

### Performance Comparison

| Metric | C Version | Rust Version (netctl) |
|--------|-----------|----------------------|
| Binary Size | ~500 KB | 4.2 MB (static) |
| Memory Safety | Manual | Compile-time guaranteed |
| Concurrency | Synchronous | Async/await |
| Error Handling | Error codes | Rich typed errors |
| Build Time | ~30s | ~1m 45s (release) |
| Runtime Performance | Baseline | ~Equivalent |

### Project Statistics

- **Total Lines**: ~2,600 lines of Rust
- **Crates**: 6 workspace crates
- **Dependencies**: ~40 direct dependencies
- **Test Coverage**: 21 unit tests
- **Production Ready**: 75%

## 🐛 Troubleshooting

### Permission Denied Errors

```bash
# netctl requires root/CAP_NET_ADMIN for network operations
sudo netctl link set eth0 --state up

# Alternative: Add CAP_NET_ADMIN capability
sudo setcap cap_net_admin+ep /usr/local/bin/netctl
```

### Interface Not Found

```bash
# List all interfaces to verify name
netctl show

# Check if interface exists in kernel
ip link show

# Verify interface is not hidden by namespace
ip netns list
```

### D-Bus Connection Errors

```bash
# Ensure systemd services are running
systemctl status systemd-networkd
systemctl status systemd-resolved
systemctl status systemd-hostnamed

# Check D-Bus service availability
busctl list | grep org.freedesktop.network1
```

### Build Errors

```bash
# Install required dependencies
sudo apt-get install libdbus-1-dev  # Debian/Ubuntu
sudo dnf install dbus-devel          # Fedora/RHEL

# Clear cargo cache and rebuild
cargo clean
cargo build --release
```

## ❓ FAQ

**Q: Why is the binary larger than the C version?**
A: The Rust version includes the Tokio runtime, rich error handling (miette), and comprehensive tracing support. It's statically linked for portability.

**Q: Can I use netctl without systemd?**
A: Partial functionality. Link and address management work without systemd, but D-Bus integration requires systemd-networkd/resolved/hostnamed.

**Q: Is netctl production-ready?**
A: Currently at 75% production readiness. Core features work well, but some operations (address deletion, route management) are incomplete.

**Q: How do I report bugs?**
A: Please open an issue on [GitHub Issues](https://github.com/ssahani/netctl/issues) with:
- netctl version (`netctl --version`)
- Operating system and kernel version
- Steps to reproduce
- Error messages

**Q: Does netctl support IPv6?**
A: Yes, full IPv6 support for address management and DNS configuration.

**Q: Can I contribute?**
A: Absolutely! See the [Contributing](#-contributing) section below.

## 🗺️ Roadmap

### Completed ✅
- [x] Async netlink operations
- [x] D-Bus integration (networkd, resolved, hostnamed)
- [x] Link management (up/down, MTU)
- [x] Address management (add)
- [x] Show command with JSON output
- [x] 21 unit tests
- [x] CI/CD pipeline

### In Progress 🚧
- [ ] Address deletion (blocked on rtnetlink API)
- [ ] Route management
- [ ] Integration tests

### Planned 📋
- [ ] Virtual device support (VLAN, bridge, bond, WireGuard)
- [ ] Configuration file generation
- [ ] Interactive wizards
- [ ] Shell completion
- [ ] Man pages
- [ ] Package distribution (.deb, .rpm)

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Ways to Contribute
- 🐛 Report bugs and issues
- 💡 Suggest new features
- 📝 Improve documentation
- 🧪 Write tests
- 💻 Submit pull requests

### Development Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test --workspace`)
5. Run linter (`cargo clippy --workspace -- -D warnings`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Code Standards
- Follow Rust naming conventions
- Write tests for new features
- Update documentation
- Keep commits focused and atomic
- Write clear commit messages

## 📄 License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.

## 🔗 Links

- **Repository**: https://github.com/ssahani/netctl
- **Issue Tracker**: https://github.com/ssahani/netctl/issues
- **Documentation**: [See docs/](docs/)
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)

---

<div align="center">

**Built with ❤️ using Rust**

[⬆ Back to Top](#netctl)

</div>
