# Development Guide

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Platform-specific build tools

### Clone Repository

```bash
git clone https://github.com/yourusername/morsel.git
cd morsel
```

### Build

```bash
cargo build
```

Build release version:

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

Run tests for specific crate:

```bash
cargo test -p morsel-core
cargo test -p morsel-storage
cargo test -p morsel-search
```

### Run Benchmarks

```bash
cargo bench
```

Run benchmarks for specific crate:

```bash
cargo bench -p morsel-core
cargo bench -p morsel-storage
```

## Project Structure

```
morsel/
├── crates/
│   ├── morsel-core/          # Core data models
│   ├── morsel-clipboard/     # Clipboard monitoring
│   ├── morsel-storage/       # Storage backend
│   ├── morsel-search/        # Search engine
│   ├── morsel-platform/      # Platform abstraction
│   ├── morsel-daemon/        # Background daemon
│   ├── morsel-cli/           # CLI interface
│   └── morsel-tui/           # TUI interface
├── docs/                     # Documentation
├── benches/                  # Benchmarks
├── tests/                    # Integration tests
└── Cargo.toml                # Workspace config
```

## Crate Overview

### morsel-core

**Purpose:** Core data models and shared types.

**Key types:**
- `ClipboardItem` - Primary data structure
- `Collection` - Grouping mechanism
- `ItemId` - Unique identifier
- `ContentType` - Content type detection
- `SensitiveDetection` - Sensitive content detection

**Adding new types:**
1. Define struct in `src/lib.rs`
2. Implement `Serialize`, `Deserialize` if needed
3. Add tests in `tests/`
4. Update documentation

### morsel-clipboard

**Purpose:** Clipboard monitoring and interaction.

**Key traits:**
- `ClipboardProvider` - Platform-agnostic clipboard interface

**Key types:**
- `ClipboardMonitor` - Monitors clipboard for changes
- `ClipboardEvent` - Events (NewItem, Cleared, etc.)

**Adding platform support:**
1. Implement `ClipboardProvider` trait
2. Add platform-specific dependencies in `Cargo.toml`
3. Add conditional compilation
4. Test on target platform

### morsel-storage

**Purpose:** Persistent storage.

**Key traits:**
- `StorageBackend` - Storage abstraction

**Key types:**
- `SqliteStorage` - SQLite implementation
- `StorageConfig` - Configuration

**Adding new storage backend:**
1. Implement `StorageBackend` trait
2. Add as optional feature
3. Add configuration options
4. Add tests

### morsel-search

**Purpose:** Search and indexing.

**Key types:**
- `SearchEngine` - In-memory search index
- `SearchQuery` - Query builder
- `SearchResult` - Search result

**Adding search algorithms:**
1. Extend `SearchEngine`
2. Add configuration options
3. Add benchmarks
4. Update documentation

### morsel-daemon

**Purpose:** Background service.

**Key types:**
- `Daemon` - Main daemon struct
- `IpcServer` - IPC server
- `IpcRequest` / `IpcResponse` - IPC types

**Adding IPC commands:**
1. Add variant to `IpcRequest`
2. Add variant to `IpcResponse`
3. Implement handler in daemon
4. Add CLI command
5. Add tests

### morsel-cli

**Purpose:** CLI interface.

**Key types:**
- CLI commands defined with `clap`

**Adding CLI commands:**
1. Add command to `clap` parser
2. Implement command handler
3. Add IPC call to daemon
4. Add tests
5. Update documentation

## Coding Standards

### Rust Style

Follow standard Rust style:
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust naming conventions

### Documentation

Document all public APIs:
```rust
/// Creates a new clipboard item.
///
/// # Arguments
///
/// * `content` - The clipboard content
///
/// # Returns
///
/// A new `ClipboardItem` instance
///
/// # Examples
///
/// ```
/// let item = ClipboardItem::new("test".to_string());
/// ```
pub fn new(content: String) -> Self {
    // ...
}
```

### Error Handling

Use `thiserror` for error types:
```rust
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Item not found: {0}")]
    ItemNotFound(ItemId),
}
```

### Testing

Write unit tests for all public functions:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_item_creation() {
        let item = ClipboardItem::new("test".to_string());
        assert_eq!(item.content, "test");
    }
}
```

Write integration tests in `tests/` directory.

## Testing

### Unit Tests

```bash
cargo test
```

Run specific test:
```bash
cargo test test_clipboard_item_creation
```

### Integration Tests

```bash
cargo test --test integration_test_name
```

### Test Coverage

Install tarpaulin:
```bash
cargo install cargo-tarpaulin
```

Generate coverage:
```bash
cargo tarpaulin --out Html
```

## Benchmarking

### Running Benchmarks

```bash
cargo bench
```

### Writing Benchmarks

Use Criterion:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_function(c: &mut Criterion) {
    c.bench_function("my_function", |b| {
        b.iter(|| {
            my_function(black_box(input))
        })
    });
}

criterion_group!(benches, bench_function);
criterion_main!(benches);
```

## Debugging

### Logging

Enable debug logging:
```bash
RUST_LOG=debug morseld run
```

### Debugger

Use lldb or gdb:
```bash
lldb target/debug/morseld
```

### VS Code Debugging

Create `.vscode/launch.json`:
```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug morseld",
    "program": "${workspaceFolder}/target/debug/morseld",
    "args": ["run"]
}
```

## Contributing

### Workflow

1. Fork the repository
2. Create a branch: `git checkout -b feature/my-feature`
3. Make changes
4. Run tests: `cargo test`
5. Run linter: `cargo clippy`
6. Format code: `cargo fmt`
7. Commit changes
8. Push to fork
9. Create pull request

### Commit Messages

Follow conventional commits:
```
feat: add new feature
fix: fix bug
docs: update documentation
test: add tests
refactor: refactor code
```

### Pull Request Guidelines

- Describe what the PR does
- Link to related issues
- Include tests
- Update documentation
- Ensure CI passes

## Release Process

### Version Bump

Update version in `Cargo.toml`:
```toml
[workspace.package]
version = "0.2.0"
```

### Changelog

Update `CHANGELOG.md`:
```markdown
## [0.2.0] - 2024-01-01

### Added
- New feature

### Fixed
- Bug fix
```

### Tag Release

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### Publish Crates

```bash
cargo publish -p morsel-core
cargo publish -p morsel-clipboard
# ... other crates
```

## Resources

### Rust Documentation

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Tools

- [cargo-watch](https://github.com/passcod/cargo-watch) - Watch for changes
- [cargo-edit](https://github.com/killercup/cargo-edit) - Edit Cargo.toml
- [cargo-audit](https://github.com/RustSec/cargo-audit) - Security audit

### Community

- [Rust Reddit](https://www.reddit.com/r/rust/)
- [Rust Discord](https://discord.gg/rust-lang)
- [Rust Forums](https://users.rust-lang.org/)
