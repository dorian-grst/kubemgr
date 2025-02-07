use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use dirs;
use crate::utils::MergeOptions;

fn get_current_kubeconfig_path() -> Option<String> {
    dirs::home_dir().map(|home| home.join(".kube/config").to_string_lossy().to_string())
}

// Function to run `kubectl config view --flatten`
fn run_kubectl_view() -> Result<String, String> {
    let output = Command::new("kubectl")
        .arg("config")
        .arg("view")
        .arg("--flatten")
        .output()
        .map_err(|e| format!("✕ Failed to execute kubectl: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("✕ kubectl returned an error:\n{}", stderr));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("✕ Invalid UTF-8 output: {}", e))
}

// Core function to merge kubeconfigs
pub fn merge_kubeconfigs(options: MergeOptions) -> Result<String, String> {
    let mut files = options.files.clone();

    // Ensure at least one file is provided
    if files.is_empty() && !options.include_current {
        return Err("✕ At least two kubeconfig files must be provided, or use --current.".into());
    }

    // Add ~/.kube/config if --current is enabled
    if options.include_current {
        if let Some(current_path) = get_current_kubeconfig_path() {
            files.insert(0, current_path);
        } else {
            return Err("✕ Failed to determine the home directory.".into());
        }
    }

    let kubeconfig_paths = files.join(":");
    env::set_var("KUBECONFIG", &kubeconfig_paths);

    let merged_config = run_kubectl_view()?;

    // If --view flag is set, return the merged config as a string
    if options.dry_run {
        return Ok(merged_config);
    }

    // Determine output file path
    let merged_file = options.output_path.unwrap_or_else(|| {
        dirs::home_dir()
            // Use ~/.kube/kubeconfig-merged.yaml as the default output path
            .map(|home| home.join(".kube/config.merged").to_string_lossy().to_string())
            // Fallback to /tmp/kubeconfig-merged.yaml if home directory is not available
            .unwrap_or_else(|| "/tmp/kubeconfig-merged.yaml".to_string())
    });

    let merged_file_path = PathBuf::from(&merged_file);

    // Check if the file already exists
    if merged_file_path.exists() {
        print!("⚠ The file '{}' already exists. Do you want to overwrite it? (Y/n): ", merged_file);
        io::stdout().flush().ok();

        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation).map_err(|_| "✕ Failed to read input")?;

        let confirmation = confirmation.trim().to_lowercase();

        if !confirmation.is_empty() && confirmation != "y" {
            return Err("✕ Operation cancelled by the user.".into());
        }
    }

    // Write the merged config to the chosen file
    fs::write(&merged_file, &merged_config)
        .map_err(|e| format!("✕ Failed to write the merged file: {}", e))?;

    Ok(merged_file)
    //dry-run
}
