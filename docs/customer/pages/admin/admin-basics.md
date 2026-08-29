# Admin Basics

## Purpose

Packaging, privileges, systemd integration, and ops.

## How to get there

- Topic id: `admin-basics`
- Section: **Admin → Admin Basics**

## Guide

| Topic | Guidance |
|-------|----------|
| **Service** | Use packaged systemd units when available; run unprivileged where the upstream docs allow |
| **Logs** | `journalctl -u <unit> -f` or container logs per deploy method |
| **Security** | Follow upstream `SECURITY.md`; restrict config file permissions |
| **Support** | [GitHub issues](https://github.com/zyvorai/netctl/issues) · [Contact Zyvor](/contact) for Enterprise |

See also [Getting started](getting-started.md).

## Operate from the console (UX)

1. Open this route from the nav or command palette and wait for live API data.
2. Use filters/search when present; drill into a row for detail.
3. For mutating actions: confirm role gates and impact before applying.
4. **Empty / fail:** Check service health, auth, and that required CRDs/backends for this domain are installed.
5. **Success:** Live data loads; created/updated objects appear without error toasts.

## Related pages

- [Getting Started](../../getting-started.md)
- [Page index](../../PAGE_INDEX.md)
