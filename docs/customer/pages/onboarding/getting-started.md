# Getting Started

## Purpose

Install netctl and apply your first network config.

## How to get there

- Topic id: `getting-started`
- Section: **Onboarding → Getting Started**

## Guide

netctl is an async Linux network CLI for links, addresses, routes, DNS, and hostname — plus declarative YAML/TOML `apply` with dry-run and `doctor` diagnostics. This page gets you from a fresh checkout to your first applied config.

## Prerequisites

- A Linux host with `sudo`/root access — link, address, and route changes require privileges.
- netctl built and installed: clone [github.com/zyvorai/netctl](https://github.com/zyvorai/netctl) and follow the build/install steps in the upstream README (source of truth for release artifacts and packaging).
- systemd-networkd and systemd-resolved present if you want netctl's integration with them; Netlink is used directly for links/addresses/routes either way.

## 1. Check current state

```bash
sudo netctl show
```

## 2. Make a change

```bash
sudo netctl link set eth0 state up
```

## 3. Apply a declarative config

```bash
sudo netctl apply config/network-config.example.yaml
```

`apply` reads YAML or TOML, backs up the prior state automatically, and supports diff/restore — see [Configuration](configuration.md) for the file format.

## Troubleshooting

- **`apply`/`link`/`addr` commands fail with a permission error** — rerun with `sudo`; netctl needs root to touch links, addresses, and routes.
- **Changes don't take effect on a systemd-networkd host** — confirm `systemd-networkd`/`systemd-resolved` are active; netctl integrates with them rather than replacing them.

## Next steps

- [Configuration](configuration.md)
- [Common workflows](workflows.md)
- [Admin basics](admin-basics.md)

## Operate from the console (UX)

1. Open this route from the nav or command palette and wait for live API data.
2. Use filters/search when present; drill into a row for detail.
3. For mutating actions: confirm role gates and impact before applying.
4. **Empty / fail:** Check service health, auth, and that required CRDs/backends for this domain are installed.
5. **Success:** Live data loads; created/updated objects appear without error toasts.

## Related pages

- [Getting Started](../../getting-started.md)
- [Page index](../../PAGE_INDEX.md)
