#![allow(dead_code)]

pub mod path_traversal;
pub mod command_injection;
pub mod unsafe_memory;

#[derive(Debug, Clone)]
pub struct BenchmarkRule {
    pub rule_id: &'static str,
    pub cwe: &'static str,
    pub description: &'static str,
    pub expected_flawed_symbol: &'static str,
    pub expected_clean_symbol: &'static str,
}

pub const BENCHMARK_CATALOG: &[BenchmarkRule] = &[
    BenchmarkRule {
        rule_id: "RULE-CWE-22-PATH-TRAVERSAL",
        cwe: "CWE-22",
        description: "Direct joining of unsanitized path strings allowing path traversal",
        expected_flawed_symbol: "pattern_bad_extract_join",
        expected_clean_symbol: "pattern_good_extract_join",
    },
    BenchmarkRule {
        rule_id: "RULE-CWE-78-COMMAND-INJECTION",
        cwe: "CWE-78",
        description: "Formatted command line string execution via system shell wrapper",
        expected_flawed_symbol: "pattern_bad_command_exec",
        expected_clean_symbol: "pattern_good_command_exec",
    },
    BenchmarkRule {
        rule_id: "RULE-CWE-119-UNSAFE-POINTER",
        cwe: "CWE-119 / CWE-843",
        description: "Unaligned raw pointer cast and dereference without layout validation",
        expected_flawed_symbol: "pattern_bad_pointer_cast",
        expected_clean_symbol: "pattern_good_slice_conversion",
    },
];
