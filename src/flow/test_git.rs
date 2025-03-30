#[cfg(test)]
mod tests {
    use crate::flow::git::which_git;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    struct EnvGuard {
        key: String,
        original_value: Option<String>,
    }

    impl EnvGuard {
        fn new(key: &str) -> Self {
            let original_value = env::var(key).ok();
            EnvGuard {
                key: key.to_string(),
                original_value,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.original_value {
                Some(value) => env::set_var(&self.key, value),
                None => env::remove_var(&self.key),
            }
        }
    }

    #[test]
    fn test_which_git_success() {
        let _guard = EnvGuard::new("PATH");
        let os = env::consts::OS;
        let temp_dir = tempdir().unwrap();

        let git_name = if os == "windows" { "git.exe" } else { "git" };
        let git_path = temp_dir.path().join(git_name);
        File::create(&git_path).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&git_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        let script_name = if os == "windows" {
            "where.bat"
        } else {
            "which"
        };
        let script_path = temp_dir.path().join(script_name);
        let mut script = File::create(&script_path).unwrap();

        if os == "windows" {
            writeln!(
                script,
                "@echo {}\\git.exe",
                temp_dir.path().display().to_string().replace("/", "\\")
            )
            .unwrap();
        } else {
            writeln!(script, "#!/bin/sh\necho '{}'", git_path.display()).unwrap();
            #[cfg(unix)]
            fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        let path_sep = if os == "windows" { ";" } else { ":" };
        let original_path = env::var("PATH").unwrap_or_else(|_| String::new());
        let new_path = format!("{}{}{}", temp_dir.path().display(), path_sep, original_path);
        env::set_var("PATH", &new_path);

        println!("Temporary directory: {}", temp_dir.path().display());
        println!("New PATH: {}", new_path);
        println!(
            "Script content:\n{}",
            fs::read_to_string(&script_path).unwrap()
        );

        let result = which_git();
        assert!(result.is_ok(), "Result should be Ok, got {:?}", result);
        assert_eq!(
            result.clone().as_ref().unwrap(),
            git_path.to_str().unwrap(),
            "Expected path: {}, Actual: {}",
            git_path.display(),
            result.unwrap()
        );
    }

    #[test]
    fn test_which_git_invalid_path() {
        let _guard = EnvGuard::new("PATH");
        let os = env::consts::OS;
        let temp_dir = tempdir().unwrap();

        let script_name = if os == "windows" {
            "where.bat"
        } else {
            "which"
        };
        let script_path = temp_dir.path().join(script_name);
        let mut script = File::create(&script_path).unwrap();

        let fake_path = if os == "windows" {
            r"C:\invalid\path\to\git.exe"
        } else {
            "/invalid/path/to/git"
        };

        if os == "windows" {
            writeln!(script, "@echo off\necho {}", fake_path).unwrap();
        } else {
            writeln!(script, "#!/bin/sh\necho \"{}\"", fake_path).unwrap();
            #[cfg(unix)]
            fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        env::set_var(
            "PATH",
            format!(
                "{}{}{}",
                temp_dir.path().display(),
                if os == "windows" { ";" } else { ":" },
                env::var("PATH").unwrap_or_else(|_| String::new())
            ),
        );

        let result = which_git();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "The found Git path is invalid");
    }
}
