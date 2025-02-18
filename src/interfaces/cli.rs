use crate::utils::errors::KubeMergeError;
use crate::utils::merger::KubeconfigContent;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

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
        help = "Kubernetes configuration files to merge"
    )]
    pub files: Vec<String>,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Specify output file path (prints to stdout if not specified)"
    )]
    pub output: Option<String>,
}

/// Validates the input files and options
pub fn validate_input(files: &[String]) -> Result<(), KubeMergeError> {
    if files.len() < 2 {
        return Err(KubeMergeError::InsufficientFiles(
            "At least two kubeconfig files must be provided".to_string(),
        ));
    }

    for file in files {
        if !PathBuf::from(file).exists() {
            return Err(KubeMergeError::FileNotFound(format!(
                "File '{}' not found",
                file
            )));
        }
    }

    Ok(())
}

/// Load kubeconfig files and convert them to KubeconfigContent
pub fn load_kubeconfig_files(paths: &[String]) -> Result<Vec<KubeconfigContent>, KubeMergeError> {
    let mut contents = Vec::new();

    for path in paths {
        let content = fs::read_to_string(path).map_err(|error| {
            KubeMergeError::FileReadError(format!(
                "Failed to read file '{}', {}",
                path.clone(),
                error
            ))
        })?;

        contents.push(KubeconfigContent { content });
    }

    Ok(contents)
}
