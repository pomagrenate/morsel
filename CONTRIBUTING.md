# Contributing to morsel 🥪

Thank you for your interest in contributing to **morsel**! We welcome all contributions, whether you're fixing bugs, adding new features, improving documentation, or writing benchmarks.

---

## 📜 Table of Contents

- [Code of Conduct](#-code-of-conduct)
- [Getting Started](#-getting-started)
- [Repository Architecture](#-repository-architecture)
- [Development Workflow](#-development-workflow)
- [Submitting a Pull Request](#-submitting-a-pull-request)
- [Coding Guidelines](#-coding-guidelines)

---

## 🤝 Code of Conduct

We are committed to maintaining a welcoming, respectful, and inclusive community. Please be kind, constructive, and respectful in all issues, pull requests, and discussions.

---

## 🚀 Getting Started

### Prerequisites

- **Rust**: Version `1.75.0` or higher (`rustup update stable`)
- **Git**: Latest release
- **Cargo Tools** (Optional but recommended):
  ```bash
  cargo install cargo-edit cargo-audit cargo-deny cargo-tarpaulin
  ```

### Setting Up the Repository

1. **Fork and Clone the Repository**:
   ```bash
   git clone https://github.com/pomagrenate/morsel.git
   cd morsel
   ```

2. **Verify Local Workspace**:
   ```bash
   cargo check --workspace
   ```

3. **Run Unit & Integration Tests**:
   ```bash
   cargo test --workspace --all-features
   ```

---

## 🏗️ Repository Architecture

`morsel` is structured as a modular, highly performant Rust workspace composed of 8 crates:

```text
morsel/
├── crates/
│   ├── morsel-core        # Core domain types, item models, clipboard traits
│   ├── morsel-storage     # Embedded SQLite & encrypted storage layer
│   ├── morsel-search      # High-performance, typo-tolerant fuzzy search engine
│   ├── morsel-clipboard   # Native platform clipboard capture & monitoring
│   ├── morsel-platform    # Platform-specific OS APIs (Windows, macOS, Linux)
│   ├── morsel-daemon      # Background service (morseld) with IPC support
│   ├── morsel-cli         # Command-line interface (`morsel`)
│   └── morsel-tui         # Terminal UI built with Ratatui (`morsel-tui`)
```

---

## 🛠️ Development Workflow

Before submitting changes, ensure your code satisfies all quality checks:

### 1. Code Formatting
Format all Rust source code using `rustfmt`:
```bash
cargo fmt --all -- --check
```
To automatically apply formatting:
```bash
cargo fmt --all
```

### 2. Linting
Run `clippy` with strict linting rules:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### 3. Testing
Run the complete test suite:
```bash
cargo test --workspace --all-features
```

### 4. Benchmarking
To run performance benchmarks (via Criterion):
```bash
cargo bench --workspace
```

---

## 🔀 Submitting a Pull Request

1. **Create a Feature Branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```
2. **Commit Your Changes**:
   Follow clear commit message conventions (e.g., `feat(search): add rank weighting for pinned items` or `fix(daemon): resolve IPC reconnection retry limit`).
3. **Push to Your Fork**:
   ```bash
   git push origin feature/amazing-feature
   ```
4. **Open a Pull Request**:
   Describe what your changes accomplish, attach benchmark numbers if applicable, and ensure all CI checks pass.

---

## ⚖️ License

By contributing to **morsel**, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
