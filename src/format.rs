use std::{io::Write, process::Command};

pub fn format_code(code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(code.as_bytes())?;
    }
    let output = child.wait_with_output()?;
    Ok(String::from_utf8(output.stdout)?)
}
