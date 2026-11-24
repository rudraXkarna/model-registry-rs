mod pipeline;
mod benchmarks;

use pipeline::model::{ModelMetadata, Framework};
use pipeline::artifact::ArtifactStore;
use benchmarks::BENCHMARK_CATALOG;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("============================================================");
    println!(" MLOps Pipeline & Detector Benchmark Suite (Rust) ");
    println!("============================================================");

    // 1. Demonstrate standard pipeline artifact management
    println!("\n[1] Initializing MLOps Artifact Store...");
    let temp_store_dir = std::env::temp_dir().join("mlops_bench_artifacts");
    let store = ArtifactStore::new(&temp_store_dir)
        .map_err(|e| format!("Store init failed: {}", e))?;

    let model = ModelMetadata::new(
        "resnet50-pipeline-v1",
        "1.2.0",
        Framework::Safetensors,
        vec![1, 3, 224, 224],
        vec![1, 1000],
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    );

    let saved_path = store.save_metadata("resnet50_meta.json", &model)?;
    println!("    Artifact metadata saved to: {:?}", saved_path);

    let loaded = store.load_metadata("resnet50_meta.json")?;
    println!("    Verified metadata load: {} (Framework: {:?})", loaded.name, loaded.framework);

    // Validate a sample tensor input shape against model definition
    let sample_input = vec![0.0f32; 1 * 3 * 224 * 224];
    match loaded.validate_input_dimensions(&sample_input) {
        Ok(_) => println!("    Input validation passed: shape [1, 3, 224, 224] matches spec."),
        Err(e) => eprintln!("    Input validation failed: {}", e),
    }

    // 2. Display security detector benchmark catalog
    println!("\n[2] Security Detector Test Catalog:");
    println!("    These paired fixtures are available for static analysis validation.\n");

    for (idx, rule) in BENCHMARK_CATALOG.iter().enumerate() {
        println!("    Rule #{}: {}", idx + 1, rule.rule_id);
        println!("      CWE:              {}", rule.cwe);
        println!("      Description:      {}", rule.description);
        println!("      Flawed Symbol:    {} (expected TRUE POSITIVE)", rule.expected_flawed_symbol);
        println!("      Clean Symbol:     {} (expected FALSE POSITIVE CONTROL)", rule.expected_clean_symbol);
        println!();
    }

    println!("Run `cargo test` to execute unit tests against these modules.");
    println!("============================================================");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_artifact_traversal_protection() {
        let temp_dir = std::env::temp_dir().join("mlops_test_traversal");
        let store = ArtifactStore::new(&temp_dir).unwrap();

        // Testing that traversal strings are rejected by the safe resolver
        let bad_path = Path::new("../../sensitive_file.txt");
        let result = store.resolve_safe_path(bad_path);
        assert!(result.is_err(), "Artifact store must reject path traversal components");
    }

    #[test]
    fn test_model_dimension_validation() {
        let model = ModelMetadata::new(
            "test-net",
            "0.1.0",
            Framework::Torch,
            vec![2, 2], // 4 elements total
            vec![1],
            "fake_hash",
        );

        let valid_input = vec![1.0, 2.0, 3.0, 4.0];
        assert!(model.validate_input_dimensions(&valid_input).is_ok());

        let invalid_input = vec![1.0, 2.0];
        assert!(model.validate_input_dimensions(&invalid_input).is_err());
    }

    #[test]
    fn test_safe_slice_conversion() {
        let sample_bytes = [0x00, 0x00, 0x80, 0x3F]; // 1.0f32 in little-endian IEEE-754
        let val = benchmarks::unsafe_memory::pattern_good_slice_conversion(&sample_bytes);
        assert_eq!(val, Some(1.0f32));
    }
}
