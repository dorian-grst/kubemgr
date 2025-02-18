# home-override

A dependency override for the [home](https://crates.io/crates/home) crate to fix compilation issues when using the `kube` crate with the `config` feature in WebAssembly environments.

## Background

The `kube` crate (version 0.98.0) with its `config` feature depends on `kube-client`, which in turn depends on the `home` crate. When compiling to WebAssembly targets, this dependency chain causes compilation failures due to known issues with the `home` crate in non-standard environments.

## Problem Description

When trying to compile a Rust project targeting WebAssembly that uses:
- `kube` crate with the `config` feature
- Which depends on `kube-client`
- Which depends on `home`

The compilation fails due to incompatibilities between the `home` crate and WebAssembly targets. This is a known issue tracked in:
- [cargo#12297](https://github.com/rust-lang/cargo/issues/12297)
- [libs-team#372](https://github.com/rust-lang/libs-team/issues/372)
