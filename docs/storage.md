# Storage Documentation

## Overview

Morsel uses SQLite as the primary storage backend for clipboard history. The storage layer is abstracted through the `StorageBackend` trait, allowing for future support of other databases.

## Database Schema

### clipboard_items Table

```sql
CREATE TABLE clipboard_items (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL,
    created_at TEXT NOT NULL,
    last_used_at TEXT NOT NULL,
    size INTEGER NOT NULL,
    is_favorite INTEGER NOT NULL DEFAULT 0,
    tags TEXT NOT NULL,  -- JSON array
    collection_id TEXT,
    expires_at TEXT,
    source TEXT
);
```

**Columns:**
- `id` - Unique identifier (UUID v4)
- `content` - Clipboard content
- `content_type` - Detected content type
- `created_at` - ISO 8601 timestamp
- `last_used_at` - ISO 8601 timestamp
- `size` - Content size in bytes
- `is_favorite` - Boolean (0/1)
- `tags` - JSON array of tag strings
- `collection_id` - Reference to collection (optional)
- `expires_at` - ISO 8601 timestamp (optional)
- `source` - Source of the item (optional)

### blobs Table

```sql
CREATE TABLE blobs (
    id TEXT PRIMARY KEY,
    mime_type TEXT NOT NULL,
    size INTEGER NOT NULL,
    hash TEXT NOT NULL,
    data BLOB NOT NULL,
    created_at TEXT NOT NULL
);
```

**Columns:**
- `id` - Unique identifier (UUID v4)
- `mime_type` - MIME type of the blob
- `size` - Size in bytes
- `hash` - MD5 hash for deduplication
- `data` - Binary data
- `created_at` - ISO 8601 timestamp

### schema_version Table

```sql
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL
);
```

## Storage Backend Trait

```rust
#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn initialize(&self) -> StorageResult<()>;
    async fn insert(&self, item: &ClipboardItem) -> StorageResult<()>;
    async fn get(&self, id: ItemId) -> StorageResult<ClipboardItem>;
    async fn update(&self, item: &ClipboardItem) -> StorageResult<()>;
    async fn delete(&self, id: ItemId) -> StorageResult<()>;
    async fn list(&self) -> StorageResult<Vec<ClipboardItem>>;
    async fn count(&self) -> StorageResult<usize>;
    async fn delete_many(&self, ids: Vec<ItemId>) -> StorageResult<usize>;
    async fn delete_older_than(&self, cutoff: DateTime<Utc>) -> StorageResult<usize>;
    async fn delete_expired(&self) -> StorageResult<usize>;
    async fn store_blob(&self, item_id: ItemId, mime_type: String, data: Vec<u8>) -> StorageResult<ItemId>;
    async fn get_blob(&self, blob_id: ItemId) -> StorageResult<(String, Vec<u8>)>;
    async fn cleanup_orphaned_blobs(&self) -> StorageResult<usize>;
}
```

## SQLite Implementation

### Connection Pooling

Morsel uses `r2d2` for connection pooling:

```rust
let pool = r2d2::Pool::builder()
    .max_pool_size(config.max_connections)
    .build(SqliteConnectionManager::new(&config.db_path))?;
```

### Write-Ahead Logging (WAL)

WAL mode is enabled by default for better concurrency:

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
```

WAL provides:
- Concurrent readers and writers
- Better performance
- Reduced disk I/O

Disable WAL on network filesystems if you experience issues.

### Schema Migrations

Migrations are versioned and applied automatically:

```rust
async fn initialize(&self) -> StorageResult<()> {
    let current_version = self.get_schema_version().await?;
    if current_version < 1 {
        self.apply_migration_v1().await?;
    }
    // Future migrations...
}
```

## Operations

### Insert

```rust
async fn insert(&self, item: &ClipboardItem) -> StorageResult<()> {
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO clipboard_items (...) VALUES (...)",
        params![...]
    )?;
    Ok(())
}
```

### Get

```rust
async fn get(&self, id: ItemId) -> StorageResult<ClipboardItem> {
    let conn = pool.get()?;
    let item = conn.query_row(
        "SELECT * FROM clipboard_items WHERE id = ?",
        params![id],
        |row| ClipboardItem::from_row(row)
    )?;
    Ok(item)
}
```

### Update

```rust
async fn update(&self, item: &ClipboardItem) -> StorageResult<()> {
    let conn = pool.get()?;
    conn.execute(
        "UPDATE clipboard_items SET ... WHERE id = ?",
        params![...]
    )?;
    Ok(())
}
```

### Delete

```rust
async fn delete(&self, id: ItemId) -> StorageResult<()> {
    let conn = pool.get()?;
    conn.execute("DELETE FROM clipboard_items WHERE id = ?", params![id])?;
    Ok(())
}
```

### List

```rust
async fn list(&self) -> StorageResult<Vec<ClipboardItem>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare("SELECT * FROM clipboard_items ORDER BY created_at DESC")?;
    let items = stmt.query_map([], |row| ClipboardItem::from_row(row))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(items)
}
```

## Blob Storage

Binary content (images, files) is stored separately:

### Storing Blobs

```rust
async fn store_blob(&self, item_id: ItemId, mime_type: String, data: Vec<u8>) -> StorageResult<ItemId> {
    let hash = md5::compute(&data);
    // Check for duplicates by hash
    // Store if not duplicate
    Ok(blob_id)
}
```

### Retrieving Blobs

```rust
async fn get_blob(&self, blob_id: ItemId) -> StorageResult<(String, Vec<u8>)> {
    let conn = pool.get()?;
    let (mime_type, data) = conn.query_row(
        "SELECT mime_type, data FROM blobs WHERE id = ?",
        params![blob_id],
        |row| Ok((row.get(0)?, row.get(1)?))
    )?;
    Ok((mime_type, data))
}
```

### Cleanup

Orphaned blobs (not referenced by any item) are cleaned up:

```rust
async fn cleanup_orphaned_blobs(&self) -> StorageResult<usize> {
    let conn = pool.get()?;
    let deleted = conn.execute(
        "DELETE FROM blobs WHERE id NOT IN (SELECT id FROM blobs WHERE id IN (...))",
        []
    )?;
    Ok(deleted)
}
```

## Performance Considerations

### Indexes

The following indexes are created:

```sql
CREATE INDEX idx_created_at ON clipboard_items(created_at);
CREATE INDEX idx_content_type ON clipboard_items(content_type);
CREATE INDEX idx_is_favorite ON clipboard_items(is_favorite);
CREATE INDEX idx_expires_at ON clipboard_items(expires_at);
CREATE INDEX idx_collection_id ON clipboard_items(collection_id);
CREATE INDEX idx_blob_hash ON blobs(hash);
```

### Batch Operations

For bulk operations, use transactions:

```rust
let conn = pool.get()?;
let tx = conn.transaction()?;
for item in items {
    tx.execute("INSERT INTO ...", params![...])?;
}
tx.commit()?;
```

### Connection Pool

Adjust pool size based on workload:

- Low usage: 5 connections
- Medium usage: 10 connections (default)
- High usage: 20+ connections

## Backup and Restore

### Backup

```bash
# SQLite backup
cp ~/.local/share/morsel/morsel.db ~/.local/share/morsel/morsel.db.backup

# Or using morsel export
morsel export backup.json
```

### Restore

```bash
# Restore from backup
cp ~/.local/share/morsel/morsel.db.backup ~/.local/share/morsel/morsel.db

# Or using morsel import
morsel import backup.json
```

## Database Maintenance

### Vacuum

Reclaim disk space:

```sql
VACUUM;
```

Morsel does not automatically vacuum. Run periodically if database grows significantly.

### Analyze

Update query planner statistics:

```sql
ANALYZE;
```

Run after large insertions or deletions.

### Integrity Check

Verify database integrity:

```sql
PRAGMA integrity_check;
```

## Troubleshooting

### Database Locked

**Symptom:** "database is locked" errors

**Solutions:**
- Increase `max_connections` in config
- Disable WAL mode on network filesystems
- Check for other processes accessing the database

### Slow Queries

**Symptom:** Slow list/search operations

**Solutions:**
- Run `ANALYZE` to update statistics
- Check indexes are present
- Consider reducing `max_items` to limit database size

### Corruption

**Symptom:** "database disk image is malformed"

**Solutions:**
- Restore from backup
- Run `PRAGMA integrity_check`
- Export data, delete database, re-import

## Configuration

### db_path

Database file location. Default: `~/.local/share/morsel/morsel.db`

### enable_wal

Enable Write-Ahead Logging. Default: `true`

### max_connections

Maximum connections in pool. Default: `10`

## Future Enhancements

Potential storage backends:

- PostgreSQL for distributed deployments
- MySQL for existing infrastructure
- Redis for in-memory caching
- S3-compatible storage for cloud backup
