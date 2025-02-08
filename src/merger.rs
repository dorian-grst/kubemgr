use crate::utils::MergeOptions;
use anyhow::{Context, Result};
use kube::config::Kubeconfig;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn get_current_kubeconfig_path() -> Option<String> {
    dirs::home_dir().map(|home| home.join(".kube/config").to_string_lossy().to_string())
}

/// Validates the input files and options
fn validate_input(files: &[String], include_current: bool) -> Result<(), String> {
    // Check if we have enough files
    if files.len() < 2 && !include_current {
        return Err("✕ At least two kubeconfig files must be provided, or use --current to merge with ~/.kube/config".into());
    }

    // Verify all files exist
    for file in files {
        if !PathBuf::from(file).exists() {
            return Err(format!("✕ File does not exist: {}", file));
        }
    }

    Ok(())
}

/// Prompts for user confirmation if the output file exists
fn confirm_overwrite(path: &str) -> Result<bool, String> {
    print!(
        "⚠ The file '{}' already exists. Do you want to overwrite it? (Y/n): ",
        path
    );
    io::stdout().flush().ok();

    let mut confirmation = String::new();
    io::stdin()
        .read_line(&mut confirmation)
        .map_err(|_| "✕ Failed to read input")?;

    let confirmation = confirmation.trim().to_lowercase();
    Ok(confirmation.is_empty() || confirmation == "y")
}

/// Check output path and ask for confirmation if needed
fn validate_output_path(path: &str) -> Result<(), String> {
    let output_path = PathBuf::from(path);
    if output_path.exists() && !confirm_overwrite(path)? {
        return Err("✕ Operation cancelled by the user.".into());
    }
    Ok(())
}

/// Load and merge kubeconfig files
async fn load_and_merge_kubeconfigs(paths: &[String]) -> Result<Kubeconfig> {
    // Load and merge all kubeconfig files
    let mut merged_config = None;

    for path in paths {
        let config_str = fs::read_to_string(path)
            .with_context(|| format!("Failed to read kubeconfig file: {}", path))?;

        let config: Kubeconfig = serde_yaml::from_str(&config_str)
            .with_context(|| format!("Failed to parse kubeconfig file: {}", path))?;

        match merged_config {
            None => merged_config = Some(config),
            Some(ref mut merged) => {
                // Merge clusters
                merged.clusters.extend(config.clusters);
                // Merge auth-infos (users)
                merged.auth_infos.extend(config.auth_infos);
                // Merge contexts
                merged.contexts.extend(config.contexts);
                // Keep the current context from the first file if not explicitly set
                if merged.current_context.is_none() {
                    merged.current_context = config.current_context;
                }
            }
        }
    }

    merged_config.ok_or_else(|| anyhow::anyhow!("No valid kubeconfig files found"))
}

/// Core function to merge kubeconfigs
pub async fn merge_kubeconfigs(options: MergeOptions) -> Result<String, String> {
    let mut files = options.files.clone();

    // Add ~/.kube/config if --current is enabled
    if options.include_current {
        if let Some(current_path) = get_current_kubeconfig_path() {
            files.insert(0, current_path);
        } else {
            return Err("✕ Failed to determine the home directory.".into());
        }
    }

    // Validate input files
    validate_input(&files, options.include_current)?;

    // Load and merge kubeconfigs
    let merged_config = load_and_merge_kubeconfigs(&files)
        .await
        .map_err(|e| format!("✕ Failed to merge kubeconfigs: {}", e))?;

    // Convert merged config to YAML
    let merged_yaml = serde_yaml::to_string(&merged_config)
        .map_err(|e| format!("✕ Failed to serialize merged config: {}", e))?;

    // If --dry-run flag is set, return the merged config as a string
    if options.dry_run {
        return Ok(merged_yaml);
    }

    // Determine output file path
    let merged_file = options.output_path.unwrap_or_else(|| {
        dirs::home_dir()
            .map(|home| {
                home.join(".kube/config.merged")
                    .to_string_lossy()
                    .to_string()
            })
            .unwrap_or_else(|| "/tmp/kubeconfig-merged.yaml".to_string())
    });

    // Validate output path and get confirmation if needed
    validate_output_path(&merged_file)?;

    // Write the merged config to the chosen file
    fs::write(&merged_file, merged_yaml)
        .map_err(|e| format!("✕ Failed to write the merged file: {}", e))?;

    Ok(merged_file)
}
