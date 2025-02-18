mod interfaces;
mod utils;

use crate::utils::errors::KubeMergeError;
use clap::Parser;
use interfaces::cli::{load_kubeconfig_files, validate_input};
use interfaces::cli::{Cli, CliOptions};
use std::fs;
use utils::merger::merge_kubeconfig_contents;
use utils::saver::validate_output_path;

#[cfg(feature = "cli")]
fn main() {
    let cli = Cli::parse();
    let options = CliOptions {
        files: cli.files,
        output_path: cli.output,
    };

    match merge_kubeconfigs(options) {
        Ok(_) => std::process::exit(0),
        Err(e) => match e {
            KubeMergeError::InsufficientFiles(msg)
            | KubeMergeError::FileNotFound(msg)
            | KubeMergeError::ParseError(msg)
            | KubeMergeError::WriteError(msg)
            | KubeMergeError::NoContent(msg)
            | KubeMergeError::UserCancelled(msg) => eprintln!("Error: {}", msg),
            _ => eprintln!("An unknown error occurred"),
        },
    }
}

pub fn merge_kubeconfigs(options: CliOptions) -> Result<String, KubeMergeError> {
    let files = options.files.clone();

    validate_input(&files)?;

    let contents = load_kubeconfig_files(&files)?;

    let merged_yaml = merge_kubeconfig_contents(&contents)
        .map_err(|e| KubeMergeError::ParseError(format!("Failed to merge kubeconfigs: {:?}", e)))?;

    if let Some(output_path) = options.output_path {
        validate_output_path(&output_path)?;

        fs::write(&output_path, &merged_yaml).map_err(|e| {
            KubeMergeError::WriteError(format!("Failed to write the merged file: {}", e))
        })?;

        println!("✓ Merged kubeconfig saved to: {}", output_path);
        Ok(output_path)
    } else {
        println!("{}", merged_yaml);
        Ok("Merged kubeconfig displayed in terminal".to_string())
    }
}
