# pushkind-common

Shared utilities and models for Pushkind services.

This repository also exposes the `@pushkind/frontend-shell` npm package from
the repo root. It now contains the first extracted shared frontend primitives
for the React-migrated services:

- `UserMenuDropdown`
- `useServiceShell`
- `useNoAccessPageData`
- `NoAccessCard`
- shared frontend shell and no-access types

The package is source-first TypeScript intended for Vite-based service
frontends consuming it from GitHub.
