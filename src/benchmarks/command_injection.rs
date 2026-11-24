//! Benchmark Fixture: CWE-78 (OS Command / Argument Injection)
//!
//! Intended for static analysis rules checking subprocess invocation and shell construction.

use std::process::Command;

/// [VULNERABILITY PATTERN: TRUE POSITIVE]
/// Flawed implementation: formats user arguments directly into a shell command line string.
pub fn pattern_bad_command_exec(user_script_arg: &str) -> Command {
    // Flagged pattern: invoking system shell with format string concatenation
    let shell_command = format!("python train.py --config {}", user_script_arg);
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(shell_command);
    cmd
}

/// [REMEDIATED PATTERN: FALSE POSITIVE CONTROL]
/// Safe implementation: passes arguments as discrete, non-interpreted parameters.
pub fn pattern_good_command_exec(config_path: &str) -> Command {
    let mut cmd = Command::new("python");
    cmd.args(["train.py", "--config", config_path]);
    cmd
}
