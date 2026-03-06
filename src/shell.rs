use std::process::Command;
use std::vec::Vec;

/// Parse each line as utf8, ignoring lines that fail utf8 conversion
fn read_utf8_lines_lossy(bytes: &Vec<u8>) -> Vec<String> {
    let mut lines: Vec<String> = vec![];

    for split in bytes.split(|v| *v == b'\n') {
        if let Ok(line) = String::from_utf8(split.to_vec()) {
            lines.push(line.to_string());
        }
    }

    lines
}

/// Run a shell command and capture its output as utf8
pub fn capture_cmd_lossy(cmd: &str) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
    };

    let lines = read_utf8_lines_lossy(&output.stdout);

    lines.join("\n")
}

#[test]
fn test_echo() {
    let out = capture_cmd_lossy("echo hello");
    assert_eq!(out.trim(), "hello");
}

#[test]
fn test_split_by_newline() {
    let input = b"hi\nthere".to_vec();
    let lines = read_utf8_lines_lossy(&input);
    assert_eq!(lines[0], "hi");
}
