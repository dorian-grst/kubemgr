# kubemgr

The fastest way to merge Kubernetes configuration files 🏎.

### Prerequisites

- Kubectl installed

### Installation

```shell
cargo install kubemgr
```

### Usage

```sh
Usage: kubemgr [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Kubeconfig files to merge

Options:
  -c, --current      Include the current kubeconfig file at ~/.kube/config
  -d, --dry-run      Print the merged kubeconfig to stdout
  -p, --path <FILE>  Specify output path for merged kubeconfig
  -h, --help         Print help
```

