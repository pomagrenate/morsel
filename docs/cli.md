# CLI Documentation

## Overview

The `morsel` CLI provides commands for managing clipboard history, searching, and configuration.

## Global Options

```
morsel [OPTIONS] <COMMAND>
```

**Options:**
- `-c, --config <PATH>` - Path to configuration file
- `-v, --verbose` - Increase verbosity
- `-q, --quiet` - Decrease verbosity
- `-h, --help` - Print help
- `-V, --version` - Print version

## Commands

### list

List clipboard history.

```
morsel list [OPTIONS]
```

**Options:**
- `-n, --limit <N>` - Limit number of results (default: 20)
- `--offset <N>` - Skip N results
- `--tag <TAG>` - Filter by tag
- `--favorites` - Show only favorites
- `--content-type <TYPE>` - Filter by content type (text, url, email, code, json, sql)
- `--reverse` - Show in reverse order

**Examples:**
```bash
morsel list
morsel list --limit 50
morsel list --tag work
morsel list --favorites --limit 10
```

### search

Search clipboard history.

```
morsel search [OPTIONS] <QUERY>
```

**Options:**
- `-n, --limit <N>` - Limit number of results (default: 10)
- `--exact` - Exact match only (disable fuzzy search)
- `--case-sensitive` - Case-sensitive search
- `--tag <TAG>` - Filter by tag
- `--favorites` - Search only favorites
- `--content-type <TYPE>` - Filter by content type
- `--copy` - Copy first result to clipboard

**Examples:**
```bash
morsel search "github"
morsel search "api" --limit 5 --copy
morsel search "SELECT" --content-type sql
```

### get

Get details of a specific item.

```
morsel get <ID>
```

**Examples:**
```bash
morsel get abc123def456
```

### copy

Copy an item to clipboard.

```
morsel copy <ID>
```

**Examples:**
```bash
morsel copy abc123def456
```

### delete

Delete an item from history.

```
morsel delete <ID>
```

**Examples:**
```bash
morsel delete abc123def456
```

### clear

Clear all clipboard history.

```
morsel clear
```

⚠️ **Warning:** This cannot be undone.

**Examples:**
```bash
morsel clear
```

### tag

Manage tags on items.

```
morsel tag <SUBCOMMAND>
```

**Subcommands:**

#### add

Add tags to an item.

```
morsel tag add <ID> <TAG>...
```

**Examples:**
```bash
morsel tag add abc123def456 work important
```

#### remove

Remove tags from an item.

```
morsel tag remove <ID> <TAG>...
```

**Examples:**
```bash
morsel tag remove abc123def456 work
```

#### list

List all tags.

```
morsel tag list
```

**Examples:**
```bash
morsel tag list
```

### favorite

Mark items as favorites.

```
morsel favorite <ID>
```

To unfavorite, use:

```
morsel unfavorite <ID>
```

**Examples:**
```bash
morsel favorite abc123def456
morsel unfavorite abc123def456
```

### collection

Manage collections.

```
morsel collection <SUBCOMMAND>
```

**Subcommands:**

#### create

Create a new collection.

```
morsel collection create <NAME> [DESCRIPTION]
```

**Examples:**
```bash
morsel collection create "Project X" "Clipboard items for Project X"
```

#### list

List all collections.

```
morsel collection list
```

**Examples:**
```bash
morsel collection list
```

#### add

Add an item to a collection.

```
morsel collection add <COLLECTION-ID> <ITEM-ID>
```

**Examples:**
```bash
morsel collection add xyz789 abc123def456
```

#### remove

Remove an item from a collection.

```
morsel collection remove <COLLECTION-ID> <ITEM-ID>
```

**Examples:**
```bash
morsel collection remove xyz789 abc123def456
```

#### delete

Delete a collection.

```
morsel collection delete <COLLECTION-ID>
```

**Examples:**
```bash
morsel collection delete xyz789
```

### export

Export clipboard history to a file.

```
morsel export [OPTIONS] <PATH>
```

**Options:**
- `--format <FORMAT>` - Export format (json, default: json)
- `--include-blobs` - Include binary blobs

**Examples:**
```bash
morsel export backup.json
morsel export backup.json --include-blobs
```

### import

Import clipboard history from a file.

```
morsel import [OPTIONS] <PATH>
```

**Options:**
- `--format <FORMAT>` - Import format (json, default: json)
- `--merge` - Merge with existing history

**Examples:**
```bash
morsel import backup.json
morsel import backup.json --merge
```

### cleanup

Clean up old or expired items.

```
morsel cleanup [OPTIONS]
```

**Options:**
- `--older-than <DURATION>` - Remove items older than duration (e.g., 30d, 1w, 6h)
- `--expired` - Remove only expired items
- `--dry-run` - Show what would be removed without actually removing

**Examples:**
```bash
morsel cleanup --older-than 30d
morsel cleanup --expired
morsel cleanup --older-than 7d --dry-run
```

### config

Manage configuration.

```
morsel config <SUBCOMMAND>
```

**Subcommands:**

#### init

Initialize configuration file.

```
morsel config init
```

**Examples:**
```bash
morsel config init
```

#### show

Show current configuration.

```
morsel config show
```

**Examples:**
```bash
morsel config show
```

#### edit

Edit configuration file.

```
morsel config edit
```

**Examples:**
```bash
morsel config edit
```

#### set

Set a configuration value.

```
morsel config set <KEY> <VALUE>
```

**Examples:**
```bash
morsel config set retention.max_items 1000
morsel config set clipboard.poll_interval_ms 100
```

#### get

Get a configuration value.

```
morsel config get <KEY>
```

**Examples:**
```bash
morsel config get retention.max_items
```

### status

Show daemon status.

```
morsel status
```

**Examples:**
```bash
morsel status
```

### info

Show system information.

```
morsel info
```

**Examples:**
```bash
morsel info
```

## Exit Codes

- `0` - Success
- `1` - General error
- `2` - Invalid usage
- `3` - Daemon not running
- `4` - Item not found
- `5` - Configuration error
