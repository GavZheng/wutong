use std::env::consts::OS;
use std::process;

pub fn which_git() -> Result<String, String> {
    let (cmd, arg) = if OS == "windows" {
        ("where", "git")
    } else {
        ("which", "git")
    };

    let output = process::Command::new(cmd)
        .arg(arg)
        .output()
        .map_err(|e| format!("Command execution failed: {}", e))?;

    if !output.status.success() {
        return Err("The Git executable could not be found".to_string());
    }

    let path_str = String::from_utf8_lossy(&output.stdout)
        .trim()
        .lines()
        .next()
        .ok_or("No valid path found")?
        .to_string();

    if path_str.is_empty() || !std::path::Path::new(&path_str).exists() {
        return Err("The found Git path is invalid".to_string());
    }

    Ok(path_str)
}
