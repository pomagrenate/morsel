# Privacy Documentation

## Overview

Morsel is designed with privacy as a core principle. All data is stored locally on your machine, and no data is sent to external servers.

## Data Storage

### Local-Only Storage

- All clipboard history is stored in a local SQLite database
- Database location: `~/.local/share/morsel/morsel.db`
- No cloud sync or backup by default
- No telemetry or analytics collection

### Data at Rest

- Database file permissions restricted to user only
- Optional encryption support (future feature)
- Sensitive content detection available

### Data in Transit

- IPC communication uses localhost-only binding
- No network transmission of clipboard content
- Optional Unix domain socket support (Linux/macOS)

## Sensitive Content Detection

Morsel can detect and optionally handle sensitive content:

### Detected Patterns

- API keys
- Access tokens
- JWT tokens
- Private keys
- Bearer tokens
- Passwords
- Secrets

### Configuration

```toml
[privacy]
enable_sensitive_detection = true

[sensitive_detection]
enable_api_key = true
enable_password = true
confidence_threshold = 0.8
```

### Handling Sensitive Content

Options for handling detected sensitive content:
1. **Log warning** - Log detection but store normally
2. **Mark as sensitive** - Tag item as sensitive
3. **Auto-delete** - Automatically delete sensitive items (future)
4. **Exclude from search** - Don't include in search results (future)

## Clipboard Content

### What is Stored

- Text content
- Content type (URL, email, code, etc.)
- Timestamps (created, last used)
- Tags
- Collection membership
- Favorite status

### What is NOT Stored

- Clipboard ownership metadata
- Source application (unless explicitly set)
- Binary content (stored separately in blobs table)

## Data Retention

### Default Retention Policy

```toml
[retention]
max_items = 10000
max_age_days = 90
```

### Custom Retention

Configure retention based on your needs:

```toml
[retention]
max_items = 1000        # Keep only last 1000 items
max_age_days = 7          # Keep items for 7 days max
```

### Manual Cleanup

```bash
morsel cleanup --older-than 30d
morsel cleanup --expired
morsel clear  # Remove all history
```

## Export and Import

### Export

When exporting, data is written to a JSON file:

```bash
morsel export backup.json
```

The export file contains all clipboard history and can be:
- Stored locally
- Backed up to external storage
- Encrypted with your own tools

### Import

When importing, data is read from a JSON file:

```bash
morsel import backup.json
```

Import options:
- `--merge` - Merge with existing history
- Default - Replace existing history

## Data Deletion

### Deleting Individual Items

```bash
morsel delete <id>
```

### Clearing All History

```bash
morsel clear
```

⚠️ **Warning:** This cannot be undone.

### Automatic Cleanup

```bash
morsel cleanup --older-than 30d
morsel cleanup --expired
```

## Privacy Best Practices

### 1. Regular Cleanup

Regularly clean up old items:

```bash
morsel cleanup --older-than 30d
```

### 2. Enable Sensitive Detection

Detect sensitive content:

```toml
[privacy]
enable_sensitive_detection = true
```

### 3. Use Tags

Organize sensitive items with tags:

```bash
morsel tag add <id> sensitive
```

Then exclude from search:

```bash
morsel search --exclude-tag sensitive
```

### 4. Review History

Regularly review your clipboard history:

```bash
morsel list --limit 100
```

### 5. Secure Backups

Encrypt backup files:

```bash
gpg --encrypt backup.json
```

### 6. Secure Database

Set appropriate file permissions:

```bash
chmod 600 ~/.local/share/morsel/morsel.db
```

## Privacy Settings

### Disable Sensitive Detection

If you don't want sensitive content detection:

```toml
[privacy]
enable_sensitive_detection = false
```

### Adjust Confidence Threshold

Reduce false positives:

```toml
[sensitive_detection]
confidence_threshold = 0.9
```

### Whitelist Patterns

Whitelist patterns that should never be flagged:

```toml
[sensitive_detection]
whitelist_patterns = [
    "my_safe_token_[a-z]+"
]
```

### Whitelist Content

Whitelist specific content:

```toml
[sensitive_detection]
whitelist_content = [
    "my_public_api_key"
]
```

## Auditing

### View History

```bash
morsel list --limit 1000
```

### Search for Sensitive Items

```bash
morsel search --tag sensitive
```

### Export for Review

```bash
morsel export review.json
```

Review the export file for sensitive content.

## Compliance

### GDPR

Morsel can help with GDPR compliance:
- Data minimization: Configure retention limits
- Right to erasure: Use `morsel clear` or `morsel delete`
- Data portability: Use `morsel export`

### HIPAA

For healthcare environments:
- Enable sensitive detection
- Set short retention periods
- Regular cleanup
- Secure backups

### Corporate Environments

For corporate use:
- Disable sensitive detection if not needed
- Configure retention policies
- Regular backups
- Audit logs (future feature)

## Third-Party Access

### No Third-Party Access

Morsel does not:
- Send data to external servers
- Include telemetry
- Collect analytics
- Use cloud services

### IPC Security

IPC communication:
- Localhost-only binding by default
- No authentication (local-only use)
- Optional Unix domain socket

## Future Privacy Features

Planned privacy enhancements:
- Database encryption at rest
- Secure delete (overwrite data)
- Audit logging
- Per-item retention policies
- Auto-deletion of sensitive items
- Privacy mode (no logging)

## Privacy Questions

### Is my data sent to the cloud?

No. All data is stored locally on your machine.

### Can Morsel access my clipboard without permission?

Morsel only accesses the clipboard when the daemon is running and you have started it.

### Does Morsel collect telemetry?

No. Morsel does not collect any telemetry or analytics.

### Can I encrypt my clipboard history?

Currently, encryption is not built-in, but you can encrypt backup files using external tools like GPG.

### How do I permanently delete my history?

```bash
morsel clear
rm ~/.local/share/morsel/morsel.db
```

### Can I prevent certain content from being stored?

Use sensitive detection to flag and optionally exclude certain content from storage.

### Is my clipboard history accessible to other users?

By default, the database file is only accessible to your user account. Ensure proper file permissions are set.

### Does Morsel work offline?

Yes. Morsel works entirely offline and requires no internet connection.
