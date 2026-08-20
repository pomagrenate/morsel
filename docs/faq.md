# FAQ

## General

### What is Morsel?

Morsel is a clipboard manager for Linux, macOS, and Windows. It stores your clipboard history, allows you to search it, and provides powerful features like tagging, collections, and sensitive content detection.

### Is Morsel free?

Yes, Morsel is open source and free to use.

### What platforms does Morsel support?

Morsel supports:
- Linux (X11 and Wayland)
- macOS
- Windows

### Does Morsel work offline?

Yes, Morsel works entirely offline and requires no internet connection.

## Installation

### How do I install Morsel?

```bash
cargo install morsel
```

See the [Installation Guide](installation.md) for more options.

### What are the system requirements?

- Rust 1.70 or later
- Platform-specific clipboard libraries (see Installation Guide)
- ~50MB disk space
- ~100MB RAM (varies with history size)

### Can I install Morsel without Rust?

Yes, prebuilt binaries are available for Linux, macOS, and Windows. See the [Installation Guide](installation.md).

## Usage

### How do I start Morsel?

Start the daemon:

```bash
morseld start
```

The daemon runs in the background and monitors your clipboard.

### How do I view my clipboard history?

```bash
morsel list
```

### How do I search my clipboard history?

```bash
morsel search "query"
```

### How do I copy an item from history?

```bash
morsel copy <id>
```

### How do I delete an item?

```bash
morsel delete <id>
```

### How do I clear all history?

```bash
morsel clear
```

⚠️ This cannot be undone.

## Features

### What is fuzzy search?

Fuzzy search allows you to find items even with typos or partial matches. For example, "gthub" will match "github".

### How do tags work?

Tags allow you to organize your clipboard history:

```bash
morsel tag add <id> work important
morsel list --tag work
```

### What are collections?

Collections are groups of clipboard items:

```bash
morsel collection create "Project X"
morsel collection add <collection-id> <item-id>
```

### What are favorites?

Favorites are important items you want to keep:

```bash
morsel favorite <id>
morsel list --favorites
```

### How does sensitive content detection work?

Morsel detects sensitive content like API keys, passwords, and tokens. You can configure it in the [Configuration Reference](configuration.md).

## Privacy and Security

### Is my clipboard history sent to the cloud?

No. All data is stored locally on your machine.

### Can Morsel access my clipboard without permission?

Morsel only accesses the clipboard when the daemon is running and you have started it.

### Does Morsel collect telemetry?

No. Morsel does not collect any telemetry or analytics.

### Is my clipboard history secure?

Morsel stores data locally with user-only file permissions. For maximum security, use full-disk encryption and enable sensitive content detection.

### Can I encrypt my clipboard history?

Currently, encryption is not built-in, but you can encrypt backup files using external tools like GPG.

## Configuration

### Where is the configuration file?

- Linux/macOS: `~/.config/morsel/config.toml`
- Windows: `%APPDATA%\morsel\config.toml`

### How do I change the configuration?

```bash
morsel config edit
```

Or edit the file directly.

### How do I change the retention policy?

```toml
[retention]
max_items = 1000
max_age_days = 30
```

### How do I change the polling interval?

```toml
[clipboard]
poll_interval_ms = 50
```

### How do I enable sensitive content detection?

```toml
[privacy]
enable_sensitive_detection = true
```

## Performance

### Why is Morsel using so much memory?

Memory usage depends on:
- Number of items in history
- Search index size
- Clipboard buffer size

Reduce memory usage by:
- Decreasing `max_items` in retention config
- Decreasing `max_buffer_size` in clipboard config
- Running cleanup: `morsel cleanup --older-than 30d`

### Why is Morsel slow?

Performance issues can be caused by:
- Large database size
- Many items in search index
- Slow disk I/O

Improve performance by:
- Running cleanup
- Vacuuming the database
- Reducing retention limits

### How do I reduce database size?

```bash
morsel cleanup --older-than 30d
sqlite3 ~/.local/share/morsel/morsel.db "VACUUM;"
```

## Troubleshooting

### The daemon won't start

Check the logs:
```bash
cat ~/.local/share/morsel/morseld.log
```

See the [Troubleshooting Guide](troubleshooting.md) for more details.

### Clipboard history is empty

Check that:
- Daemon is running: `morsel status`
- Clipboard monitoring is enabled in config
- Platform-specific dependencies are installed

### Search returns no results

Try:
- Exact match: `morsel search --exact "query"`
- Remove filters: `morsel search "query"`
- Check items exist: `morsel list`

### Database is locked

1. Check for other processes accessing the database
2. Disable WAL mode in config
3. Increase connection pool size
4. Restart daemon

See the [Troubleshooting Guide](troubleshooting.md) for more details.

## Backup and Restore

### How do I backup my clipboard history?

```bash
morsel export backup.json
```

### How do I restore from backup?

```bash
morsel import backup.json
```

### How do I automate backups?

Create a cron job or systemd timer:

```bash
# Cron
0 0 * * * morsel export ~/.backups/morsel-$(date +\%Y\%m\%d).json
```

## Advanced

### Can I use Morsel programmatically?

Yes, Morsel provides an IPC interface. See the [Architecture Documentation](architecture.md) for details.

### Can I write my own storage backend?

Yes, implement the `StorageBackend` trait. See the [Storage Documentation](storage.md) for details.

### Can I add custom search algorithms?

Yes, extend the `SearchEngine`. See the [Search Documentation](search.md) for details.

### Can I run multiple instances?

Running multiple daemon instances on the same machine is not recommended as they would conflict on the IPC port. You can change the port in the configuration if needed.

## Development

### How do I build from source?

```bash
git clone https://github.com/yourusername/morsel.git
cd morsel
cargo build --release
```

### How do I run tests?

```bash
cargo test
```

### How do I run benchmarks?

```bash
cargo bench
```

See the [Development Guide](development.md) for more details.

### How do I contribute?

See the [Contribution Guide](contribution.md) for details.

## Comparison

### How does Morsel compare to other clipboard managers?

Morsel offers:
- Cross-platform support (Linux, macOS, Windows)
- Fuzzy search
- Tagging and collections
- Sensitive content detection
- Local-only storage (no cloud)
- Open source and extensible

### Why use Morsel instead of [other manager]?

Morsel is designed with:
- Privacy as a core principle
- Modern Rust implementation
- Extensible architecture
- Active development

## Limitations

### What are Morsel's limitations?

- No built-in encryption (planned)
- No cloud sync (by design)
- No mobile app (planned)
- No web interface (planned)
- No sync between machines (by design)

### Will you add [feature]?

Check the [TODO.md](../TODO.md) for planned features. Feel free to open an issue to request features.

## Support

### Where can I get help?

- Documentation: See the docs folder
- Issues: https://github.com/yourusername/morsel/issues
- Discussions: https://github.com/yourusername/morsel/discussions

### How do I report a bug?

Report bugs on GitHub Issues with:
- Morsel version
- OS version
- Steps to reproduce
- Expected behavior
- Actual behavior
- Logs (if applicable)

### How do I request a feature?

Request features on GitHub Issues with:
- Feature description
- Use case
- Proposed implementation (if known)

## License

### What license is Morsel under?

Morsel is released under the MIT License. See the LICENSE file for details.

### Can I use Morsel in commercial projects?

Yes, the MIT License allows commercial use.

## Other

### Who maintains Morsel?

Morsel is maintained by the Morsel team. See the CONTRIBUTORS file for details.

### How can I donate?

Donations are not currently accepted, but you can support the project by:
- Contributing code
- Reporting bugs
- Suggesting features
- Spreading the word

### Where can I find the source code?

https://github.com/yourusername/morsel
