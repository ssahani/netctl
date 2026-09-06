# Common workflows

| Workflow | Steps |
|----------|-------|
| Lab install | Clone → build → install binary → enable systemd unit (if applicable) |
| Validate | Run status/health command → check logs → confirm metrics or API |
| Upgrade | Stop service → replace binary → migrate config if release notes require → start service |

For netctl-specific examples, see the upstream `docs/` tree and tutorials on GitHub.

## Operate from the console (UX)

1. Open this route from the nav or command palette and wait for live API data.
2. Use filters/search when present; drill into a row for detail.
3. For mutating actions: confirm role gates and impact before applying.
4. **Empty / fail:** Check service health, auth, and that required CRDs/backends for this domain are installed.
5. **Success:** Live data loads; created/updated objects appear without error toasts.

