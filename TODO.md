# Morsel — TODO

> **A blazing-fast, local-first clipboard manager for developers.**

Morsel turns the operating system clipboard into a searchable, reusable local memory layer — built with Rust, designed for developers, and private by default.

---

## 🧭 Project Roadmap

```text
Phase 01 ─ Core Clipboard
    ↓
Phase 02 ─ Search & Storage
    ↓
Phase 03 ─ Developer Experience
    ↓
Phase 04 ─ Security & Privacy
    ↓
Phase 05 ─ Cross-Platform
    ↓
Phase 06 ─ Performance
    ↓
Phase 07 ─ Extensibility
    ↓
v1.0 ─ Stable Release
```

---

# Phase 01 — Core Clipboard Engine

## 🧱 Project Foundation

* [ ] Initialize Rust workspace
* [ ] Define workspace crate structure
* [ ] Configure Cargo profiles
* [ ] Configure release optimizations
* [ ] Add project-wide lint configuration
* [ ] Configure `rustfmt`
* [ ] Configure `clippy`
* [ ] Add CI workflow
* [ ] Add automated formatting checks
* [ ] Add automated lint checks
* [ ] Add automated test checks
* [ ] Add minimum supported Rust version policy
* [ ] Document development environment
* [ ] Document local build instructions

### Proposed Workspace

```text
morsel/
├── crates/
│   ├── morsel-core/
│   ├── morsel-storage/
│   ├── morsel-search/
│   ├── morsel-clipboard/
│   ├── morsel-platform/
│   ├── morsel-daemon/
│   ├── morsel-cli/
│   └── morsel-tui/
│
├── tests/
├── benches/
├── docs/
└── Cargo.toml
```

---

# Phase 02 — Clipboard Core

## 📋 Clipboard Monitoring

* [ ] Implement clipboard abstraction
* [ ] Define `ClipboardProvider` trait
* [ ] Implement clipboard polling
* [ ] Detect clipboard changes
* [ ] Prevent duplicate clipboard entries
* [ ] Handle clipboard access errors
* [ ] Handle clipboard ownership changes
* [ ] Handle empty clipboard content
* [ ] Handle unsupported clipboard formats
* [ ] Add clipboard event tests
* [ ] Add clipboard stress tests

---

## 📦 Clipboard Content Model

Define a unified clipboard representation.

* [ ] Design `ClipboardItem`
* [ ] Design unique item IDs
* [ ] Store creation timestamp
* [ ] Store last-used timestamp
* [ ] Store content type
* [ ] Store content size
* [ ] Store source metadata
* [ ] Store favorite state
* [ ] Store tags
* [ ] Store collection associations
* [ ] Store expiration metadata

### Content Types

* [ ] Plain text
* [ ] Rich text
* [ ] URL
* [ ] Image
* [ ] File
* [ ] Unknown / unsupported content

---

# Phase 03 — Local Storage

## 💾 Storage Layer

* [ ] Select embedded storage backend
* [ ] Design storage schema
* [ ] Implement database initialization
* [ ] Implement schema migrations
* [ ] Implement clipboard item insertion
* [ ] Implement clipboard item retrieval
* [ ] Implement clipboard item deletion
* [ ] Implement bulk deletion
* [ ] Implement favorite state persistence
* [ ] Implement tags persistence
* [ ] Implement collection persistence
* [ ] Implement timestamps
* [ ] Implement retention metadata
* [ ] Add storage integrity tests
* [ ] Add migration tests

---

## 🗂️ Blob Storage

For binary clipboard content:

* [ ] Design blob storage layout
* [ ] Store image metadata
* [ ] Store image data
* [ ] Store file metadata
* [ ] Store file references where appropriate
* [ ] Detect duplicate blobs
* [ ] Implement blob cleanup
* [ ] Implement orphaned blob cleanup
* [ ] Test large binary content
* [ ] Test storage recovery

---

# Phase 04 — Search Engine

## 🔎 Basic Search

* [ ] Implement text normalization
* [ ] Implement tokenization
* [ ] Implement full-text indexing
* [ ] Implement exact search
* [ ] Implement partial search
* [ ] Implement case-insensitive search
* [ ] Implement search ranking
* [ ] Implement result pagination
* [ ] Implement search by content type
* [ ] Implement search by date
* [ ] Implement search by collection
* [ ] Implement search by tag
* [ ] Implement search by favorite status

---

## ⚡ Fuzzy Search

* [ ] Design fuzzy matching strategy
* [ ] Implement typo-tolerant search
* [ ] Implement prefix matching
* [ ] Implement ranking improvements
* [ ] Benchmark fuzzy search
* [ ] Optimize large clipboard histories

### Search Benchmark Targets

* [ ] Benchmark 10,000 items
* [ ] Benchmark 100,000 items
* [ ] Benchmark 1,000,000 items
* [ ] Measure average search latency
* [ ] Measure P95 search latency
* [ ] Measure memory consumption
* [ ] Measure index size

Target:

> Search should feel instantaneous even with a large clipboard history.

---

# Phase 05 — Morsel CLI

## 🖥️ Command-Line Interface

Implement:

```bash
morsel
```

* [ ] Create CLI entry point
* [ ] Add command parser
* [ ] Add global configuration
* [ ] Add consistent error messages
* [ ] Add shell exit codes
* [ ] Add shell completion

---

## 📚 History Commands

```bash
morsel list
morsel search <query>
morsel get <id>
morsel delete <id>
```

* [ ] `morsel list`
* [ ] `morsel search`
* [ ] `morsel get`
* [ ] `morsel delete`
* [ ] `morsel clear`
* [ ] `morsel stats`

---

## ⭐ Favorites

```bash
morsel pin <id>
morsel unpin <id>
morsel favorites
```

* [ ] Implement pinning
* [ ] Implement unpinning
* [ ] List favorites
* [ ] Search favorites

---

## 📁 Collections

```bash
morsel collection list
morsel collection create rust
morsel collection delete rust
```

* [ ] Create collection
* [ ] Rename collection
* [ ] Delete collection
* [ ] List collections
* [ ] Add item to collection
* [ ] Remove item from collection
* [ ] Search collection

---

## 🏷️ Tags

```bash
morsel tag add <id> rust
morsel tag remove <id> rust
morsel tag list
```

* [ ] Add tags
* [ ] Remove tags
* [ ] List tags
* [ ] Search by tag
* [ ] Support multiple tags per item

---

# Phase 06 — TUI

## 🎛️ Terminal User Interface

Create the primary developer interface.

* [ ] Design Morsel TUI
* [ ] Implement application state
* [ ] Implement search input
* [ ] Implement result list
* [ ] Implement item preview
* [ ] Implement keyboard navigation
* [ ] Implement item selection
* [ ] Implement paste action
* [ ] Implement delete action
* [ ] Implement pin action
* [ ] Implement collection navigation
* [ ] Implement tag navigation
* [ ] Implement history navigation
* [ ] Implement empty states
* [ ] Implement loading states
* [ ] Implement error states

### Keyboard Navigation

* [ ] `↑` / `↓` — Navigate
* [ ] `Enter` — Paste
* [ ] `Esc` — Close
* [ ] `Ctrl + K` — Search
* [ ] `Delete` — Delete
* [ ] `P` — Pin
* [ ] `T` — Add tag
* [ ] `C` — Collection
* [ ] `?` — Help

---

# Phase 07 — Global Hotkeys

## ⌨️ Quick Paste

* [ ] Design global hotkey abstraction
* [ ] Implement global hotkey listener
* [ ] Configure default hotkey
* [ ] Allow custom hotkeys
* [ ] Open TUI from hotkey
* [ ] Restore previous clipboard after selection
* [ ] Automatically paste selected item
* [ ] Handle hotkey conflicts
* [ ] Add platform-specific implementations

Default:

```text
Ctrl + Shift + V
```

---

# Phase 08 — Developer Experience

## 🧠 Smart Clipboard Detection

Automatically identify common clipboard content.

* [ ] URL detection
* [ ] Email detection
* [ ] UUID detection
* [ ] IP address detection
* [ ] Git commit hash detection
* [ ] JSON detection
* [ ] YAML detection
* [ ] XML detection
* [ ] Shell command detection
* [ ] SQL detection
* [ ] Code detection
* [ ] Markdown detection
* [ ] JWT detection

---

## 🏷️ Automatic Metadata

* [ ] Detect content type
* [ ] Detect probable programming language
* [ ] Detect URL domain
* [ ] Detect source application where supported
* [ ] Detect content size
* [ ] Generate searchable metadata

---

# Phase 09 — Privacy & Security

## 🔒 Privacy Architecture

* [ ] Ensure local-only storage by default
* [ ] Ensure no mandatory network connection
* [ ] Ensure no account requirement
* [ ] Ensure no telemetry
* [ ] Ensure no analytics
* [ ] Document privacy architecture
* [ ] Document all network behavior
* [ ] Add privacy-focused configuration

---

## 🛡️ Sensitive Content Detection

Implement heuristic detection for potentially sensitive clipboard data.

* [ ] Detect API keys
* [ ] Detect access tokens
* [ ] Detect JWTs
* [ ] Detect private keys
* [ ] Detect bearer tokens
* [ ] Detect common secret formats
* [ ] Add configurable detection rules
* [ ] Add false-positive handling
* [ ] Allow users to disable detection

> Sensitive-content detection must be treated as a heuristic, not a security guarantee.

---

## 🔐 Encryption

* [ ] Design encrypted storage
* [ ] Select cryptographic primitives
* [ ] Implement key management
* [ ] Encrypt sensitive clipboard content
* [ ] Encrypt local metadata where appropriate
* [ ] Implement key rotation strategy
* [ ] Implement encrypted backup
* [ ] Add encryption tests
* [ ] Document threat model

---

## ⏱️ Retention Policies

* [ ] Implement item expiration
* [ ] Implement configurable retention period
* [ ] Support 1-hour retention
* [ ] Support 1-day retention
* [ ] Support 7-day retention
* [ ] Support 30-day retention
* [ ] Support unlimited retention
* [ ] Add automatic cleanup
* [ ] Add sensitive-item expiration
* [ ] Add storage cleanup tests

---

# Phase 10 — Import & Export

## 📤 Export

```bash
morsel export backup.morsel
```

* [ ] Design backup format
* [ ] Export text items
* [ ] Export metadata
* [ ] Export collections
* [ ] Export tags
* [ ] Export favorites
* [ ] Export binary content
* [ ] Add optional encryption
* [ ] Validate backup integrity

---

## 📥 Import

```bash
morsel import backup.morsel
```

* [ ] Import clipboard items
* [ ] Import metadata
* [ ] Import collections
* [ ] Import tags
* [ ] Import favorites
* [ ] Handle duplicate items
* [ ] Handle incompatible versions
* [ ] Add migration support

---

# Phase 11 — Cross-Platform Support

## 🐧 Linux

* [ ] Implement Linux clipboard integration
* [ ] Support X11
* [ ] Support Wayland
* [ ] Implement global hotkeys
* [ ] Implement desktop autostart
* [ ] Test Ubuntu
* [ ] Test Debian
* [ ] Test Fedora
* [ ] Test Arch Linux

---

## 🍎 macOS

* [ ] Implement macOS clipboard integration
* [ ] Implement global hotkeys
* [ ] Implement application lifecycle
* [ ] Implement launch-at-login
* [ ] Test Intel Macs
* [ ] Test Apple Silicon
* [ ] Package application

---

## 🪟 Windows

* [ ] Implement Windows clipboard integration
* [ ] Implement global hotkeys
* [ ] Implement background process
* [ ] Implement startup behavior
* [ ] Test Windows 10
* [ ] Test Windows 11
* [ ] Package executable
* [ ] Build installer

---

# Phase 12 — Background Daemon

## ⚙️ Morsel Daemon

```bash
morseld
```

* [ ] Implement daemon process
* [ ] Implement IPC protocol
* [ ] Implement daemon lifecycle
* [ ] Implement start command
* [ ] Implement stop command
* [ ] Implement restart command
* [ ] Implement status command
* [ ] Detect daemon availability
* [ ] Handle daemon crashes
* [ ] Implement graceful shutdown

---

## 🔌 IPC

* [ ] Define IPC protocol
* [ ] Define request format
* [ ] Define response format
* [ ] Implement local communication
* [ ] Authenticate local clients where required
* [ ] Handle disconnected clients
* [ ] Add IPC tests

---

# Phase 13 — Performance

## ⚡ Startup Performance

* [ ] Measure cold startup
* [ ] Measure warm startup
* [ ] Reduce unnecessary allocations
* [ ] Reduce initialization overhead
* [ ] Optimize configuration loading
* [ ] Optimize database initialization

---

## 🧠 Memory Usage

* [ ] Profile memory usage
* [ ] Reduce unnecessary copies
* [ ] Optimize clipboard buffers
* [ ] Optimize image handling
* [ ] Optimize search index memory
* [ ] Add memory benchmarks
* [ ] Test large clipboard histories

---

## 💾 Storage Performance

* [ ] Benchmark insert throughput
* [ ] Benchmark retrieval
* [ ] Benchmark deletion
* [ ] Benchmark database startup
* [ ] Benchmark database growth
* [ ] Optimize database indexes
* [ ] Optimize blob storage
* [ ] Optimize cleanup

---

## 🔍 Search Performance

* [ ] Benchmark exact search
* [ ] Benchmark fuzzy search
* [ ] Benchmark prefix search
* [ ] Benchmark filtered search
* [ ] Benchmark large histories
* [ ] Optimize ranking
* [ ] Reduce search allocations

---

# Phase 14 — Testing

## 🧪 Unit Tests

* [ ] Core data model tests
* [ ] Clipboard tests
* [ ] Storage tests
* [ ] Search tests
* [ ] Tag tests
* [ ] Collection tests
* [ ] Retention tests
* [ ] Sensitive content detection tests
* [ ] Configuration tests

---

## 🔗 Integration Tests

* [ ] Clipboard → storage
* [ ] Storage → search
* [ ] Search → paste
* [ ] CLI → daemon
* [ ] TUI → daemon
* [ ] Import → storage
* [ ] Export → import
* [ ] Retention → cleanup

---

## 💥 Failure Testing

* [ ] Database unavailable
* [ ] Database corrupted
* [ ] Storage permission denied
* [ ] Clipboard unavailable
* [ ] Clipboard format unsupported
* [ ] Large clipboard item
* [ ] Large image
* [ ] Large clipboard history
* [ ] Invalid configuration
* [ ] Interrupted write
* [ ] Unexpected daemon termination

---

## 🔄 Recovery Testing

* [ ] Database recovery
* [ ] Partial write recovery
* [ ] Interrupted import recovery
* [ ] Interrupted export recovery
* [ ] Orphaned blob recovery
* [ ] Configuration recovery

---

# Phase 15 — Benchmark Suite

Create a reproducible benchmark suite.

* [ ] Clipboard insertion benchmark
* [ ] Clipboard retrieval benchmark
* [ ] Search benchmark
* [ ] Fuzzy search benchmark
* [ ] Delete benchmark
* [ ] Bulk insertion benchmark
* [ ] Storage benchmark
* [ ] Memory benchmark
* [ ] Startup benchmark
* [ ] Large-history benchmark

Benchmark datasets:

* [ ] 10K clipboard items
* [ ] 100K clipboard items
* [ ] 1M clipboard items
* [ ] Mixed text and URLs
* [ ] Code-heavy dataset
* [ ] Large JSON dataset
* [ ] Mixed binary dataset

Document results in:

```text
docs/benchmarks/
```

---

# Phase 16 — Documentation

## 📖 Documentation

* [ ] Installation guide
* [ ] Quick start
* [ ] CLI documentation
* [ ] TUI documentation
* [ ] Configuration reference
* [ ] Storage documentation
* [ ] Search documentation
* [ ] Privacy documentation
* [ ] Security documentation
* [ ] Troubleshooting guide
* [ ] FAQ
* [ ] Development guide
* [ ] Architecture documentation
* [ ] Benchmark documentation
* [ ] Contribution guide

---

## 🏗️ Architecture Documentation

Document:

```text
Clipboard Layer
       ↓
Core
       ↓
Storage
       ↓
Search
       ↓
Daemon
       ↓
CLI / TUI
       ↓
Platform Integration
```

* [ ] Architecture overview
* [ ] Crate responsibilities
* [ ] Data flow
* [ ] Clipboard lifecycle
* [ ] Search lifecycle
* [ ] Storage lifecycle
* [ ] IPC architecture
* [ ] Platform abstraction
* [ ] Security model

---

# Phase 17 — Developer Tooling

* [ ] Add `cargo fmt` CI
* [ ] Add `cargo clippy` CI
* [ ] Add `cargo test` CI
* [ ] Add release builds
* [ ] Add benchmark CI where appropriate
* [ ] Add dependency auditing
* [ ] Add security auditing
* [ ] Add code coverage
* [ ] Add documentation checks
* [ ] Add cross-platform build matrix

---

# Phase 18 — Packaging & Distribution

## 📦 Binary Releases

* [ ] Linux x86_64
* [ ] Linux ARM64
* [ ] macOS x86_64
* [ ] macOS ARM64
* [ ] Windows x86_64
* [ ] Windows ARM64 where practical

---

## 📥 Installation

Support:

```bash
cargo install morsel
```

* [ ] Publish crates
* [ ] Provide prebuilt binaries
* [ ] Provide install script
* [ ] Provide package manager instructions
* [ ] Provide checksums
* [ ] Provide signed releases where practical

---

# Phase 19 — Configuration

## ⚙️ Configuration File

Design:

```text
morsel.toml
```

Possible configuration:

```toml
[clipboard]
enabled = true

[history]
retention = "7d"
max_items = 100000

[search]
fuzzy = true

[security]
detect_secrets = true
encrypt_sensitive = true

[hotkey]
open = "Ctrl+Shift+V"
```

Tasks:

* [ ] Define configuration schema
* [ ] Implement config parser
* [ ] Implement defaults
* [ ] Implement validation
* [ ] Implement environment overrides
* [ ] Implement configuration migration
* [ ] Document configuration

---

# Phase 20 — Optional Extensibility

## 🧩 Plugin Architecture

Only consider this after the core is stable.

* [ ] Define plugin goals
* [ ] Define plugin API
* [ ] Define plugin lifecycle
* [ ] Define plugin permissions
* [ ] Define plugin isolation
* [ ] Define plugin configuration
* [ ] Prototype plugin system

Potential plugins:

```text
Clipboard Providers
Search Providers
Content Detectors
Exporters
Importers
Actions
Integrations
```

---

# Phase 21 — Optional Local AI

> **Do not make AI a core dependency.**

If local AI is introduced:

* [ ] Define optional AI architecture
* [ ] Keep AI completely opt-in
* [ ] Support local inference
* [ ] Avoid mandatory cloud APIs
* [ ] Add local summarization
* [ ] Add semantic clipboard search
* [ ] Add code explanation
* [ ] Add content transformation
* [ ] Add local classification

Potential commands:

```bash
morsel ai explain <id>
morsel ai summarize <id>
morsel ai clean <id>
```

AI must remain an optional layer:

```text
                 Morsel Core
                     │
             ┌───────┴───────┐
             │               │
          No AI           Local AI
             │               │
             └───────┬───────┘
                     ▼
                 Same Storage
```

---

# Phase 22 — Release Preparation

## 🚀 v0.1.0

* [ ] Clipboard monitoring
* [ ] Local storage
* [ ] Clipboard history
* [ ] Basic search
* [ ] CLI
* [ ] TUI
* [ ] Delete
* [ ] Pin
* [ ] Linux support
* [ ] Basic documentation
* [ ] Basic CI

---

## 🚀 v0.2.0

* [ ] Fuzzy search
* [ ] Collections
* [ ] Tags
* [ ] Smart type detection
* [ ] Import/export
* [ ] Retention policies
* [ ] macOS support
* [ ] Windows support

---

## 🚀 v0.3.0

* [ ] Encryption
* [ ] Sensitive content detection
* [ ] Improved daemon
* [ ] Improved IPC
* [ ] Advanced search
* [ ] Performance optimization
* [ ] Full benchmark suite

---

# Phase 23 — v1.0

## 🎯 Stability

* [ ] Stable storage format
* [ ] Stable configuration format
* [ ] Stable CLI interface
* [ ] Stable IPC protocol
* [ ] Migration system
* [ ] Backward compatibility policy
* [ ] Comprehensive test coverage
* [ ] Cross-platform testing
* [ ] Security review
* [ ] Performance review

---

## 📊 v1.0 Quality Goals

* [ ] Instant clipboard capture
* [ ] Instant search
* [ ] Low idle CPU usage
* [ ] Low idle memory usage
* [ ] No mandatory network connection
* [ ] No mandatory account
* [ ] No telemetry
* [ ] Reliable background operation
* [ ] Safe storage
* [ ] Predictable data retention

---

# 🌟 Launch Preparation

## GitHub

* [ ] Write polished README
* [ ] Add project logo
* [ ] Add screenshots
* [ ] Add terminal demo
* [ ] Add feature overview
* [ ] Add architecture diagram
* [ ] Add benchmarks
* [ ] Add installation instructions
* [ ] Add contributing guide
* [ ] Add code of conduct
* [ ] Add security policy
* [ ] Add issue templates
* [ ] Add pull request template
* [ ] Add GitHub Discussions

---

## 📣 Project Presentation

* [ ] Create demo GIF
* [ ] Create terminal recording
* [ ] Create feature showcase
* [ ] Create benchmark showcase
* [ ] Create privacy architecture diagram
* [ ] Create architecture overview
* [ ] Create release announcement

---

# 🧹 Quality Checklist

Before every release:

* [ ] `cargo fmt --check`
* [ ] `cargo clippy --all-targets --all-features -- -D warnings`
* [ ] `cargo test --workspace`
* [ ] `cargo doc --workspace`
* [ ] Run integration tests
* [ ] Run benchmarks
* [ ] Check memory usage
* [ ] Check binary size
* [ ] Check startup time
* [ ] Check search latency
* [ ] Check storage integrity
* [ ] Test clipboard recovery
* [ ] Test daemon recovery
* [ ] Test import/export
* [ ] Test retention cleanup
* [ ] Review security-sensitive code

---

# 🗺️ Long-Term Vision

The long-term goal is not to build another clipboard history viewer.

Morsel should become:

```text
                    Morsel
                       │
            Local Memory Layer
                       │
        ┌──────────────┼──────────────┐
        │              │              │
        ▼              ▼              ▼
     Clipboard      Search        Organization
        │              │              │
        └──────────────┼──────────────┘
                       │
                       ▼
                  Fast Retrieval
                       │
                       ▼
                  Instant Reuse
```

The core experience should always remain simple:

```text
COPY
  ↓
REMEMBER
  ↓
SEARCH
  ↓
PASTE
```

---

# ⭐ North Star

Morsel should make users stop asking:

> **“What did I copy earlier?”**

and start thinking:

> **“Morsel probably has it.”**

---

## Current Priority

```text
[ ] Rust workspace
[ ] Clipboard abstraction
[ ] Clipboard watcher
[ ] Clipboard item model
[ ] Local storage
[ ] Basic search
[ ] CLI
[ ] TUI
[ ] Global hotkey
[ ] First working prototype
```

> **Build the smallest useful Morsel first. Optimize later. Expand only when the core experience is solid.**
