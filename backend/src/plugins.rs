use serde_json::{json, Value};
use std::{
    path::Path,
    process::{Command, Stdio},
};

fn run_program(program: &str, args: &[&str], stdin_payload: &str) -> Result<String, String> {
    let mut command = Command::new(program);
    command.args(args);
    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|e| format!("spawn failed: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin
            .write_all(stdin_payload.as_bytes())
            .map_err(|e| format!("stdin write failed: {e}"))?;
    }

    let output = child.wait_with_output().map_err(|e| format!("wait failed: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(if stderr.is_empty() {
            format!("process exited with {}", output.status)
        } else {
            stderr
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn python_analyze(plugin_dir: &Path, payload: &str) -> Value {
    let path = plugin_dir.join("python").join("analyze.py");
    if path.exists() {
        let path_str = path.to_string_lossy().to_string();
        if let Ok(raw) = run_program("python3", &[&path_str], payload) {
            if let Ok(value) = serde_json::from_str::<Value>(&raw) {
                return value;
            }
            return json!({ "tool": "python3", "raw": raw, "fallback": false });
        }
    }

    json!({ "tool": "rust", "fallback": true, "summary": payload.len(), "mode": "analysis" })
}

pub fn zig_check(plugin_dir: &Path, payload: &str) -> Value {
    let path = plugin_dir.join("zig").join("processor.zig");
    json!({
        "tool": "zig",
        "source": path.exists(),
        "fallback": true,
        "input_len": payload.len()
    })
}

pub fn mojo_check(plugin_dir: &Path, payload: &str) -> Value {
    let path = plugin_dir.join("mojo").join("processor.mojo");
    json!({
        "tool": "mojo",
        "source": path.exists(),
        "fallback": true,
        "input_len": payload.len()
    })
}

pub fn c_monitor(plugin_dir: &Path, payload: &str) -> Value {
    let path = plugin_dir.join("c").join("monitor.c");
    json!({
        "tool": "c",
        "source": path.exists(),
        "fallback": true,
        "input_len": payload.len()
    })
}

pub fn cpp_verify(plugin_dir: &Path, payload: &str) -> Value {
    let path = plugin_dir.join("cpp").join("verify.cpp");
    json!({
        "tool": "cpp",
        "source": path.exists(),
        "fallback": true,
        "input_len": payload.len()
    })
}

pub fn java_execute(plugin_dir: &Path, payload: &str) -> Value {
    let path = plugin_dir.join("java").join("Executor.java");
    json!({
        "tool": "java",
        "source": path.exists(),
        "fallback": true,
        "input_len": payload.len()
    })
}
