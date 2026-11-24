//! Benchmark Fixture: CWE-22 (Path Traversal / Zip Slip pattern)
//!
//! Intended for static analysis rules checking file extraction and path joins.

use std::path::{Path, PathBuf};

/// [VULNERABILITY PATTERN: TRUE POSITIVE]
/// Flawed implementation: joins incoming relative path directly without checking
/// for parent directory references (`..`), allowing paths to escape destination.
pub fn pattern_bad_extract_join(dest_dir: &Path, user_supplied_filename: &str) -> PathBuf {
    // Flagged pattern: direct join of untrusted path component
    dest_dir.join(user_supplied_filename)
}

/// [REMEDIATED PATTERN: FALSE POSITIVE CONTROL]
/// Safe implementation: verifies that no path components traverse upwards.
pub fn pattern_good_extract_join(dest_dir: &Path, user_supplied_filename: &str) -> Result<PathBuf, String> {
    let rel_path = Path::new(user_supplied_filename);
    for component in rel_path.components() {
        if matches!(component, std::path::Component::ParentDir | std::path::Component::RootDir) {
            return Err("Disallowed path traversal component detected".to_string());
        }
    }
    Ok(dest_dir.join(rel_path))
}
