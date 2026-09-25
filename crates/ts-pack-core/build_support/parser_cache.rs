//! Parser-source layout probes shared by workspace and release-cache builds.
use std::path::{Path, PathBuf};

pub fn parsers_root_populated(root: &Path, selected: &[String]) -> bool {
    if selected.is_empty() {
        root.join("python/src/parser.c").is_file()
    } else {
        selected
            .iter()
            .all(|language| root.join(language).join("src/parser.c").is_file())
    }
}

pub fn cached_parsers_root(cache: &Path, selected: &[String]) -> Option<PathBuf> {
    let nested = cache.join("parsers");
    if parsers_root_populated(&nested, selected) {
        Some(nested)
    } else if parsers_root_populated(cache, selected) {
        Some(cache.to_path_buf())
    } else {
        None
    }
}
