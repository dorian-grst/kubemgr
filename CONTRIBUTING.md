# Kubemgr Contribution Guidelines

## Welcome Contributors

We appreciate your interest in improving Kubemgr. All contributors must adhere to the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Pull Request Process

1. Submit pull requests to the `main` branch on GitHub.
2. I will:
- Evaluate your contribution
- Merge acceptable changes
- Request modifications if needed
3. Continuous Integration (CI) will automatically lint and build your code.

Commit Strategy:
- Feel free to amend previous commits or add new ones
- Final pull request will be squashed into a single commit

## Issue Tracking

Report issues, suggest improvements, and track project progress [on our GitHub Issues page](https://github.com/dorian-grst/kubemgr/issues).

Issue Labels:
- `enhancement`: New feature requests
- `bug`: Reported problems or malfunctions
- `documentation`: Documentation-related tasks
- `help wanted`: Open issues seeking community contribution
- `good first issue`: Beginner-friendly tasks

## Development Setup

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

- **Kubernetes cluster access**: Ensure you have a valid Kubernetes configuration (`~/.kube/config`) if you plan to interact with clusters.
