# Contribution Guide

Thank you for your interest in contributing to Morsel!

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- Basic knowledge of Rust
- Familiarity with the project

### Setting Up Development Environment

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/yourusername/morsel.git
   cd morsel
   ```

3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/original/morsel.git
   ```

4. Install dependencies:
   ```bash
   cargo build
   ```

5. Run tests:
   ```bash
   cargo test
   ```

## Contribution Guidelines

### Code of Conduct

Be respectful and constructive:
- Use inclusive language
- Welcome newcomers
- Focus on what is best for the community
- Show empathy towards other community members

### What to Contribute

We welcome contributions in:
- Bug fixes
- New features
- Documentation improvements
- Performance improvements
- Test coverage
- Platform support

### Before You Start

1. Check existing issues for similar work
2. Discuss large changes in an issue first
3. Follow the coding standards
4. Write tests for your changes
5. Update documentation

## Development Workflow

### Branch Naming

Use descriptive branch names:
```
feature/add-dark-mode
fix/clipboard-crash
docs/update-installation
test/add-storage-tests
```

### Making Changes

1. Create a new branch:
   ```bash
   git checkout -b feature/my-feature
   ```

2. Make your changes

3. Test your changes:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

4. Commit your changes:
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

5. Push to your fork:
   ```bash
   git push origin feature/my-feature
   ```

### Commit Messages

Follow conventional commits:

```
<type>: <subject>

<body>

<footer>
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style changes (formatting)
- `refactor` - Code refactoring
- `test` - Adding or updating tests
- `chore` - Maintenance tasks
- `perf` - Performance improvements

**Examples:**
```
feat: add fuzzy search to CLI

Implements fuzzy search for clipboard history search
with configurable sensitivity threshold.

Closes #123
```

```
fix: resolve database lock on Windows

The WAL mode was causing issues on Windows filesystems.
Disabled WAL mode by default on Windows.

Fixes #456
```

## Pull Requests

### Creating a Pull Request

1. Go to the repository on GitHub
2. Click "New Pull Request"
3. Select your branch
4. Fill in the PR template

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist
- [ ] Code follows project style
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings
- [ ] Tests pass locally
- [ ] Added tests for new functionality
```

### PR Review Process

1. Automated checks run (CI)
2. Maintainers review the PR
3. Address review comments
4. Update PR as needed
5. Approval and merge

### Merging

- Squash commits for clean history
- Use "Squash and merge" button
- Delete branch after merge

## Coding Standards

### Rust Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust naming conventions
- Document public APIs

### Documentation

Document all public functions:
```rust
/// Creates a new clipboard item.
///
/// # Arguments
///
/// * `content` - The clipboard content
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
}
```

### Testing

Write tests for all new functionality:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_functionality() {
        // Test code
    }
}
```

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p morsel-core

# Specific test
cargo test test_clipboard_item_creation
```

### Test Coverage

Aim for high test coverage:
- Unit tests for all public functions
- Integration tests for workflows
- Edge case testing

### Adding Tests

1. Write test alongside code
2. Add integration tests in `tests/`
3. Test error cases
4. Test edge cases

## Documentation

### Updating Documentation

- Update relevant docs in `docs/`
- Update README if needed
- Update inline code documentation
- Add examples for new features

### Documentation Style

- Use clear, concise language
- Include code examples
- Use proper Markdown formatting
- Update table of contents if needed

## Platform Support

### Adding Platform Support

1. Implement platform-specific code
2. Add conditional compilation
3. Add platform-specific tests
4. Update documentation
5. Test on target platform

### Example: Adding Linux Support

```rust
#[cfg(target_os = "linux")]
mod linux {
    // Linux-specific implementation
}
```

## Performance

### Performance Guidelines

- Benchmark new features
- Avoid unnecessary allocations
- Use efficient algorithms
- Profile before optimizing

### Adding Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_new_feature(c: &mut Criterion) {
    c.bench_function("new_feature", |b| {
        b.iter(|| {
            new_feature(black_box(input))
        })
    });
}
```

## Security

### Security Guidelines

- Never hardcode secrets
- Validate all inputs
- Use secure defaults
- Follow security best practices

### Reporting Security Issues

Report security vulnerabilities privately:
- Email: security@morsel.dev
- Include details and reproduction steps

## Release Process

### Version Bumping

Maintainers handle version bumping before release.

### Changelog

Update CHANGELOG.md with your changes:
```markdown
## [Unreleased]

### Added
- Your new feature

### Fixed
- Bug you fixed
```

## Getting Help

### Questions

- Open an issue with "question" label
- Join discussions
- Check existing documentation

### Issues

- Search existing issues first
- Provide detailed information
- Include reproduction steps
- Specify environment details

## Recognition

Contributors are recognized in:
- CONTRIBUTORS file
- Release notes
- Project documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Flow](https://guides.github.com/introduction/flow/)

Thank you for contributing to Morsel! 🎉
