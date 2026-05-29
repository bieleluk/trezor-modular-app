# Modular App Repository

This repository consists of:

- **Minimal Modular App**  
  See the [app README](./README.md) for details.
- **xtask Build Tool**  
  See the [xtask README](./xtask/README.md) for build and test automation.

---

## Setup

- **Nix Shell**  
  Use `nix-shell` for dependency management (Rust, Python, cargo-binutils, ARM binutils, etc.).
- **uv**  
  Used for Python-specific tasks and dependency management.

---

## Running in VS Code

- Open the repository in VS Code.
- Launch a Nix shell for a fully configured environment.
- The recommended `settings.json` for Rust Analyzer integration:

  ```jsonc
  "rust-analyzer.cargo.targetDir": true,
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.cargo.extraEnv": {
      "IS_RUST_ANALYZER": "true",
      <!-- "VIRTUAL_ENV": "${workspaceFolder}/.venv", -->
  }
  ```

This ensures Rust Analyzer and Python tooling work seamlessly with the Nix and virtual environment setup.
