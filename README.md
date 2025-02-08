# kubemgr

The fastest way to merge Kubernetes configuration files 🏎.

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

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>