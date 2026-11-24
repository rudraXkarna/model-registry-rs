use std::fs;
use std::path::{Component, Path, PathBuf};
use crate::pipeline::model::ModelMetadata;

pub struct ArtifactStore {
    root_dir: PathBuf,
}

impl ArtifactStore {
    pub fn new(root: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let canonical_root = fs::canonicalize(&root).unwrap_or_else(|_| root.as_ref().to_path_buf());
        if !canonical_root.exists() {
            fs::create_dir_all(&canonical_root)?;
        }
        Ok(Self {
            root_dir: canonical_root,
        })
    }

    /// Resolves an artifact path safely ensuring it stays contained within root_dir
    pub fn resolve_safe_path(&self, relative_path: &Path) -> Result<PathBuf, String> {
        // 1. Explicitly reject absolute paths
        if relative_path.is_absolute() {
            return Err("Absolute paths are not permitted in artifact paths".to_string());
        }

        // 2. Reject RootDir, Prefix (e.g. Windows drive letters C:), and ParentDir (..)
        for component in relative_path.components() {
            match component {
                Component::ParentDir => {
                    return Err("Path traversal (..) not permitted in artifact paths".to_string());
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err("Root and prefix components are not permitted in artifact paths".to_string());
                }
                Component::Normal(_) | Component::CurDir => {}
            }
        }

        let full_path = self.root_dir.join(relative_path);

        // 3. Symlink containment check for existing paths or parents
        let mut ancestor = full_path.as_path();
        while !ancestor.exists() {
            if let Some(parent) = ancestor.parent() {
                ancestor = parent;
            } else {
                break;
            }
        }

        if ancestor.exists() {
            let canonical_ancestor = fs::canonicalize(ancestor)
                .map_err(|e| format!("Filesystem resolution error: {}", e))?;
            if !canonical_ancestor.starts_with(&self.root_dir) {
                return Err("Path traversal via symlink detected".to_string());
            }
        }

        Ok(full_path)
    }

    /// Saves model metadata into the artifact repository using structured JSON
    pub fn save_metadata(&self, filename: &str, metadata: &ModelMetadata) -> Result<PathBuf, String> {
        let target_path = self.resolve_safe_path(Path::new(filename))?;
        
        // Ensure parent directory exists if filename includes subdirectories
        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directories: {}", e))?;
            }
        }

        let json_data = serde_json::to_string_pretty(metadata)
            .map_err(|e| format!("Serialization error: {}", e))?;

        fs::write(&target_path, json_data)
            .map_err(|e| format!("Filesystem write failed: {}", e))?;

        Ok(target_path)
    }

    /// Loads and parses model metadata from the artifact repository
    pub fn load_metadata(&self, filename: &str) -> Result<ModelMetadata, String> {
        let target_path = self.resolve_safe_path(Path::new(filename))?;
        let content = fs::read_to_string(&target_path)
            .map_err(|e| format!("Filesystem read failed: {}", e))?;

        let metadata: ModelMetadata = serde_json::from_str(&content)
            .map_err(|e| format!("Schema validation error: {}", e))?;

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rejects_absolute_path() {
        let temp_dir = std::env::temp_dir().join("test_artifact_store_abs");
        let store = ArtifactStore::new(&temp_dir).unwrap();

        let exploit_path = Path::new("/etc/passwd");
        assert!(store.resolve_safe_path(exploit_path).is_err());
    }

    #[test]
    fn test_rejects_parent_dir_traversal() {
        let temp_dir = std::env::temp_dir().join("test_artifact_store_dotdot");
        let store = ArtifactStore::new(&temp_dir).unwrap();

        let exploit_path = Path::new("../escaped.json");
        assert!(store.resolve_safe_path(exploit_path).is_err());
    }

    #[test]
    fn test_allows_valid_relative_path() {
        let temp_dir = std::env::temp_dir().join("test_artifact_store_valid");
        let store = ArtifactStore::new(&temp_dir).unwrap();

        let valid_path = Path::new("models/v1/metadata.json");
        let resolved = store.resolve_safe_path(valid_path).unwrap();
        assert!(resolved.starts_with(&store.root_dir));
    }
}
