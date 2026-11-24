use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Framework {
    Torch,
    Safetensors,
    ONNX,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub name: String,
    pub version: String,
    pub framework: Framework,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
    pub sha256_checksum: String,
}

impl ModelMetadata {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        framework: Framework,
        input_shape: Vec<usize>,
        output_shape: Vec<usize>,
        checksum: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            framework,
            input_shape,
            output_shape,
            sha256_checksum: checksum.into(),
        }
    }

    pub fn validate_input_dimensions(&self, input: &[f32]) -> Result<(), String> {
        let expected_size: usize = self.input_shape.iter().product();
        if input.len() != expected_size {
            return Err(format!(
                "Dimension mismatch: expected {} elements, got {}",
                expected_size,
                input.len()
            ));
        }
        Ok(())
    }
}
