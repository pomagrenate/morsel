# Quick Start Guide

Get started with Morsel in 5 minutes.

## 1. Installation

```bash
cargo install morsel
```

## 2. Start the Daemon

The daemon monitors your clipboard and stores history:

```bash
morseld start
```

## 3. Basic Usage

### View Clipboard History

```bash
morsel list
```

This shows your recent clipboard items with IDs.

### Search History

```bash
morsel search "github"
```

Fuzzy search is enabled by default, so "gthub" will also match "github".

### Copy an Item to Clipboard

```bash
morsel copy <id>
```

Replace `<id>` with the item ID from the list.

### Get Item Details

```bash
morsel get <id>
```

Shows full content, metadata, and tags.

## 4. Working with Tags

### Add Tags

```bash
morsel tag add <id> work important
```

### Remove Tags

```bash
morsel tag remove <id> work
```

### List by Tag

```bash
morsel list --tag work
```

## 5. Collections

### Create a Collection

```bash
morsel collection create "Project X"
```

### Add to Collection

```bash
morsel collection add <collection-id> <item-id>
```

### List Collections

```bash
morsel collection list
```

## 6. Favorites

Mark important items as favorites:

```bash
morsel favorite <id>
```

List only favorites:

```bash
morsel list --favorites
```

## 7. Export and Import

### Export History

```bash
morsel export backup.json
```

### Import History

```bash
morsel import backup.json
```

## 8. Configuration

View current configuration:

```bash
morsel config show
```

Edit configuration:

```bash
morsel config edit
```

## Common Workflows

### Copy from Search Results

```bash
morsel search "api" --copy
```

This copies the first matching result to your clipboard.

### Clean Old Items

```bash
morsel cleanup --older-than 30d
```

Removes items older than 30 days.

### Clear All History

```bash
morsel clear
```

⚠️ This cannot be undone.

## Stopping the Daemon

```bash
morseld stop
```

## Next Steps

- Read the [CLI Documentation](cli.md) for all commands
- Configure retention policies in [Configuration Reference](configuration.md)
- Learn about [Privacy](privacy.md) and [Security](security.md)
