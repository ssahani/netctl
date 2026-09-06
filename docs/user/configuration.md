# Configuration

netctl's config surface is the file you pass to `apply` — a YAML or TOML description of links, addresses, routes, DNS, and hostname — plus the systemd-networkd/resolved state it integrates with on the host.

## Config file format

```bash
sudo netctl apply config/network-config.example.yaml
```

The example file in the repo (`config/network-config.example.yaml`) is the starting point — copy and edit it for your host's links, addresses, routes, DNS, and hostname. TOML is also accepted.

## Safety around `apply`

- `apply` supports a dry-run mode so you can preview the diff before it touches the running network state.
- Every apply is backed up automatically, so you can restore or diff against the previous configuration if a change goes wrong.
- Run `netctl doctor` to check the current state for problems before or after applying.

## Watching and scripting

netctl supports JSON output and a watch mode, plus shell completions — useful for wiring `netctl show`/`netctl apply` into GitOps or config-management pipelines instead of hand-running commands.

## Troubleshooting

- **`apply` reports a conflict with systemd-networkd** — netctl integrates with systemd-networkd/resolved rather than owning the link exclusively; check for a competing `.network` file.
- **Unsure what a config change will do** — use the dry-run preview first, and keep the automatic backup so you can restore if the applied state isn't what you expected.

## Next steps

- [Getting started](getting-started.md)
- [Common workflows](workflows.md)
- [Admin basics](admin-basics.md)

## Operate from the console (UX)

1. Open this route from the nav or command palette and wait for live API data.
2. Use filters/search when present; drill into a row for detail.
3. For mutating actions: confirm role gates and impact before applying.
4. **Empty / fail:** Check service health, auth, and that required CRDs/backends for this domain are installed.
5. **Success:** Live data loads; created/updated objects appear without error toasts.

