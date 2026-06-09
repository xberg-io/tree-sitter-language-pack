```rust title="Rust"
// Requires feature = "download" (enabled by default).
use std::path::PathBuf;
use tree_sitter_language_pack::{PackConfig, download, downloaded_languages, init};

fn main() -> anyhow::Result<()> {
    // Pre-download specific languages; returns count of ensured languages.
    let _count = download(&["python", "javascript", "rust"])?;

    // Or initialize with config (cache_dir is PathBuf, not String).
    let config = PackConfig {
        languages: Some(vec!["python".into(), "go".into()]),
        cache_dir: Some(PathBuf::from("/tmp/parsers")),
        groups: None,
    };
    init(&config)?;

    println!("{:?}", downloaded_languages());
    Ok(())
}
```
