# netctl - Quick Start Guide

## Installation

### From Source

```bash
# Clone repository
git clone https://github.com/vmware/netctl
cd netctl

# Build release binary
cargo build --release

# Install (optional)
sudo cp target/release/netctl /usr/local/bin/

# Verify installation
netctl --version
```

### Using Docker

```bash
# Build image
docker build -t netctl:latest .

# Run
docker run --rm netctl:latest --help
```

### Cross-Compile for musl

```bash
# Install cross
cargo install cross

# Build static binary
cross build --release --target x86_64-unknown-linux-musl

# Binary is at: target/x86_64-unknown-linux-musl/release/netctl
```

## Basic Usage

### Show Interfaces

```bash
# Show all interfaces
netctl show

# Show specific interface
netctl show eth0
```

### Manage Links

```bash
# Bring link up
sudo netctl link set eth0 --state up

# Bring link down
sudo netctl link set eth0 --state down

# Set MTU
sudo netctl link set eth0 --mtu 9000
```

### Manage IP Addresses

```bash
# Add IPv4 address
sudo netctl addr add eth0 192.168.1.10/24

# Add IPv6 address
sudo netctl addr add eth0 2001:db8::1/64
```

## Examples

### Configure Static IP

```bash
# Set interface up
sudo netctl link set eth0 --state up

# Add IP address
sudo netctl addr add eth0 192.168.1.100/24

# Set MTU
sudo netctl link set eth0 --mtu 1500
```

### Configure Interface with Multiple IPs

```bash
sudo netctl link set eth0 --state up
sudo netctl addr add eth0 192.168.1.10/24
sudo netctl addr add eth0 10.0.0.10/8
```

## Getting Help

```bash
# General help
netctl --help

# Command-specific help
netctl link --help
netctl link set --help
netctl addr --help
netctl addr add --help
```

## Troubleshooting

### Permission Denied

If you get permission errors, run with sudo:

```bash
sudo netctl link set eth0 --state up
```

### Interface Not Found

List available interfaces first:

```bash
netctl show
# or
ip link show
```

### Enable Debug Logging

```bash
RUST_LOG=debug sudo netctl link set eth0 --state up
```

## What's Next?

- Read the full [README.md](README.md)
- Check the [REVIEW.md](REVIEW.md) for status
- See [TESTING.md](TESTING.md) for testing guide
- Contribute at [GitHub](https://github.com/vmware/netctl)
