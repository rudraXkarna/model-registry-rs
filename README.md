<div align="center">

# 🦀 `model-registry-rs`

### **High-Performance MLOps Model Registry & Security Evaluation Harness in Rust**

[![Rust](https://img.shields.io/badge/language-Rust-DEA584.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Release](https://img.shields.io/badge/Release-v0.1.1-2ea44f.svg?style=for-the-badge&logo=github)](https://github.com/rudraXkarna/model-registry-rs/releases/tag/v0.1.1)
[![Security Advisory](https://img.shields.io/badge/Security-GHSA--v82p--522j--rjrr-red.svg?style=for-the-badge&logo=githubsecurity)](https://github.com/rudraXkarna/model-registry-rs/security/advisories/GHSA-v82p-522j-rjrr)
[![License](https://img.shields.io/badge/License-Apache--2.0-blue.svg?style=for-the-badge)](LICENSE)

<p align="center">
  A blazingly fast, type-safe MLOps control plane for managing machine learning model metadata, validating multidimensional tensor input constraints, and storing model artifacts with hardened path containment — accompanied by a dedicated static analysis evaluation benchmark suite.
</p>

[Key Features](#-key-features) • [Architecture](#-architecture) • [Getting Started](#-getting-started) • [Security & CWE Hardening](#-security--cwe-hardening) • [Benchmarks](#-static-analysis-benchmark-suite)

---
</div>

## 📖 What is `model-registry-rs`?

In modern production machine learning pipelines, model artifacts and weights are constantly transferred across distributed training clusters, inference engines, and storage tiers. Without strict validation and containment:
1. **Model Drift & Shape Crashes:** Discrepancies between expected model tensor shapes and incoming inference payloads cause runtime segmentation faults or panics in native inference engines.
2. **Artifact Poisoning & Path Traversal:** Insecure artifact resolution exposes host filesystems to traversal attacks and arbitrary file overwrite during model rollout.

**`model-registry-rs`** is engineered to solve these core operational challenges natively in Rust, offering a zero-cost abstraction foundation for MLOps engineers, security researchers, and static analysis tool authors.

---

## ✨ Key Features

- **🧠 Strongly-Typed Model Governance**  
  Support for industry-standard ML model formats including **Safetensors**, **PyTorch (`Torch`)**, and **ONNX** with cryptographic SHA-256 integrity checks.

- **📐 Pre-Inference Tensor Shape Validation**  
  Zero-allocation, compile-time verified multidimensional tensor size and dimension validation before handing execution to heavy inference runtimes.

- **🛡️ Containment-Hardened Artifact Storage (`v0.1.1`)**  
  Multi-layer path normalization and containment engine:
  - Rejection of absolute paths and Windows drive/prefix components.
  - Parent directory traversal (`..`) defense.
  - Symlink escape detection via canonical ancestor traversal.

- **🎯 Dual-Control Security Benchmark Harness**  
  A built-in benchmark fixture catalog designed for testing static analysis detectors (SAST) and vulnerability scanners against paired **True Positive** anti-patterns and **False Positive Control** remediations across **CWE-22**, **CWE-78**, and **CWE-119 / CWE-843**.

---

## 🏗️ Architecture

```
                                  ┌──────────────────────────────┐
                                  │   Incoming Model Artifact    │
                                  └──────────────┬───────────────┘
                                                 │
                                                 ▼
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                                   model-registry-rs                                    │
│                                                                                        │
│   ┌───────────────────────────┐                    ┌───────────────────────────────┐   │
│   │    ModelMetadata Engine   │                    │       ArtifactStore           │   │
│   │  - Framework Identification│                    │  - Path Traversal Shield      │   │
│   │  - Tensor Shape Validation │                    │  - Symlink Containment Check  │   │
│   │  - SHA-256 Hash Auditing  │                    │  - Isolated JSON Serializer   │   │
│   └─────────────┬─────────────┘                    └───────────────┬───────────────┘   │
└─────────────────┼──────────────────────────────────────────────────┼───────────────────┘
                  │                                                  │
                  ▼                                                  ▼
     Validated Tensor Ingestion                        Safe On-Disk Model Storage
```

---

## 🚀 Getting Started

### Prerequisites

Ensure you have the Rust toolchain installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building & Running

```bash
# Clone the repository
git clone https://github.com/rudraXkarna/model-registry-rs.git
cd model-registry-rs

# Compile with release optimizations
cargo build --release

# Execute unit and regression test suite
cargo test

# Run the live pipeline demonstration and benchmark catalog
cargo run
```

---

## 💻 Code Example

```rust
use model_registry_rs::pipeline::model::{ModelMetadata, Framework};
use model_registry_rs::pipeline::artifact::ArtifactStore;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize hardened artifact store
    let store = ArtifactStore::new("/var/model-storage")?;

    // 2. Register model metadata with shape specifications
    let model = ModelMetadata::new(
        "resnet50-safetensors",
        "1.2.0",
        Framework::Safetensors,
        vec![1, 3, 224, 224], // Batch, Channels, Height, Width
        vec![1, 1000],
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    );

    // 3. Save metadata securely (immune to CWE-22 path traversal)
    store.save_metadata("resnet50_v1.json", &model)?;

    // 4. Validate tensor input shape prior to inference
    let input_tensor = vec![0.0f32; 1 * 3 * 224 * 224];
    model.validate_input_dimensions(&input_tensor)?;
    println!("✅ Tensor shape verified. Safe for inference dispatch.");

    Ok(())
}
```

---

## 🛡️ Security & Responsible Disclosure

Security is a primary focus of `model-registry-rs`. 

### Security Advisory GHSA-v82p-522j-rjrr (Fixed in `v0.1.1`)
In versions `<= 0.1.0`, `ArtifactStore::resolve_safe_path` did not reject absolute paths or root/prefix components, which allowed `Path::join` to replace the target directory entirely (CWE-22). 

This vulnerability was responsibly disclosed and remediated in **v0.1.1**:
- **Advisory:** [GHSA-v82p-522j-rjrr](https://github.com/rudraXkarna/model-registry-rs/security/advisories/GHSA-v82p-522j-rjrr)
- **Reporter Credit:** Special thanks to [@ATLURI0001](https://github.com/ATLURI0001) for discovering, reproducing, and coordinating the responsible disclosure of this issue.

To report new vulnerabilities, please utilize GitHub’s **Private Vulnerability Reporting** via the [Security tab](https://github.com/rudraXkarna/model-registry-rs/security).

---

## 🎯 Static Analysis Benchmark Suite

`model-registry-rs` includes a security benchmark catalog under `src/benchmarks/` built to evaluate static analysis security testing (SAST) tools and memory safety checkers:

| Rule ID | CWE | Description | Flawed Symbol (True Positive) | Clean Symbol (Control) |
| :--- | :--- | :--- | :--- | :--- |
| `RULE-CWE-22` | **CWE-22** | Unsanitized path joining | `pattern_bad_extract_join` | `pattern_good_extract_join` |
| `RULE-CWE-78` | **CWE-78** | Unescaped shell command invocation | `pattern_bad_command_exec` | `pattern_good_command_exec` |
| `RULE-CWE-119` | **CWE-119 / 843** | Unaligned raw pointer dereferencing | `pattern_bad_pointer_cast` | `pattern_good_slice_conversion` |

---

## 📜 License

Licensed under the [Apache License, Version 2.0](LICENSE).