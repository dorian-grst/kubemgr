
<div align="center">

# kubemgr

The fastest way to merge Kubernetes configuration files 🦀.

![board](https://github.com/dorian-grst/kubemgr/blob/main/examples/demo.gif?raw=true)

[![crates.io](https://img.shields.io/crates/v/kubemgr.svg)](https://crates.io/crates/kubemgr)
[![Website](https://img.shields.io/badge/website-kubemgr.com-red)](https://kubemgr.com/)

</div>

### Description

Kubemgr is a powerful CLI tool that simplifies merging multiple Kubernetes configuration files, ensuring seamless access and efficient cluster management.

### Pre-requisites

- **Rust & Cargo**: Ensure you have Rust and Cargo installed. You can install them using [rustup](https://rustup.rs/).

- **Rust version**: This project requires Rust **1.65 or later**. You can check your Rust version with:

  ```shell
  rustc --version
  ```

  If your version is outdated, update Rust with:

  ```shell
  rustup update
  ```

### Installation

```shell
cargo install kubemgr
```

### Usage

```sh
Usage: kubemgr [OPTIONS] <FILES>...

Arguments:
  <FILES>...  Kubernetes configuration files to merge

Options:
  -o, --output <FILE>  Specify output file path (prints to stdout if not specified)
  -h, --help           Print help
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