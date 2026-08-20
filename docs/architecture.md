# Architecture Documentation

## Overview

Morsel is organized as a modular Rust application with clear separation of concerns. The architecture follows a layered approach:

```
┌─────────────────────────────────────────────────────────┐
│                   CLI / TUI                             │
│              (User Interface Layer)                      │
└────────────────────┬────────────────────────────────────┘
                     │ IPC
┌────────────────────▼────────────────────────────────────┐
│                    Daemon                               │
│              (Orchestration Layer)                       │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
┌───────▼──────┐ ┌──▼──────┐ ┌──▼──────────┐
│  Searching   │ │Storage  │ │ Clipboard   │
│              │ │         │ │             │
└───────┬──────┘ └──┬──────┘ └──┬──────────┘
        │            │            │
        └────────────┼────────────┘
                     │
            ┌────────▼────────┐
            │      Core       │
            │  (Data Models)  │
            └─────────────────┘
                     │
            ┌────────▼────────┐
            │    Platform     │
            │  (Abstraction)  │
            └─────────────────┘
```

## Crate Responsibilities

### morsel-core

**Purpose:** Core data models and shared types.

**Responsibilities:**
- `ClipboardItem` - Primary data structure for clipboard content
- `Collection` - Grouping mechanism for clipboard items
- `ItemId` - Unique identifier for items
- `ContentType` - Content type detection (URL, email, code, etc.)
- `SensitiveDetection` - Detection of sensitive content
- `Backup` - Backup/restore format
- Error types and shared utilities

**Dependencies:**
- `serde` - Serialization
- `chrono` - Date/time handling
- `uuid` - Unique identifiers
- `regex` - Pattern matching

### morsel-clipboard

**Purpose:** Clipboard monitoring and interaction.

**Responsibilities:**
- `ClipboardProvider` trait - Platform-agnostic clipboard interface
- `ClipboardMonitor` - Monitors clipboard for changes
- `ClipboardEvent` - Events (NewItem, Cleared, OwnershipChange)
- Platform implementations (Windows, macOS, Linux)

**Dependencies:**
- `morsel-core`
- `tokio` - Async runtime
- Platform-specific clipboard libraries

### morsel-storage

**Purpose:** Persistent storage of clipboard history.

**Responsibilities:**
- `StorageBackend` trait - Storage abstraction
- `SqliteStorage` - SQLite implementation
- CRUD operations for clipboard items
- Blob storage for binary content
- Schema migrations
- Connection pooling

**Dependencies:**
- `morsel-core`
- `rusqlite` - SQLite bindings
- `r2d2` - Connection pooling
- `tokio` - Async runtime

### morsel-search

**Purpose:** Search and indexing of clipboard history.

**Responsibilities:**
- `SearchEngine` - In-memory search index
- `SearchQuery` - Search query builder
- Fuzzy matching
- Content type filtering
- Ranking and scoring

**Dependencies:**
- `morsel-core`
- `fuzzy-matcher` - Fuzzy search algorithm

### morsel-platform

**Purpose:** Platform-specific abstractions.

**Responsibilities:**
- Platform detection
- Native notifications
- System tray integration
- Auto-start configuration

**Dependencies:**
- Platform-specific libraries

### morsel-daemon

**Purpose:** Background service orchestrating all components.

**Responsibilities:**
- IPC server (TCP)
- Clipboard monitoring
- Storage management
- Search indexing
- Cleanup tasks
- Configuration management

**Dependencies:**
- All other crates
- `tokio` - Async runtime
- `tracing` - Logging

### morsel-cli

**Purpose:** Command-line interface.

**Responsibilities:**
- Command parsing
- IPC client
- User interaction
- Configuration management

**Dependencies:**
- `morsel-core`
- `morsel-daemon` (IPC client)
- `clap` - CLI parsing

### morsel-tui

**Purpose:** Terminal user interface.

**Responsibilities:**
- Interactive TUI
- Real-time clipboard display
- Search interface
- Keyboard navigation

**Dependencies:**
- `morsel-core`
- `morsel-daemon` (IPC client)
- TUI library (ratatui, crossterm)

## Data Flow

### Clipboard Item Lifecycle

1. **Capture**
   - `ClipboardMonitor` detects clipboard change
   - Creates `ClipboardEvent::NewItem`
   - Emits event via channel

2. **Processing**
   - Daemon receives event
   - Detects content type
   - Checks for sensitive content
   - Applies retention policies

3. **Storage**
   - `SqliteStorage::insert()` stores item
   - Item indexed in `SearchEngine`

4. **Retrieval**
   - CLI/TUI requests via IPC
   - Daemon queries storage
   - Results returned via IPC

### Search Lifecycle

1. **Indexing**
   - Items indexed on storage
   - `SearchEngine::index_item()` adds to in-memory index
   - Index updated on delete/clear

2. **Query**
   - User submits search query
   - `SearchEngine::search()` processes query
   - Applies filters (content type, tags, favorites)
   - Returns ranked results

3. **Display**
   - Results formatted for display
   - Metadata included (score, content type, tags)

### Storage Lifecycle

1. **Initialization**
   - Database file created if needed
   - Schema migrations applied
   - WAL mode enabled (if configured)

2. **Operations**
   - All operations use connection pool
   - Write operations use blocking tasks
   - Read operations can be concurrent

3. **Cleanup**
   - Scheduled cleanup removes old items
   - Expired items removed based on retention policy
   - Orphaned blobs cleaned up

## IPC Architecture

### Protocol

- **Transport:** TCP
- **Default Port:** 54321
- **Format:** JSON
- **Connection:** Single daemon, multiple clients

### Request Types

```rust
enum IpcRequest {
    Status,
    GetHistory { limit: usize },
    Search { query: String, limit: usize },
    AddItem { content: String },
    GetItem { id: ItemId },
    DeleteItem { id: ItemId },
    ClearHistory,
    Stop,
}
```

### Response Types

```rust
enum IpcResponse {
    Status { running: bool },
    Success { data: Option<serde_json::Value> },
    Error { message: String },
}
```

### Security

- Localhost-only binding by default
- No authentication (local-only use)
- Optional Unix domain socket on Linux/macOS

## Platform Abstraction

### Clipboard

Platform-specific implementations of `ClipboardProvider`:

- **Windows:** `winapi` / `windows-rs`
- **macOS:** `NSPasteboard`
- **Linux:** X11 (`x11-dl`) or Wayland (`wayland-client`)

### Notifications

- **Windows:** Windows Toast Notifications
- **macOS:** `NSUserNotification`
- **Linux:** `libnotify` (D-Bus)

### System Integration

- **Windows:** Registry for auto-start
- **macOS:** LaunchAgents
- **Linux:** systemd user units or autostart desktop files

## Security Model

### Data Storage

- SQLite database with file permissions restricted to user
- Optional encryption for sensitive fields
- No network transmission of clipboard content

### Sensitive Content

- Detection patterns for common secrets
- Configurable confidence threshold
- Whitelisting capability
- Optional auto-deletion of sensitive items

### Privacy

- All data stored locally
- No telemetry or analytics
- No cloud sync (by design)
- User controls retention policies

## Performance Considerations

### Memory

- Search index is in-memory
- Clipboard events buffered (configurable size)
- Connection pooling for database

### Disk I/O

- WAL mode for concurrent reads/writes
- Batch operations for bulk inserts
- Periodic cleanup to manage database size

### CPU

- Polling interval configurable
- Fuzzy search is CPU-intensive
- Sensitive detection runs on each new item

## Concurrency Model

### Async/Await

- Tokio runtime for async operations
- Blocking database operations in spawn_blocking
- Channel-based event passing

### Thread Pool

- Tokio default thread pool for async tasks
- Blocking operations use dedicated thread pool
- Configurable database connection pool

## Error Handling

### Error Types

Each crate defines its own error types:

- `CoreError` - Core data model errors
- `ClipboardError` - Clipboard operation errors
- `StorageError` - Storage operation errors
- `SearchError` - Search operation errors
- `DaemonError` - Daemon operation errors

### Error Propagation

- Errors use `thiserror` for clean error messages
- Errors propagated via `Result<T, E>`
- IPC errors serialized and sent to clients

## Extensibility

### Adding New Platforms

1. Implement `ClipboardProvider` trait
2. Add platform-specific dependencies
3. Implement notification system
4. Add auto-start mechanism

### Adding New Storage Backends

1. Implement `StorageBackend` trait
2. Add as optional feature
3. Update configuration

### Adding Search Algorithms

1. Extend `SearchEngine`
2. Add configuration options
3. Update query builder
