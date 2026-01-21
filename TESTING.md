# netctl - Testing Guide

## Quick Test Commands

### Build & Test

```bash
# Check all crates compile
cargo check --workspace

# Build debug binary
cargo build

# Build release binary
cargo build --release

# Run all tests
cargo test --workspace

# Format code
cargo fmt --all

# Lint code
cargo clippy --workspace

# Security audit
cargo audit
```

### CLI Testing

```bash
# Show help
cargo run -- --help

# Show version
cargo run -- --version

# Link commands
cargo run -- link --help
cargo run -- link set --help

# Address commands
cargo run -- addr --help
cargo run -- addr add --help
```

### Example Commands (requires root/CAP_NET_ADMIN)

```bash
# Show all interfaces
sudo ./target/release/netctl show

# Show specific interface
sudo ./target/release/netctl show eth0

# Show in JSON format
sudo ./target/release/netctl show --json
sudo ./target/release/netctl show eth0 --json

# Set link up
sudo ./target/release/netctl link set eth0 --state up

# Set link down
sudo ./target/release/netctl link set eth0 --state down

# Set MTU
sudo ./target/release/netctl link set eth0 --mtu 9000

# Add IP address
sudo ./target/release/netctl addr add eth0 192.168.1.10/24
```

## Expected Outputs

### ✅ Show Command - Table Format

```bash
$ sudo ./target/release/netctl show
INDEX NAME            STATE    MTU      MAC ADDRESS
------------------------------------------------------------
1     lo              UP       65536    -
2     eth0            UP       1500     aa:bb:cc:dd:ee:ff
3     wlan0           DOWN     1500     11:22:33:44:55:66

Total: 3 interface(s)
```

### ✅ Show Command - Detail Format

```bash
$ sudo ./target/release/netctl show eth0
Interface: eth0
  Index: 2
  State: UP
  MTU: 1500
  MAC Address: aa:bb:cc:dd:ee:ff
```

### ✅ Show Command - JSON Format

```bash
$ sudo ./target/release/netctl show eth0 --json
{
  "index": 2,
  "name": "eth0",
  "state": "Up",
  "mtu": 1500,
  "mac_address": [170, 187, 204, 221, 238, 255],
  "addresses": []
}
```

### ✅ Link Management Success

```bash
$ sudo ./target/release/netctl link set eth0 --state up
✓ Interface eth0 is now up
```

### ❌ Error Case - Interface Not Found

```bash
$ sudo ./target/release/netctl link set nonexistent --state up
Error: netctl::not_found

  × interface 'nonexistent' not found
  help: Use 'netctl show' to list interfaces
```

### ❌ Error Case - Invalid CIDR

```bash
$ sudo ./target/release/netctl addr add eth0 192.168.1.10
Error: invalid CIDR: 192.168.1.10
```

### ❌ Error Case - Permission Denied

```bash
$ ./target/release/netctl link set eth0 --state up
Error: netlink error: Operation not permitted
```

## Test Coverage

### Unit Tests ✅

**netctl-types** (21 tests passing):
- [x] IpNetwork parsing (IPv4/IPv6)
- [x] IpNetwork validation and errors
- [x] IpNetwork display and roundtrip
- [x] MacAddress parsing and formatting
- [x] MacAddress validation
- [x] Error types and messages
- [x] Error helper functions

### Currently Working ✅

**CLI Commands:**
- [x] Show all interfaces (table format)
- [x] Show specific interface (detail format)
- [x] JSON output for show commands
- [x] Link up/down
- [x] MTU configuration
- [x] Add IP address
- [x] CLI help system
- [x] Error handling with miette
- [x] Logging/tracing

**D-Bus Integration:**
- [x] systemd-networkd reload
- [x] systemd-networkd reconfigure_link
- [x] systemd-resolved DNS configuration
- [x] systemd-resolved domain configuration
- [x] systemd-hostnamed hostname management

**Netlink Operations:**
- [x] List all network interfaces
- [x] Get interface by name
- [x] Get detailed interface information
- [x] Set link up/down
- [x] Set MTU
- [x] Add IP address

### Not Yet Implemented ⚠️

- [ ] Delete IP address (blocked on rtnetlink API)
- [ ] Route management
- [ ] Virtual devices (VLAN, bridge, bond)
- [ ] Config file generation
- [ ] Integration tests with systemd
- [ ] Netlink operation unit tests (mocking)

## Performance Testing

### Build Times

```bash
# Debug build
$ time cargo build
real    0m8.234s

# Release build
$ time cargo build --release
real    0m45.321s
```

### Binary Size

```bash
$ ls -lh target/release/netctl
-rwxr-xr-x  2 user user 4.1M Jan 21 08:20 netctl
```

### Memory Usage

```bash
# Check with valgrind (debug build)
$ valgrind --leak-check=full ./target/debug/netctl --help

# Profile with heaptrack
$ heaptrack ./target/release/netctl link set eth0 --state up
```

## CI/CD Testing

### GitHub Actions

The project includes a CI workflow that runs:

```yaml
jobs:
  - check: cargo check --workspace
  - test: cargo test --workspace
  - fmt: cargo fmt --all -- --check
  - clippy: cargo clippy --workspace -- -D warnings
  - build-musl: cross build for x86_64 and aarch64
```

### Local CI Simulation

```bash
# Run all CI checks locally
./scripts/ci-local.sh  # (to be created)

# Or manually:
cargo check --workspace && \
cargo test --workspace && \
cargo fmt --all -- --check && \
cargo clippy --workspace -- -D warnings
```

## Integration Testing

### With Real Network Interfaces

```bash
# Create test interface
sudo ip link add test0 type dummy

# Test with netctl
sudo ./target/release/netctl link set test0 --state up
sudo ./target/release/netctl addr add test0 10.0.0.1/24

# Verify
ip addr show test0

# Cleanup
sudo ip link del test0
```

### With Docker

```bash
# Build container
docker build -t netctl:latest .

# Run (requires --net=host and --cap-add=NET_ADMIN)
docker run --rm --net=host --cap-add=NET_ADMIN netctl:latest show
```

## Debugging

### Enable Verbose Logging

```bash
# Set RUST_LOG environment variable
RUST_LOG=debug cargo run -- link set eth0 --state up

# Trace level for maximum detail
RUST_LOG=trace cargo run -- link set eth0 --state up
```

### GDB Debugging

```bash
# Build with debug symbols
cargo build

# Run with GDB
gdb --args ./target/debug/netctl link set eth0 --state up
```

### Tracing Output

```bash
# Pretty format (default)
cargo run -- link set eth0 --state up

# JSON format (for log aggregation)
RUST_LOG_FORMAT=json cargo run -- link set eth0 --state up
```

## Benchmarking

### Cargo Bench (to be added)

```bash
cargo bench
```

### Flamegraph Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
sudo cargo flamegraph -- link set eth0 --state up
```

## Test Checklist

Before submitting PRs, ensure:

- [ ] `cargo check --workspace` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo audit` shows no vulnerabilities
- [ ] Manual CLI testing completed
- [ ] Documentation updated
- [ ] CHANGELOG updated

## Known Test Limitations

1. **No Netlink Mocking** - Tests requiring actual netlink operations need CAP_NET_ADMIN
2. **No D-Bus Mocking** - D-Bus operations need running systemd services
3. **Platform-Specific** - Tests only run on Linux
4. **Root Required** - Many operations require elevated privileges

## Future Test Improvements

1. Add unit tests for all modules
2. Mock netlink operations
3. Mock D-Bus operations
4. Add property-based tests with `proptest`
5. Add benchmark suite
6. Add fuzzing tests
7. Add coverage reporting
