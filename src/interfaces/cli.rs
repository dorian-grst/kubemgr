use std::fs;
use std::path::PathBuf;
use clap::Parser;
use crate::utils::merger::KubeconfigContent;

#[derive(Debug)]
pub struct CliOptions {
    pub files: Vec<String>,
    pub output_path: Option<String>,
}

#[derive(Parser, Debug)]
#[command(name = "kubemgr")]
#[command(about = "The fastest way to merge Kubernetes configuration files 🏎.")]
pub struct Cli {
    #[arg(
        required = true,
        value_name = "FILES",
        help = "Kubeconfig files to merge"
    )]
    pub files: Vec<String>,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Specify output path for merged kubeconfig"
    )]
    pub output: Option<String>,
}

/// Validates the input files and options
pub fn validate_input(files: &[String]) -> Result<(), String> {
    if files.len() < 2 {
        return Err("✕ At least two kubeconfig files must be provided".into());
    }

    for file in files {
        if !PathBuf::from(file).exists() {
            return Err(format!("✕ File does not exist: {}", file));
        }
    }

    Ok(())
}

/// Load kubeconfig files and convert them to KubeconfigContent
pub fn load_kubeconfig_files(paths: &[String]) -> Result<Vec<KubeconfigContent>, String> {
    let mut contents = Vec::new();

    for path in paths {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("✕ Failed to read kubeconfig file {}: {}", path, e))?;

        contents.push(KubeconfigContent { content });
    }

    Ok(contents)
}