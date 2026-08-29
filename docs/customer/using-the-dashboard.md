# Using netctl (CLI)

netctl is a Linux **CLI** (optional TUI) for host networking — no first-party web console.

## Essentials

```bash
sudo netctl show
sudo netctl link set eth0 state up
sudo netctl apply config/network-config.example.yaml
sudo netctl doctor
```

## Where to go next

| Job | Doc |
|-----|-----|
| First install | [Getting Started](getting-started.md) |
| Config format | [Configuration](configuration.md) |
| Day-2 jobs | [Workflows](workflows.md) |
| Host / packaging | [Admin Basics](admin-basics.md) |
| Topic index | [PAGE_INDEX.md](PAGE_INDEX.md) |
