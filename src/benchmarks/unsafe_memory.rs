//! Benchmark Fixture: CWE-119 / CWE-843 (Type Confusion / Unsafe Pointer Misuse)
//!
//! Intended for static analysis rules auditing `unsafe` blocks and pointer casting.

/// [VULNERABILITY PATTERN: TRUE POSITIVE]
/// Flawed implementation: transmuting between unrelated types via raw pointers.
#[allow(clippy::transmute_ptr_to_ptr)]
pub fn pattern_bad_pointer_cast(data: &[u8]) -> Option<f32> {
    if data.len() < 4 {
        return None;
    }
    // Flagged pattern: dereferencing unaligned/unverified raw pointer inside unsafe block
    unsafe {
        let ptr = data.as_ptr() as *const f32;
        Some(*ptr)
    }
}

/// [REMEDIATED PATTERN: FALSE POSITIVE CONTROL]
/// Safe implementation: using safe slice conversion methods with explicit alignment and bounds.
pub fn pattern_good_slice_conversion(data: &[u8]) -> Option<f32> {
    if data.len() < 4 {
        return None;
    }
    let bytes: [u8; 4] = data[0..4].try_into().ok()?;
    Some(f32::from_le_bytes(bytes))
}
