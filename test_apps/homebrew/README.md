# homebrew test_app

Exercises the configured Homebrew formulae from tap `kreuzberg-dev/homebrew-tap` at version `1.9.0`.

| Formula     | Purpose                                                    |
| ----------- | ---------------------------------------------------------- |
| `ts-pack`   | CLI binary: tree-sitter language pack parser manager       |
| `libts-pack` | Shared library: C FFI for embedding in other languages    |

## Running

```bash
bash run_tests.sh
```

## What it tests

1. `brew bundle install` succeeds (tap + both formulae install without error).
2. `ts-pack --version` output contains `1.9.0`.
3. `ts-pack --help` produces non-empty output.
4. `ffi_smoke.c` compiles against `libts-pack` (via `pkg-config`) and the
   compiled binary calls `ts_pack_available_languages` successfully.
