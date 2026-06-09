---
priority: high
---

# Language Registry Management

- Each language entry must specify: grammar repo, commit/tag, optional external scanner flag.
- Maintain a central registry file listing all supported languages and their metadata.
- Version bumps require running the full test suite against real source files.
- Document any language-specific quirks or known parsing limitations.
- Track upstream grammar changes and automate update detection.
