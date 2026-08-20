<div align="center">

# 🥪 morsel

### A blazing-fast, encrypted, local-first clipboard manager built for developers.

**Instant Fuzzy Search · Zero Cloud · Local AES-256-GCM · Developer-Aware Syntax Highlighting · Zero-Dependency Binary**

<p>
  <a href="#-why-morsel">Why morsel</a> ·
  <a href="#-key-features">Key Features</a> ·
  <a href="#-architecture">Architecture</a> ·
  <a href="#-quick-start">Quick Start</a> ·
  <a href="#-cli--tui-usage">Usage</a> ·
  <a href="#-benchmarks">Benchmarks</a>
</p>

<p>
  <img src="https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge" alt="MIT License">
  <img src="https://img.shields.io/badge/Language-Pure_Rust-DEA584.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Pure Rust">
  <img src="https://img.shields.io/badge/Storage-Local_Encrypted-green.svg?style=for-the-badge" alt="Encrypted Storage">
  <img src="https://img.shields.io/badge/Memory_Footprint-<5MB-orange.svg?style=for-the-badge" alt="Low Memory">
</p>

</div>

---

## ⚡ What is morsel?

**`morsel`** is a native, ultra-lightweight clipboard manager written in 100% pure Rust. It operates entirely on-device, capturing every piece of text, code snippet, terminal command, and sensitive token you copy, storing it securely in an append-only local binary journal with instant full-text and fuzzy search.

Traditional clipboard utilities are either bloated Electron apps consuming hundreds of megabytes of RAM, or cloud-synced services that inadvertently upload your proprietary code and API keys. 

`morsel` enforces a strictly deterministic, privacy-first paradigm:

> **Your clipboard never touches the internet. Zero background telemetry. Single binary under 4MB.**

---

## 🎯 Why morsel?

```text
Standard Clipboard                        morsel Workflow
───────────────────                       ───────────────
Copy "secret_key"                          Copy "secret_key"
       │                                          │
Copy "new_text"                            Copy "new_text"
       │                                          │
❌ "secret_key" is lost forever!           ✅ Automatically indexed & encrypted locally
                                                  │
                                           Ctrl + Shift + V (or `morsel search`)
                                                  │
                                           Instant restore in < 2ms