mod cli;
mod merger;
mod utils;

use clap::Parser;
use cli::Cli;
use merger::merge_kubeconfigs;
use std::process;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let options = utils::MergeOptions {
        files: cli.files,
        include_current: cli.current,
        dry_run: cli.dry_run,
        output_path: cli.path,
    };

    match merge_kubeconfigs(options).await {
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
