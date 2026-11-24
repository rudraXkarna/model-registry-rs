# model-registry-rs

A sample Rust project demonstrating core MLOps artifact management components, model registry metadata validation, and a dedicated benchmark fixture suite for testing and evaluating vulnerability detectors and static analysis tools.

## Project Structure

```
model-registry-rs/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs                   # Entry point and pipeline demonstration
    ├── pipeline/                 # Production-grade MLOps components
    │   ├── mod.rs
    │   ├── model.rs              # Model metadata, framework, and shape validation
    │   └── artifact.rs           # Safe artifact storage and path traversal defense
    └── benchmarks/               # Security detector benchmark fixtures
        ├── mod.rs                # Benchmark catalog and rule definitions
        ├── path_traversal.rs     # CWE-22: Bad vs Good path join patterns
        ├── command_injection.rs  # CWE-78: Bad vs Good command execution patterns
        └── unsafe_memory.rs      # CWE-119 / CWE-843: Bad vs Good memory cast patterns
```

## How to Test Your Detector Against This Project

Each benchmark file under `src/benchmarks/` contains paired functions:
1. **`pattern_bad_*`**: Represents an isolated anti-pattern matching a specific CWE. A vulnerability detector should flag this as a **True Positive**.
2. **`pattern_good_*`**: Represents the safe, remediated equivalent. A vulnerability detector should treat this as a **False Positive control** (it should not trigger an alert).

### Evaluating Your Detector:
1. Point your detector tool at `src/` (or specifically `src/benchmarks/`).
2. Verify that:
   - All `pattern_bad_*` occurrences are detected.
   - All `pattern_good_*` and `pipeline/` functions pass cleanly without false alarms.

## Running the Project

```bash
# Build the project
cargo build

# Run unit tests
cargo test

# Run the pipeline demo and view the benchmark catalog
cargo run
```
