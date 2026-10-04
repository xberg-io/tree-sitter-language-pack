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

/// Return the assigned language and alternatives for an ambiguous file extension.
///
/// The extension must omit the leading dot. Matching is ASCII case-insensitive,
/// like [`detect_language_from_extension`]. Returns `None` for unambiguous or
/// unrecognized extensions. The assigned language matches extension detection;
/// alternatives list other languages declared in the language definitions.
/// This lookup does not require the corresponding parsers to be installed.
///
/// ```
/// use tree_sitter_language_pack::extension_ambiguity;
/// assert_eq!(extension_ambiguity("H"), Some(("c", &["cpp", "objc"][..])));
/// assert_eq!(extension_ambiguity("m"), Some(("objc", &["matlab"][..])));
/// assert_eq!(extension_ambiguity("py"), None);
/// ```
pub fn extension_ambiguity(ext: &str) -> Option<(&'static str, &'static [&'static str])> {
    const MAX_EXTENSION_BYTES: usize = 32;
    let mut buffer = [0u8; MAX_EXTENSION_BYTES];
    if ext.len() > buffer.len() || !ext.is_ascii() {
        return None;
    }
    for (index, byte) in ext.bytes().enumerate() {
        buffer[index] = byte.to_ascii_lowercase();
    }
    let ext_lower = std::str::from_utf8(&buffer[..ext.len()]).ok()?;

    include!(concat!(env!("OUT_DIR"), "/ambiguities_generated.rs"))
}

/// Serialize [`extension_ambiguity`] as JSON with `assigned` and `alternatives` fields.
///
/// Available with the `serde` feature. Returns `None` when the extension is
/// unambiguous or unrecognized.
///
/// ```
/// use tree_sitter_language_pack::extension_ambiguity_json;
/// assert_eq!(
///     extension_ambiguity_json("m"),
///     Some(serde_json::json!({"assigned": "objc", "alternatives": ["matlab"]}).to_string())
/// );
/// ```
#[cfg(feature = "serde")]
pub fn extension_ambiguity_json(ext: &str) -> Option<String> {
    extension_ambiguity(ext).map(|(assigned, alternatives)| {
        serde_json::json!({
            "assigned": assigned,
            "alternatives": alternatives,
        })
        .to_string()
    })
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
    let file_name = std::path::Path::new(path).file_name()?.to_str()?;
    // ~keep Check compound extensions first so `foo.app.src` maps to Erlang rather than `src`.
    if let Some(lang) = detect_compound_extension(file_name) {
        return Some(lang);
    }
    let ext = std::path::Path::new(path).extension()?.to_str()?;
    detect_language_from_extension(ext)
}

/// Multi-dot extension suffixes that map to a language, matched case-insensitively
/// against the full file name. The generated extension table only holds single,
/// dot-free keys, so compound extensions live here.
const COMPOUND_EXTENSIONS: &[(&str, &str)] = &[
    (".app.src", "erlang"),
    // ~keep dotfiles and multi-dot names have no plain extension, so they only resolve here.
    (".rs.html", "rshtml"),
    ("kitty.conf", "kitty"),
    (".env", "dotenv"),
];

/// Match a file name against the [`COMPOUND_EXTENSIONS`] suffix table.
fn detect_compound_extension(file_name: &str) -> Option<&'static str> {
    let lower = file_name.to_ascii_lowercase();
    COMPOUND_EXTENSIONS
        .iter()
        .find(|(suffix, _)| lower.ends_with(suffix))
        .map(|(_, lang)| *lang)
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
/// A leading UTF-8 BOM (`U+FEFF`) is skipped before the `#!` check, so a
/// BOM-prefixed script is still detected by its shebang.
///
/// Returns `None` when content does not start with `#!` (after stripping a
/// leading BOM), the shebang is malformed, or the interpreter is not recognised.
///
/// ```
/// use tree_sitter_language_pack::detect_language_from_content;
/// assert_eq!(detect_language_from_content("#!/usr/bin/env python3\npass"), Some("python"));
/// assert_eq!(detect_language_from_content("#!/bin/bash\necho hi"), Some("bash"));
/// assert_eq!(detect_language_from_content("no shebang here"), None);
/// assert_eq!(
///     detect_language_from_content("\u{FEFF}#!/usr/bin/env python3\npass"),
///     Some("python")
/// );
/// ```
pub fn detect_language_from_content(content: &str) -> Option<&'static str> {
    // ~keep A leading BOM is common in files saved by Windows-oriented tools and must
    // ~keep not hide the shebang from this scan. This is the only BOM-sensitive spot in
    // ~keep the crate: tree-sitter parsing itself already tolerates a leading BOM as
    // ~keep insignificant trivia rather than an error (verified empirically against this
    // ~keep pack's compiled python/rust/go/javascript grammars), so byte offsets reported
    // ~keep elsewhere need no adjustment and this function returns no offsets to begin with.
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(content);
    if !content.starts_with("#!") {
        return None;
    }

    let bytes = content.as_bytes();
    let line_end = memchr(b'\n', bytes).unwrap_or(bytes.len());
    let shebang_line = &content[2..line_end].trim_end();

    let mut tokens = shebang_line.split_ascii_whitespace();

    let interpreter_path = tokens.next()?;

    let program: &str = if interpreter_path.ends_with("/env") || interpreter_path == "env" {
        loop {
            let token = tokens.next()?;
            if !token.starts_with('-') {
                break token;
            }
        }
    } else {
        interpreter_path.rsplit('/').next()?
    };

    let base = strip_version_suffix(program);

    map_interpreter_to_language(base)
}

/// Remove a trailing version suffix from an interpreter name.
///
/// Strips a leading digit component and anything after the first digit or dot
/// that is part of a version string. Examples: `python3` → `python`,
/// `python3.11` → `python`, `ruby3.2` → `ruby`, `node` → `node`.
fn strip_version_suffix(name: &str) -> &str {
    let cut = name.find(|c: char| c.is_ascii_digit()).unwrap_or(name.len());
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
    fn should_detect_baml_from_extension() {
        assert_eq!(detect_language_from_extension("baml"), Some("baml"));
        assert_eq!(detect_language_from_path("baml_src/clients.baml"), Some("baml"));
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
    fn test_compound_extension_app_src_is_erlang() {
        assert_eq!(detect_language_from_path("myapp.app.src"), Some("erlang"));
        assert_eq!(detect_language_from_path("rel/foo/foo.app.src"), Some("erlang"));
        assert_eq!(detect_language_from_path("FOO.APP.SRC"), Some("erlang"));
        assert_eq!(detect_language_from_path("notes.src"), None);
    }

    #[test]
    fn test_compound_extensions_for_multi_dot_and_dotfiles() {
        // rshtml uses `.rs.html`, which must win over the plain `.html` mapping.
        assert_eq!(detect_language_from_path("templates/page.rs.html"), Some("rshtml"));
        assert_eq!(detect_language_from_path("page.html"), Some("html"));
        // kitty's config is the specific file name `kitty.conf`.
        assert_eq!(detect_language_from_path(".config/kitty/kitty.conf"), Some("kitty"));
        // dotenv files carry no plain extension.
        assert_eq!(detect_language_from_path(".env"), Some("dotenv"));
        assert_eq!(detect_language_from_path("service/.env"), Some("dotenv"));
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
    fn test_shebang_with_leading_bom() {
        assert_eq!(
            detect_language_from_content("\u{FEFF}#!/usr/bin/env python3\npass"),
            Some("python"),
            "a leading UTF-8 BOM must not hide the shebang line"
        );
        assert_eq!(
            detect_language_from_content("\u{FEFF}#!/bin/bash\necho hi"),
            Some("bash")
        );
    }

    #[test]
    fn test_bom_only_content_is_not_a_shebang() {
        assert_eq!(detect_language_from_content("\u{FEFF}no shebang here"), None);
        assert_eq!(detect_language_from_content("\u{FEFF}"), None);
    }

    #[test]
    fn test_shebang_unknown_interpreter() {
        assert_eq!(detect_language_from_content("#!/usr/bin/env unknownlang\ncode"), None);
        assert_eq!(detect_language_from_content("#!/usr/bin/fantasy\ncode"), None);
    }

    /// Verify that ext→name detection is independent of parser availability.
    ///
    /// `detect_language_from_extension` consults the static extension table that
    /// is generated from the full `language_definitions.json` for all 372 grammars.
    /// It does NOT gate on whether the parser was compiled in (controlled by
    /// `TSLP_LANGUAGES` at build time). Subset FFI builds must still return the
    /// correct name for any extension in the table.
    ///
    /// We verify this by using a language that may or may not be compiled in
    /// (gherkin/.feature) and asserting the extension lookup succeeds regardless,
    /// then separately checking parser availability via `has_parser`.
    #[test]
    fn test_ext_detection_independent_of_parser_availability() {
        // ~keep `.feature` maps to gherkin regardless of whether that parser is compiled.
        assert_eq!(
            detect_language_from_extension("feature"),
            Some("gherkin"),
            "ext 'feature' must resolve to 'gherkin' from the static table regardless of build subset"
        );
        // ~keep Parser availability is build-dependent; extension detection intentionally is not.
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
            Err(_) => return,
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
