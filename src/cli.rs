use clap::Parser;

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
        help = "Include the current kubeconfig file at ~/.kube/config"
    )]
    pub current: bool,

    #[arg(
        short,
        long = "dry-run",
        help = "Print the merged kubeconfig to stdout"
    )]
    pub dry_run: bool,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Specify output path for merged kubeconfig"
    )]
    pub path: Option<String>,
}
