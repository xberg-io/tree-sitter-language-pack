```rust title="Rust"
use tree_sitter_language_pack::{ProcessConfig, process};

fn main() -> anyhow::Result<()> {
    let config = ProcessConfig::new("rust").all();
    let result = process("fn main() { println!(\"hello\"); }", &config)?;

    println!("Language: {}", result.language);
    println!("Functions: {}", result.structure.len());
    Ok(())
}
```
