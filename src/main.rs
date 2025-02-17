mod utils;
mod interfaces;

use clap::Parser;
use std::{fs};
use interfaces::cli::{CliOptions, Cli};
use utils::merger::merge_kubeconfig_contents;
use utils::saver::validate_output_path;
use interfaces::cli::{validate_input, load_kubeconfig_files};
use crate::utils::errors::KubeMergeError;

#[cfg(feature = "cli")]
fn main() {
    let cli = Cli::parse();
    let options = CliOptions {
        files: cli.files,
        output_path: cli.output,
    };

    if let Err(e) = merge_kubeconfigs(options) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
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

        fs::write(&output_path, &merged_yaml)
            .map_err(|e| KubeMergeError::WriteError(format!("Failed to write the merged file: {}", e)))?;

        println!("✓ Merged kubeconfig saved to: {}", output_path);
        Ok(output_path)
    } else {
        println!("{}", merged_yaml);
        Ok("Merged kubeconfig displayed in terminal".to_string())
    }
}