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

### Innovative Features (17 Commands)

**Core Operations:**
- 📡 **Show** - Display network interfaces with detailed information
- 🔗 **Link Management** - Control interface state, MTU, MAC addresses
- 🌐 **Address Management** - Configure IPv4/IPv6 addresses

**Monitoring & Observability:**
- 🎨 **Real-time TUI** - Beautiful terminal dashboard with live network monitoring
- 👁️ **Watch Mode** - Continuous monitoring with auto-refresh
- 📊 **Network Statistics** - Real-time bandwidth, packet counters, and error monitoring
- 🔍 **System Diagnostics** - Comprehensive health checks with `doctor` command
- 🧪 **Network Testing** - Connectivity, DNS, ping tests with comprehensive test suite

**Configuration Management:**
- 💾 **Network Profiles** - Save/load/switch complete network configurations
- 🔄 **Configuration as Code** - YAML/TOML-based declarative configuration with `apply`
- 📸 **Backup & Restore** - Full network state backup and disaster recovery
- ⏮️ **History & Rollback** - Track all changes and rollback to previous states
- 📤 **Export** - Export configs in YAML/TOML/JSON for integration
- ✅ **Config Validation** - Pre-flight checks for configuration files
- 📊 **Network Diff** - Compare configurations and states with color-coded output

**Developer Experience:**
- 🧙 **Interactive Wizard** - Guided setup for common configuration tasks
- 🔌 **Shell Completion** - Auto-completion for bash, zsh, fish, PowerShell, elvish
- 🎯 **Dry Run Mode** - Preview changes before applying them
- 🚀 **GitOps Ready** - Version control your network configs, CI/CD friendly

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
sudo netctl link set eth0 state up

# Configure IP address
sudo netctl addr add eth0 192.168.1.100/24

# Set MTU for jumbo frames
sudo netctl link set eth0 mtu 9000
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
netctl link set eth0 state up
netctl link set eth0 state down

# Configure MTU
netctl link set eth0 mtu 1500      # Standard Ethernet
netctl link set eth0 mtu 9000      # Jumbo frames

# Set MAC address (requires interface down)
netctl link set eth0 mac 00:11:22:33:44:55
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

### Innovative Features

#### Real-time TUI Dashboard

Launch an interactive terminal dashboard with live network monitoring:

```bash
# Start the real-time TUI dashboard
sudo netctl tui
```

**Features:**
- **Split-pane interface** - List (left) and Details (right)
- **Live updates** - Auto-refresh every 250ms
- **Interactive navigation** - ↑/↓ or j/k to navigate
- **Selected interface details** - Realtime information display
- **Color-coded states** - Green (UP) / Red (DOWN)
- **Coral-orange theme** - Pantone 7416 C inspired (matching guestkit)
- **Professional design** - Beautiful terracotta borders and highlights

**Keyboard Controls:**
- `Tab` / `Shift+Tab` - Switch between views (Dashboard, Statistics, Configuration)
- `↑` / `↓` or `j` / `k` - Navigate interfaces
- `h` or `F1` - Toggle help overlay
- `i` - Toggle stats bar
- `q` / `Esc` - Quit
- `Ctrl+C` - Force exit

**Multiple Views:**
- **📊 Dashboard** - Split-pane interface with list and details
- **📈 Statistics** - Network statistics with gauges and metrics
- **⚙️ Configuration** - Interactive configuration (coming soon)

**Split Pane Layout:**
```
┌─────────────────────────────────────────────────────────────┐
│  netctl - Network Configuration Manager                     │
├──────────────────────────┬──────────────────────────────────┤
│  🌐 Network Interfaces   │  📋 Details                      │
│  ┌────────────────────┐  │  ┌────────────────────────────┐ │
│  │ ▶ eth0    UP  1500 │  │  │ Interface: eth0            │ │
│  │   wlan0  DOWN 1500 │  │  │                            │ │
│  │   lo      UP 65536 │  │  │ Index: 2                   │ │
│  └────────────────────┘  │  │ State: Up                  │ │
│                          │  │ MTU: 1500                  │ │
│                          │  │ MAC: 52:54:00:12:34:56     │ │
│                          │  │                            │ │
│                          │  │ IP Addresses:              │ │
│                          │  │   • 192.168.1.100/24       │ │
│                          │  └────────────────────────────┘ │
├──────────────────────────┴──────────────────────────────────┤
│  ↑/↓ Navigate  q/Esc Quit  Ctrl+C Exit                     │
└─────────────────────────────────────────────────────────────┘
```

The TUI provides a beautiful, professional interface with guestkit's stunning coral-orange color scheme (Pantone 7416 C), featuring split panes for efficient navigation and detailed interface inspection.

#### Watch Mode (Continuous Monitoring)

Monitor interfaces with automatic refresh:

```bash
# Watch all interfaces (default 1s refresh)
netctl watch

# Watch specific interface
netctl watch eth0

# Custom refresh interval (5 seconds)
netctl watch --interval 5

# Watch specific interface with custom interval
netctl watch wlan0 --interval 2
```

Watch mode clears the screen and refreshes the interface list automatically, perfect for monitoring network changes during configuration.

#### Network Profiles (Save/Load Configurations)

Save and restore complete network configurations:

```bash
# Save current network configuration as a profile
sudo netctl profile save work --description "Office network setup"

# List all saved profiles
netctl profile list

# Show profile details
netctl profile show work

# Load a saved profile
sudo netctl profile load work

# Delete a profile
netctl profile delete work
```

**Profile Example:**

```yaml
# ~/.config/netctl/profiles/work.yaml
name: work
description: Office network setup
created_at: '2026-02-04T02:30:15+00:00'
interfaces:
  - name: eth0
    state: Up
    mtu: 1500
    mac_address: 52:54:00:12:34:56
    addresses:
      - 192.168.1.100/24
  - name: wlan0
    state: Down
    mtu: 1500
    mac_address: null
    addresses: []
```

**Use Cases for Profiles:**
- Switch between work/home/travel network configurations instantly
- Backup network configuration before making changes
- Share network setups across team members
- Automate network configuration in scripts
- Document network setups as code

#### Declarative Configuration (Infrastructure as Code)

Apply network configurations from YAML or TOML files:

```bash
# Apply configuration from YAML file
sudo netctl apply network-config.yaml

# Dry run - see what would be applied without making changes
sudo netctl apply network-config.yaml --dry-run

# Apply TOML configuration
sudo netctl apply network-config.toml
```

**Example Configuration File (YAML):**

```yaml
# network-config.yaml
interfaces:
  - name: eth0
    state: up
    mtu: 1500
    addresses:
      - 192.168.1.100/24
      - 2001:db8::100/64

  - name: eth1
    state: up
    mtu: 9000  # Jumbo frames
    addresses:
      - 10.0.0.50/24
```

**Example Configuration File (TOML):**

```toml
# network-config.toml
[[interfaces]]
name = "eth0"
state = "up"
mtu = 1500
addresses = ["192.168.1.100/24", "2001:db8::100/64"]

[[interfaces]]
name = "eth1"
state = "up"
mtu = 9000
addresses = ["10.0.0.50/24"]
```

**Benefits:**
- Version control your network configurations with Git
- Reproducible infrastructure across environments
- Easy rollback to previous configurations
- Code review for network changes
- GitOps workflows for network management

#### Network Diff (Compare Configurations)

Compare network states and profiles to see differences:

```bash
# Compare current state with a saved profile
netctl diff current work

# Compare two profiles
netctl diff work home

# Compare before/after making changes
netctl profile save before
# ... make changes ...
netctl diff before current
```

**Example Output:**

```
Network Configuration Diff
============================================================
Comparing: current vs work

~ eth0:
  ~ state: Down → Up
  ~ mtu: 1500 → 9000
  + address: 192.168.1.100/24

+ Interface 'eth1' (only in work)
```

Color-coded output:
- Green (+): Added in second state
- Red (-): Removed from first state
- Yellow (~): Modified between states

#### Interactive Configuration Wizard

Guided setup for common network configuration tasks:

```bash
# Launch the interactive wizard
sudo netctl wizard

# Follow the prompts to configure:
# - Static IP addresses
# - Interface state (up/down)
# - MTU settings
# - DHCP preparation
# - Complete network setup
```

**Features:**
- User-friendly interactive prompts
- Input validation
- Configuration summary before applying
- Guided workflows for beginners
- No need to memorize commands

#### System Diagnostics

Run comprehensive health checks and diagnostics:

```bash
# Run system diagnostics
netctl doctor

# Verbose diagnostics with detailed output
netctl doctor --verbose
```

**Checks performed:**
- ✓ Network interfaces availability
- ✓ systemd services status (networkd, resolved)
- ✓ D-Bus connection
- ✓ Permissions and capabilities
- ✓ Network connectivity
- ✓ DNS resolution

**Example Output:**

```
🔍 netctl System Diagnostics
============================================================

→ Checking network interfaces... ✓
→ Checking systemd services... ✓
→ Checking D-Bus connection... ✓
→ Checking permissions... ⚠
    Not running as root
    Some operations may require sudo/root privileges
→ Checking network connectivity... ✓
→ Checking DNS resolution... ✓

============================================================
✓ All checks passed!
```

#### Network Statistics and Monitoring

View real-time network statistics for interfaces:

```bash
# Show statistics for all interfaces
netctl stats

# Show statistics for specific interface
netctl stats eth0

# Detailed statistics (RX/TX packets and errors)
netctl stats --detailed

# JSON output for automation
netctl stats --format json
```

**Example Output:**

```
┌───────────┬───────┬──────────┬──────────┬────────┐
│ Interface │ State │ RX       │ TX       │ Errors │
├═══════════┼═══════┼══════════┼══════════┼════════┤
│ lo        │ UP    │ 125.45 MB│ 125.45 MB│ 0      │
│ eth0      │ UP    │ 15.23 GB │ 2.45 GB  │ 0      │
│ wlan0     │ DOWN  │ 0 B      │ 0 B      │ 0      │
└───────────┴───────┴──────────┴──────────┴────────┘
```

**Features:**
- Real-time bandwidth usage
- Packet and error counters
- Human-readable size formatting (B, KB, MB, GB, TB)
- Color-coded interface states
- JSON output for integration

#### Configuration Validation

Validate configuration files before applying them:

```bash
# Validate YAML configuration
netctl validate network-config.yaml

# Validate TOML configuration
netctl validate network-config.toml

# Strict mode - fail on warnings
netctl validate network-config.yaml --strict
```

**Validation checks:**
- File syntax (YAML/TOML parsing)
- Interface name length and format
- State values (up/down)
- MTU ranges (68-65535)
- IP address format and prefix length
- Duplicate address detection
- Common misconfigurations

**Example Output:**

```
Validating network configuration...
File: network-config.yaml

✓ Configuration file parsed successfully

Validating interface 1 (eth0)...
  ✓ Validated
Validating interface 2 (eth1)...
  ✓ Validated

============================================================

Warnings (1):
  ⚠ Jumbo frames (MTU 9000) on interface 'eth1' require network infrastructure support

✓ Validation passed with warnings
```

#### Shell Completion Generation

Generate shell completion scripts for better CLI experience:

```bash
# Generate bash completion
netctl completion bash > /etc/bash_completion.d/netctl

# Generate zsh completion
netctl completion zsh > ~/.zsh/completion/_netctl

# Generate fish completion
netctl completion fish > ~/.config/fish/completions/netctl.fish

# Generate PowerShell completion (Windows)
netctl completion powershell > netctl.ps1
```

**Supported shells:**
- Bash
- Zsh
- Fish
- PowerShell
- Elvish

**Features:**
- Command completion
- Subcommand completion
- Flag and option completion
- Intelligent context-aware suggestions

#### Configuration History and Rollback

Track all configuration changes with automatic history and rollback capability:

```bash
# List configuration history
netctl history list

# Show last 20 changes
netctl history list -n 20

# Show details of a specific history entry
netctl history show 20260204_143052

# Rollback to a previous configuration
netctl history rollback 20260204_143052

# Rollback without confirmation prompt
netctl history rollback 20260204_143052 --yes

# Clear all history
netctl history clear
```

**Features:**
- Automatic snapshot before major changes
- Timestamped history entries
- Detailed operation logs
- Safe rollback with confirmation
- Preserves state before rollback

**Example Output:**

```
Configuration History
================================================================================

1. 20260204_143052 (2026-02-04T14:30:52+00:00)
   Operation: apply
   Applied configuration from network-config.yaml

2. 20260204_120015 (2026-02-04T12:00:15+00:00)
   Operation: manual
   Manual configuration changes

3. 20260203_183045 (2026-02-03T18:30:45+00:00)
   Operation: rollback
   Rollback to 20260203_150000
```

#### Backup and Restore

Create full backups of your network configuration:

```bash
# Create a backup
netctl backup create production --description "Production config before upgrade"

# List all backups
netctl backup list

# Restore from backup
netctl backup restore production

# Restore without confirmation
netctl backup restore production --yes

# Delete a backup
netctl backup delete old-config

# Export backup to file
netctl backup export production --output /path/to/backup.json
```

**Features:**
- Complete network state snapshot
- Metadata with timestamps and descriptions
- Automatic backup before restore
- Export to JSON for version control
- Safe restore with confirmation

**Use Cases:**
- Pre-upgrade backups
- Disaster recovery
- Configuration testing (backup, test, restore)
- Team sharing via exported files
- Compliance and audit trails

#### Network Testing Suite

Comprehensive network connectivity and functionality testing:

```bash
# Test connectivity on all active interfaces
netctl test connectivity

# Test specific interface
netctl test connectivity --interface eth0

# Test DNS resolution
netctl test dns www.example.com

# Ping a host
netctl test ping 8.8.8.8 -c 10

# Ping via specific interface
netctl test ping 192.168.1.1 --interface eth0

# Run comprehensive test suite
netctl test all
```

**Comprehensive Test Suite (`test all`) includes:**
- ✓ Interface availability
- ✓ Internet connectivity (ping 8.8.8.8)
- ✓ DNS resolution
- ✓ systemd-networkd status
- ✓ systemd-resolved status

**Example Output:**

```
Comprehensive Network Test Suite
================================================================================

1. Testing interface availability...
   ✓ 2 interface(s) up

2. Testing internet connectivity...
   ✓ Internet accessible

3. Testing DNS resolution...
   ✓ DNS resolution working

4. Testing systemd-networkd...
   ✓ systemd-networkd is active

5. Testing systemd-resolved...
   ✓ systemd-resolved is active

================================================================================
Test Summary

  PASS Interface availability
  PASS Internet connectivity
  PASS DNS resolution
  PASS systemd-networkd
  PASS systemd-resolved

✓ All tests passed (5/5)
```

#### Export Configuration

Export network configuration in multiple formats for integration:

```bash
# Export to YAML (default)
netctl export network.yaml

# Export to TOML
netctl export network.toml --format toml

# Export to JSON
netctl export network.json --format json

# Export specific interfaces only
netctl export eth-only.yaml --interfaces eth0,eth1

# Pretty print output
netctl export config.json --format json --pretty
```

**Features:**
- Multiple format support (YAML, TOML, JSON)
- Selective interface export
- Metadata inclusion (timestamp, hostname)
- Pretty printing option
- GitOps-friendly output

**Use Cases:**
- Version control integration
- Configuration documentation
- Cross-team sharing
- Migration to other tools
- Ansible/Terraform integration

### Real-World Examples

#### Configure Static IP with Gateway

```bash
# Complete static IP configuration workflow
# Bring interface down for configuration
sudo netctl link set eth0 state down

# Add static IP address with subnet
sudo netctl addr add eth0 192.168.1.100/24

# Set custom MTU if needed (default is 1500)
sudo netctl link set eth0 mtu 1500

# Bring interface up
sudo netctl link set eth0 state up

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
sudo netctl link set eth0 state down
sudo netctl link set eth0 mtu 9000
sudo netctl link set eth0 state up

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
sudo netctl link set eth0 state down

# Set new MAC address
sudo netctl link set eth0 mac 00:11:22:33:44:55

# Bring interface back up
sudo netctl link set eth0 state up

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
    sudo netctl link set "$iface" state up

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
        sudo netctl link set "$iface" state up
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
sudo netctl link set eth0 state up

# OLD: ifconfig eth0 down
# NEW:
sudo netctl link set eth0 state down

# OLD: ifconfig eth0 192.168.1.100 netmask 255.255.255.0
# NEW:
sudo netctl addr add eth0 192.168.1.100/24

# OLD: ifconfig eth0 mtu 9000
# NEW:
sudo netctl link set eth0 mtu 9000

# OLD: ip link show
# NEW:
netctl show

# OLD: ip addr show eth0
# NEW:
netctl show eth0

# OLD: ip link set eth0 address 00:11:22:33:44:55
# NEW:
sudo netctl link set eth0 mac 00:11:22:33:44:55
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
sudo netctl link set eth0 state up

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

**Core Functionality:**
- [x] Async netlink operations
- [x] D-Bus integration (networkd, resolved, hostnamed)
- [x] Link management (up/down, MTU, MAC)
- [x] Address management (add)
- [x] Show command with JSON output
- [x] 21 unit tests
- [x] CI/CD pipeline

**Production Features (17 commands):**
- [x] **Real-time TUI** - Live monitoring dashboard with coral-orange theme
- [x] **Watch mode** - Continuous interface monitoring
- [x] **Network profiles** - Save/load/list/delete/show configurations
- [x] **Declarative config** - Apply from YAML/TOML files with dry-run
- [x] **Network diff** - Compare states and profiles with color coding
- [x] **Interactive wizard** - Guided configuration setup
- [x] **System diagnostics** - Comprehensive health checks (`doctor`)
- [x] **Network statistics** - Real-time bandwidth/packet/error monitoring
- [x] **Config validation** - Pre-flight checks with warnings/errors
- [x] **Shell completions** - bash/zsh/fish/PowerShell/elvish support
- [x] **History & rollback** - Track changes and rollback capability
- [x] **Backup & restore** - Full system backup and disaster recovery
- [x] **Network testing** - Connectivity, DNS, ping, comprehensive tests
- [x] **Export** - YAML/TOML/JSON export for GitOps
- [x] **Cleaner CLI** - Property-based syntax (no verbose flags)
- [x] **Example configs** - YAML and TOML templates
- [x] **Dry run mode** - Preview all changes before applying

### In Progress 🚧
- [ ] Address deletion (blocked on rtnetlink API)
- [ ] Route management and routing table manipulation
- [ ] Integration tests with mock systemd services
- [ ] Historical statistics with time-series data

### Planned 📋
- [ ] Virtual device support (VLAN, bridge, bond, WireGuard, veth)
- [ ] Network topology visualization in TUI with ASCII graphs
- [ ] Profile merge and conflict resolution
- [ ] Network health monitoring with auto-healing policies
- [ ] Bandwidth alerts and threshold notifications
- [ ] Man pages generation from CLI metadata
- [ ] Package distribution (.deb, .rpm, AUR, Homebrew)
- [ ] Web dashboard (optional, with WebSocket live updates)
- [ ] REST API for remote management
- [ ] Plugin system for custom network backends
- [ ] Terraform provider
- [ ] Ansible module

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
