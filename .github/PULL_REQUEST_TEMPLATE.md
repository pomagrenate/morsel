## 🔀 Pull Request Description

### Summary of Changes
Provide a clear, concise summary of what changes were made in this PR and why.

---

### Related Issues
Fixes # (issue number)  
Closes # (issue number)  

---

### Type of Change
Please check the option(s) that apply:

- [ ] 🐛 **Bug Fix** (non-breaking change fixing an issue)
- [ ] ⚡ **New Feature** (non-breaking change adding functionality)
- [ ] 💥 **Breaking Change** (fix or feature causing existing behavior to change)
- [ ] ⚡ **Performance Improvement** (optimization to search, RAM, latency, or storage)
- [ ] 📖 **Documentation** (updates or additions to docs, README, or guides)
- [ ] 🛠️ **Refactoring / Maintenance** (code changes without feature or bug impact)

---

### Component(s) Affected
- [ ] `morsel-core`
- [ ] `morsel-storage`
- [ ] `morsel-search`
- [ ] `morsel-clipboard`
- [ ] `morsel-platform`
- [ ] `morsel-daemon`
- [ ] `morsel-cli`
- [ ] `morsel-tui`
- [ ] CI / Workflows / Documentation

---

### Developer Quality Checklist

- [ ] Code follows the Rust style guidelines and workspace lints.
- [ ] `cargo fmt --all -- --check` passes cleanly.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes without warnings.
- [ ] `cargo test --workspace --all-features` passes cleanly.
- [ ] Benchmarks have been executed if modifying performance-critical code (`cargo bench`).
- [ ] Documentation has been updated accordingly.
