# Configuration Reference

## Configuration File Location

The configuration file is located at:
- Linux/macOS: `~/.config/morsel/config.toml`
- Windows: `%APPDATA%\morsel\config.toml`

## Configuration Structure

```toml
[clipboard]
poll_interval_ms = 100
ignore_duplicates = true
max_buffer_size = 1000
max_content_size = 1048576
detect_ownership_changes = true

[retention]
max_items = 10000
max_age_days = 90
cleanup_interval_hours = 24

[storage]
db_path = "~/.local/share/morsel/morsel.db"
enable_wal = true
max_connections = 10

[search]
case_insensitive = true
fuzzy = true
max_results = 100

[privacy]
enable_sensitive_detection = true
[sensitive_detection]
enable_api_key = true
enable_access_token = true
enable_jwt = true
enable_private_key = true
enable_bearer_token = true
enable_password = true
enable_secret = true
confidence_threshold = 0.8
custom_patterns = []
whitelist_patterns = []
whitelist_content = []

[daemon]
ipc_address = "127.0.0.1:54321"
log_level = "info"
log_file = "~/.local/share/morsel/morseld.log"
```

## Section: clipboard

Controls clipboard monitoring behavior.

### poll_interval_ms

Polling interval in milliseconds for clipboard changes.

- **Type:** Integer
- **Default:** `100`
- **Range:** `10 - 10000`

Lower values provide faster detection but use more CPU.

### ignore_duplicates

Whether to ignore consecutive duplicate clipboard content.

- **Type:** Boolean
- **Default:** `true`

### max_buffer_size

Maximum number of clipboard events to buffer in memory.

- **Type:** Integer
- **Default:** `1000`
- **Range:** `100 - 10000`

### max_content_size

Maximum size of clipboard content to store (in bytes).

- **Type:** Integer
- **Default:** `1048576` (1MB)
- **Range:** `1024 - 104857600` (1KB - 100MB)

### detect_ownership_changes

Whether to detect clipboard ownership changes.

- **Type:** Boolean
- **Default:** `true`

## Section: retention

Controls how long clipboard items are kept.

### max_items

Maximum number of items to store in history.

- **Type:** Integer
- **Default:** `10000`
- **Range:** `100 - 1000000`

When exceeded, oldest items are removed.

### max_age_days

Maximum age of items to keep (in days).

- **Type:** Integer
- **Default:** `90`
- **Range:** `1 - 3650`

Items older than this are automatically removed.

### cleanup_interval_hours

How often to run cleanup (in hours).

- **Type:** Integer
- **Default:** `24`
- **Range:** `1 - 168` (1 hour to 1 week)

## Section: storage

Controls database storage behavior.

### db_path

Path to the SQLite database file.

- **Type:** String
- **Default:** `~/.local/share/morsel/morsel.db`

Supports `~` expansion for home directory.

### enable_wal

Enable Write-Ahead Logging for better concurrency.

- **Type:** Boolean
- **Default:** `true`

Disable if you experience issues on network filesystems.

### max_connections

Maximum number of database connections in the pool.

- **Type:** Integer
- **Default:** `10`
- **Range:** `1 - 100`

## Section: search

Controls search behavior.

### case_insensitive

Whether search is case-insensitive by default.

- **Type:** Boolean
- **Default:** `true`

### fuzzy

Whether fuzzy search is enabled by default.

- **Type:** Boolean
- **Default:** `true`

### max_results

Maximum number of search results to return.

- **Type:** Integer
- **Default:** `100`
- **Range:** `10 - 1000`

## Section: privacy

Controls privacy and sensitive content detection.

### enable_sensitive_detection

Whether to detect sensitive content.

- **Type:** Boolean
- **Default:** `true`

## Subsection: sensitive_detection

Fine-tunes sensitive content detection.

### enable_api_key

Detect API keys.

- **Type:** Boolean
- **Default:** `true`

### enable_access_token

Detect access tokens.

- **Type:** Boolean
- **Default:** `true`

### enable_jwt

Detect JWT tokens.

- **Type:** Boolean
- **Default:** `true`

### enable_private_key

Detect private keys.

- **Type:** Boolean
- **Default:** `true`

### enable_bearer_token

Detect bearer tokens.

- **Type:** Boolean
- **Default:** `true`

### enable_password

Detect passwords.

- **Type:** Boolean
- **Default:** `true`

### enable_secret

Detect secrets.

- **Type:** Boolean
- **Default:** `true`

### confidence_threshold

Confidence threshold for detection (0.0 - 1.0).

- **Type:** Float
- **Default:** `0.8`
- **Range:** `0.0 - 1.0`

Higher values reduce false positives.

### custom_patterns

Custom regex patterns for sensitive content.

- **Type:** Array of strings
- **Default:** `[]`

Example:
```toml
custom_patterns = [
    "my_custom_token_[a-zA-Z0-9]{32}",
    "secret_key_[a-f0-9]{64}"
]
```

### whitelist_patterns

Patterns to whitelist (never mark as sensitive).

- **Type:** Array of strings
- **Default:** `[]`

### whitelist_content

Specific content to whitelist.

- **Type:** Array of strings
- **Default:** `[]`

## Section: daemon

Controls daemon behavior.

### ipc_address

IPC address for daemon communication.

- **Type:** String
- **Default:** `"127.0.0.1:54321"`

### log_level

Logging level.

- **Type:** String
- **Default:** `"info"`
- **Options:** `trace`, `debug`, `info`, `warn`, `error`

### log_file

Path to daemon log file.

- **Type:** String
- **Default:** `~/.local/share/morsel/morseld.log`

## Environment Variables

Configuration can be overridden with environment variables:

- `MORSEL_CONFIG_PATH` - Path to config file
- `MORSEL_DB_PATH` - Database path
- `MORSEL_LOG_LEVEL` - Log level
- `MORSEL_IPC_ADDRESS` - IPC address

## Example Configurations

### Minimal Configuration

```toml
[clipboard]
poll_interval_ms = 100

[retention]
max_items = 1000
```

### Privacy-Focused Configuration

```toml
[privacy]
enable_sensitive_detection = true

[sensitive_detection]
confidence_threshold = 0.9
enable_password = true
enable_api_key = true
enable_secret = true

[retention]
max_age_days = 7
```

### High-Performance Configuration

```toml
[clipboard]
poll_interval_ms = 50
max_buffer_size = 5000

[storage]
enable_wal = true
max_connections = 20

[search]
max_results = 500
```
