```rust title="Rust"
use tree_sitter_language_pack::{ProcessConfig, process};

fn main() -> anyhow::Result<()> {
    let config = ProcessConfig::new("python")
        .all()
        .with_chunking(1000);

    let result = process("def hello(): pass\ndef world(): pass", &config)?;

    for item in &result.structure {
        // item.kind implements Debug (not Display); item.name is Option<String>
        println!("{:?}: {}", item.kind, item.name.as_deref().unwrap_or("<unnamed>"));
    }
    for chunk in &result.chunks {
        println!("chunk: lines {}-{}", chunk.start_line, chunk.end_line);
    }
    Ok(())
}
```
