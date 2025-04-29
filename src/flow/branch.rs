use crate::BranchType;
use std::process;

pub fn branch(branch_type: BranchType, branch_name: &str) -> Result<(), String> {
    let git_path = crate::flow::git::which_git().map_err(|e| e.to_string())?;

    let prefix = match branch_type {
        BranchType::Feature => "feature",
        BranchType::Release => "release",
        BranchType::Hotfix => "hotfix",
    };

    let branch = format!("{}/{}", prefix, branch_name);

    let output = process::Command::new(&git_path)
        .arg("checkout")
        .arg("-b")
        .arg(&branch)
        .output()
        .map_err(|e| format!("Failed to execute Git command: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Failed to create branch '{}': {}",
            branch, error_msg
        ));
    }

    Ok(())
}
