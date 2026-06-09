---
priority: medium
---

# Version Management

- Use semantic versioning for the language pack crate.
- Breaking grammar updates (ABI changes, removed languages) require major version bumps.
- New language additions are minor version bumps.
- Grammar bug fixes and updates are patch version bumps.
- Synchronize version across Cargo.toml, package.json, pyproject.toml, and other binding manifests.
