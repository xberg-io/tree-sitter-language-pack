# ts-pack-cli

CLI proxy for [`ts-pack`](https://github.com/xberg-io/tree-sitter-language-pack). Installing
this package provides a `ts-pack` command. Platform wheels bundle the native binary; the
source distribution downloads the binary of the same release version from GitHub releases
and verifies it against that release's `SHA256SUMS` before running it.

The package version is the `ts-pack` release version, so `ts-pack-cli==X.Y.Z` always runs
`ts-pack` `X.Y.Z`.
