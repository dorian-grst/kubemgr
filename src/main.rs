mod cli;
mod merger;
mod utils;

use clap::Parser;
use cli::Cli;
use merger::merge_kubeconfigs;
use std::process;

fn main() {
    let cli = Cli::parse();
    let options = utils::MergeOptions {
        files: cli.files,
        include_current: cli.current,
        dry_run: cli.dry_run,
        output_path: cli.path,
    };
    // Call the merge_kubeconfigs function with the provided files and other options
    match merge_kubeconfigs(options) {
        Ok(result) => {
            if cli.dry_run {
                println!("{}", result);
            } else {
                println!("✓ Merged kubeconfig saved to: {}", result);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
