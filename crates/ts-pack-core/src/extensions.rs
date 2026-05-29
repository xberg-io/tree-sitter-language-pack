//! File extension to language name mapping.
//!
//! Mappings are auto-generated from `sources/language_definitions.json` by `build.rs`.
//! To add or modify extension mappings, edit that JSON file and rebuild.

use memchr::memchr;

/// Detect language name from a file extension (without leading dot).
///
/// Returns `None` for unrecognized extensions. The match is case-insensitive.
///
/// ```
/// use tree_sitter_language_pack::detect_language_from_extension;
/// assert_eq!(detect_language_from_extension("py"), Some("python"));
/// assert_eq!(detect_language_from_extension("RS"), Some("rust"));
/// assert_eq!(detect_language_from_extension("xyz"), None);
/// ```
#[inline]
pub fn detect_language_from_extension(ext: &str) -> Option<&'static str> {
    include!(concat!(env!("OUT_DIR"), "/extensions_generated.rs"))
}

/// Detect language name from a file path.
///
/// Extracts the file extension and looks it up. Returns `None` if the
/// path has no extension or the extension is not recognized.
///
/// ```
/// use tree_sitter_language_pack::detect_language_from_path;
/// assert_eq!(detect_language_from_path("src/main.rs"), Some("rust"));
/// assert_eq!(detect_language_from_path("README.md"), Some("markdown"));
/// assert_eq!(detect_language_from_path("Makefile"), None);
/// ```
pub fn detect_language_from_path(path: &str) -> Option<&'static str> {
    let ext = std::path::Path::new(path).extension()?.to_str()?;
    detect_language_from_extension(ext)
}

/// Detect language name from file content using the shebang line (`#!`).
///
/// Inspects only the first line of `content`. If it begins with `#!`, the
/// interpreter name is extracted and mapped to a language name.
///
/// Handles common patterns:
/// - `#!/usr/bin/env python3` → `"python"`
/// - `#!/bin/bash` → `"bash"`
/// - `#!/usr/bin/env node` → `"javascript"`
///
/// The `-S` flag accepted by some `env` implementations is skipped automatically.
/// Version suffixes (e.g. `python3.11`, `ruby3.2`) are stripped before matching.
///
/// Returns `None` when content does not start with `#!`, the shebang is
/// malformed, or the interpreter is not recognised.
///
/// ```
/// use tree_sitter_language_pack::detect_language_from_content;
/// assert_eq!(detect_language_from_content("#!/usr/bin/env python3\npass"), Some("python"));
/// assert_eq!(detect_language_from_content("#!/bin/bash\necho hi"), Some("bash"));
/// assert_eq!(detect_language_from_content("no shebang here"), None);
/// ```
pub fn detect_language_from_content(content: &str) -> Option<&'static str> {
    // Fast-path: must start with '#!'
    if !content.starts_with("#!") {
        return None;
    }

    // Locate the end of the first line using memchr for efficiency.
    let bytes = content.as_bytes();
    let line_end = memchr(b'\n', bytes).unwrap_or(bytes.len());
    let shebang_line = &content[2..line_end].trim_end();

    // Split the shebang into whitespace-separated tokens.
    let mut tokens = shebang_line.split_ascii_whitespace();

    // The first token is the interpreter path (e.g. `/usr/bin/env` or `/bin/bash`).
    let interpreter_path = tokens.next()?;

    // Determine the effective program name.
    let program: &str = if interpreter_path.ends_with("/env") || interpreter_path == "env" {
        // The next token after `env` may be a flag like `-S`; skip leading dashes.
        loop {
            let token = tokens.next()?;
            if !token.starts_with('-') {
                break token;
            }
        }
    } else {
        // Direct path: take the final path component.
        interpreter_path.rsplit('/').next()?
    };

    // Strip version suffixes (e.g. `python3.11` → `python`, `ruby3.2` → `ruby`).
    let base = strip_version_suffix(program);

    map_interpreter_to_language(base)
}

/// Remove a trailing version suffix from an interpreter name.
///
/// Strips a leading digit component and anything after the first digit or dot
/// that is part of a version string. Examples: `python3` → `python`,
/// `python3.11` → `python`, `ruby3.2` → `ruby`, `node` → `node`.
fn strip_version_suffix(name: &str) -> &str {
    // Find the first digit or dot that begins the version portion.
    let cut = name.find(|c: char| c.is_ascii_digit()).unwrap_or(name.len());
    // If the character just before the cut is a dot (e.g. `something.1`),
    // also remove that dot so we don't leave trailing punctuation.
    let cut = if cut > 0 && name.as_bytes()[cut - 1] == b'.' {
        cut - 1
    } else {
        cut
    };
    &name[..cut]
}

/// Map a lowercase interpreter base name to a tree-sitter language name.
fn map_interpreter_to_language(interpreter: &str) -> Option<&'static str> {
    match interpreter {
        "python" | "python3" | "python2" => Some("python"),
        "bash" | "sh" | "dash" | "ash" => Some("bash"),
        "zsh" => Some("bash"),
        "node" | "nodejs" => Some("javascript"),
        "ruby" | "jruby" => Some("ruby"),
        "perl" | "perl5" | "perl6" => Some("perl"),
        "lua" => Some("lua"),
        "php" => Some("php"),
        "elixir" => Some("elixir"),
        "julia" => Some("julia"),
        "Rscript" | "r" | "R" => Some("r"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_extensions() {
        assert_eq!(detect_language_from_extension("py"), Some("python"));
        assert_eq!(detect_language_from_extension("pyi"), Some("python"));
        assert_eq!(detect_language_from_extension("rs"), Some("rust"));
        assert_eq!(detect_language_from_extension("js"), Some("javascript"));
        assert_eq!(detect_language_from_extension("ts"), Some("typescript"));
        assert_eq!(detect_language_from_extension("c"), Some("c"));
        assert_eq!(detect_language_from_extension("h"), Some("c"));
        assert_eq!(detect_language_from_extension("cpp"), Some("cpp"));
        assert_eq!(detect_language_from_extension("go"), Some("go"));
        assert_eq!(detect_language_from_extension("rb"), Some("ruby"));
        assert_eq!(detect_language_from_extension("java"), Some("java"));
        assert_eq!(detect_language_from_extension("cs"), Some("csharp"));
        assert_eq!(detect_language_from_extension("tsx"), Some("tsx"));
        assert_eq!(detect_language_from_extension("html"), Some("html"));
        assert_eq!(detect_language_from_extension("css"), Some("css"));
        assert_eq!(detect_language_from_extension("json"), Some("json"));
        assert_eq!(detect_language_from_extension("yaml"), Some("yaml"));
        assert_eq!(detect_language_from_extension("toml"), Some("toml"));
        assert_eq!(detect_language_from_extension("sql"), Some("sql"));
        assert_eq!(detect_language_from_extension("md"), Some("markdown"));
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(detect_language_from_extension("PY"), Some("python"));
        assert_eq!(detect_language_from_extension("Rs"), Some("rust"));
        assert_eq!(detect_language_from_extension("JS"), Some("javascript"));
        assert_eq!(detect_language_from_extension("CPP"), Some("cpp"));
        assert_eq!(detect_language_from_extension("Tsx"), Some("tsx"));
    }

    #[test]
    fn test_unknown() {
        assert_eq!(detect_language_from_extension("xyz"), None);
        assert_eq!(detect_language_from_extension(""), None);
        assert_eq!(detect_language_from_extension("abcdef"), None);
    }

    #[test]
    fn test_path_detection() {
        assert_eq!(detect_language_from_path("src/main.rs"), Some("rust"));
        assert_eq!(detect_language_from_path("/path/to/file.py"), Some("python"));
        assert_eq!(detect_language_from_path("README.md"), Some("markdown"));
        assert_eq!(detect_language_from_path("app.test.tsx"), Some("tsx"));
        assert_eq!(detect_language_from_path("Cargo.toml"), Some("toml"));
    }

    #[test]
    fn test_path_no_extension() {
        assert_eq!(detect_language_from_path("Makefile"), None);
        assert_eq!(detect_language_from_path(""), None);
        assert_eq!(detect_language_from_path("/usr/bin/env"), None);
    }

    #[test]
    fn test_long_extension_rejected() {
        let long = "a".repeat(33);
        assert_eq!(detect_language_from_extension(&long), None);
    }

    // ── shebang detection tests ────────────────────────────────────────────────

    #[test]
    fn test_shebang_env_python3() {
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env python3\npass\n"),
            Some("python")
        );
    }

    #[test]
    fn test_shebang_env_python() {
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env python\npass"),
            Some("python")
        );
    }

    #[test]
    fn test_shebang_direct_python() {
        assert_eq!(detect_language_from_content("#!/usr/bin/python\npass"), Some("python"));
    }

    #[test]
    fn test_shebang_bash() {
        assert_eq!(detect_language_from_content("#!/bin/bash\necho hi"), Some("bash"));
    }

    #[test]
    fn test_shebang_sh() {
        assert_eq!(detect_language_from_content("#!/bin/sh\necho hi"), Some("bash"));
    }

    #[test]
    fn test_shebang_env_node() {
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env node\nconsole.log(1)"),
            Some("javascript")
        );
    }

    #[test]
    fn test_shebang_env_ruby() {
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env ruby\nputs 'hi'"),
            Some("ruby")
        );
    }

    #[test]
    fn test_shebang_direct_perl() {
        assert_eq!(detect_language_from_content("#!/usr/bin/perl\nprint 1"), Some("perl"));
    }

    #[test]
    fn test_shebang_env_lua() {
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env lua\nprint(1)"),
            Some("lua")
        );
    }

    #[test]
    fn test_shebang_env_php() {
        assert_eq!(detect_language_from_content("#!/usr/bin/env php\n<?php"), Some("php"));
    }

    #[test]
    fn test_shebang_env_elixir() {
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env elixir\nIO.puts(1)"),
            Some("elixir")
        );
    }

    #[test]
    fn test_shebang_env_julia() {
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env julia\nprintln(1)"),
            Some("julia")
        );
    }

    #[test]
    fn test_shebang_env_rscript() {
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env Rscript\nprint(1)"),
            Some("r")
        );
    }

    #[test]
    fn test_shebang_env_s_flag() {
        // env -S skips the -S flag and reads the next token as the program.
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env -S python3\npass"),
            Some("python")
        );
    }

    #[test]
    fn test_shebang_version_suffix() {
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env python3.11\npass"),
            Some("python")
        );
        assert_eq!(
            detect_language_from_content("#!/usr/bin/env ruby3.2\nputs 1"),
            Some("ruby")
        );
    }

    #[test]
    fn test_no_shebang() {
        assert_eq!(detect_language_from_content("def foo(): pass"), None);
        assert_eq!(detect_language_from_content("# not a shebang"), None);
    }

    #[test]
    fn test_empty_content() {
        assert_eq!(detect_language_from_content(""), None);
    }

    #[test]
    fn test_shebang_unknown_interpreter() {
        assert_eq!(detect_language_from_content("#!/usr/bin/env unknownlang\ncode"), None);
        assert_eq!(detect_language_from_content("#!/usr/bin/fantasy\ncode"), None);
    }

    /// Verify that ext→name detection is independent of parser availability.
    ///
    /// `detect_language_from_extension` consults the static extension table that
    /// is generated from the full `language_definitions.json` for all 306 grammars.
    /// It does NOT gate on whether the parser was compiled in (controlled by
    /// `TSLP_LANGUAGES` at build time). Subset FFI builds must still return the
    /// correct name for any extension in the table.
    ///
    /// We verify this by using a language that may or may not be compiled in
    /// (gherkin/.feature) and asserting the extension lookup succeeds regardless,
    /// then separately checking parser availability via `has_parser`.
    #[test]
    fn test_ext_detection_independent_of_parser_availability() {
        // `.feature` files are always mapped to "gherkin" in the static table.
        // This holds whether or not the gherkin parser is compiled into this build.
        assert_eq!(
            detect_language_from_extension("feature"),
            Some("gherkin"),
            "ext 'feature' must resolve to 'gherkin' from the static table regardless of build subset"
        );
        // Callers that need to parse should gate on has_parser, not on ext detection.
        // We don't assert a specific value for has_parser here since it depends on the
        // build config — the point is that ext detection is always non-None.
        let _ = crate::has_language("gherkin");
    }

    /// Validate that JSON definitions match generated code by round-tripping.
    /// Loads language_definitions.json at test time and checks every extension
    /// resolves correctly via the generated lookup.
    #[test]
    fn test_roundtrip_json_to_generated() {
        let json_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../sources/language_definitions.json");
        let json_str = match std::fs::read_to_string(json_path) {
            Ok(s) => s,
            Err(_) => return, // Skip when sources/ not available (e.g. crates.io install)
        };
        let defs: std::collections::BTreeMap<String, serde_json::Value> =
            serde_json::from_str(&json_str).expect("Failed to parse language_definitions.json");

        for (lang_name, def) in &defs {
            if let Some(extensions) = def.get("extensions").and_then(|v| v.as_array()) {
                for ext_val in extensions {
                    let ext = ext_val.as_str().expect("extension must be a string");
                    let result = detect_language_from_extension(ext);
                    assert_eq!(
                        result,
                        Some(lang_name.as_str()),
                        "Extension '{ext}' should map to '{lang_name}' but got {result:?}"
                    );
                }
            }
        }
    }
}
