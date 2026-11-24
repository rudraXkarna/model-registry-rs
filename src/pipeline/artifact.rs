use std::path::{Path, PathBuf};
use std::fs;
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
        // Prevent path traversal components
        for component in relative_path.components() {
            if let std::path::Component::ParentDir = component {
                return Err("Path traversal (..) not permitted in artifact paths".to_string());
            }
        }

        let full_path = self.root_dir.join(relative_path);
        Ok(full_path)
    }

    /// Saves model metadata into the artifact repository using structured JSON
    pub fn save_metadata(&self, filename: &str, metadata: &ModelMetadata) -> Result<PathBuf, String> {
        let target_path = self.resolve_safe_path(Path::new(filename))?;
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
