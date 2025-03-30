use std::process;

pub fn merge(branch_name: &str) -> Result<(), String> {
    let git_path = crate::flow::git::which_git().map_err(|e| e.to_string())?;

    let output = process::Command::new(&git_path)
        .arg("merge")
        .arg("--no-ff")
        .arg(branch_name)
        .output()
        .map_err(|e| format!("Failed to execute Git command: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Failed to merge branch '{}': {}",
            branch_name, error_msg
        ));
    }

    Ok(())
}
