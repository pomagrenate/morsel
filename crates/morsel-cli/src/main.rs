//! # morsel CLI
//!
//! Command-line interface for the morsel clipboard manager.

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use morsel_clipboard::{ClipboardMonitor, InMemoryClipboard, MonitorConfig};
use morsel_platform::Hotkey;
use morsel_search::{SearchEngine, SearchQuery};
use morsel_storage::{SqliteStorage, StorageBackend, StorageConfig};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, error, info, Level};
use tracing_subscriber;

/// Global configuration for morsel
#[derive(Debug, Serialize, Deserialize, Clone)]
struct MorselConfig {
    /// Database path
    #[serde(default = "default_db_path")]
    db_path: String,
    
    /// Maximum number of items to keep
    #[serde(default = "default_max_items")]
    max_items: usize,
    
    /// Polling interval for clipboard monitoring (ms)
    #[serde(default = "default_poll_interval")]
    poll_interval: u64,
    
    /// Enable verbose logging by default
    #[serde(default)]
    verbose: bool,
    
    /// Global hotkey for opening TUI
    #[serde(default = "default_hotkey")]
    hotkey: String,
    
    /// Enable sensitive content detection
    #[serde(default = "default_sensitive_detection")]
    sensitive_detection: bool,
    
    /// Auto-delete detected sensitive content
    #[serde(default)]
    auto_delete_sensitive: bool,
    
    /// Enable encryption (future feature)
    #[serde(default)]
    enable_encryption: bool,
    
    /// Retention policy in days (0 = indefinite)
    #[serde(default = "default_retention_days")]
    retention_days: u32,
}

fn default_db_path() -> String {
    "morsel.db".to_string()
}

fn default_max_items() -> usize {
    1000
}

fn default_poll_interval() -> u64 {
    250
}

fn default_hotkey() -> String {
    "Ctrl+Shift+V".to_string()
}

fn default_sensitive_detection() -> bool {
    true
}

fn default_retention_days() -> u32 {
    0
}

impl Default for MorselConfig {
    fn default() -> Self {
        Self {
            db_path: default_db_path(),
            max_items: default_max_items(),
            poll_interval: default_poll_interval(),
            verbose: false,
            hotkey: default_hotkey(),
            sensitive_detection: default_sensitive_detection(),
            auto_delete_sensitive: false,
            enable_encryption: false,
            retention_days: default_retention_days(),
        }
    }
}

/// Get the configuration file path
fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Failed to get config directory")?;
    let morsel_dir = config_dir.join("morsel");
    std::fs::create_dir_all(&morsel_dir)
        .context("Failed to create morsel config directory")?;
    Ok(morsel_dir.join("config.toml"))
}

/// Load configuration from file
fn load_config() -> Result<MorselConfig> {
    let config_file = config_path()?;
    
    if !config_file.exists() {
        // Create default config
        let default_config = MorselConfig::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }
    
    let content = std::fs::read_to_string(&config_file)
        .context("Failed to read config file")?;
    
    let config: MorselConfig = toml::from_str(&content)
        .context("Failed to parse config file")?;
    
    Ok(config)
}

/// Save configuration to file
fn save_config(config: &MorselConfig) -> Result<()> {
    let config_file = config_path()?;
    let content = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;
    
    std::fs::write(&config_file, content)
        .context("Failed to write config file")?;
    
    Ok(())
}

/// morsel - A blazing-fast, encrypted, local-first clipboard manager
#[derive(Parser, Debug)]
#[command(name = "morsel")]
#[command(about = "A blazing-fast, encrypted, local-first clipboard manager built for developers.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List clipboard history
    List {
        /// Maximum number of items to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// Search clipboard history
    Search {
        /// Search query
        query: String,

        /// Enable fuzzy search
        #[arg(short, long)]
        fuzzy: bool,

        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// Get a specific clipboard item by ID
    Get {
        /// Item ID
        id: String,
    },

    /// Delete a clipboard item
    Delete {
        /// Item ID
        id: String,
    },

    /// Clear all clipboard history
    Clear,

    /// Show statistics
    Stats,

    /// Pin (favorite) a clipboard item
    Pin {
        /// Item ID
        id: String,
    },

    /// Unpin (unfavorite) a clipboard item
    Unpin {
        /// Item ID
        id: String,
    },

    /// List favorite items
    Favorites {
        /// Maximum number of items to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// Manage collections
    Collection {
        #[command(subcommand)]
        command: CollectionCommands,
    },

    /// Generate shell completion scripts
    Completions {
        /// Shell type (bash, zsh, fish, elvish, powershell)
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Configure morsel settings
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Run hotkey listener to open TUI
    Hotkey,

    /// Clean up expired clipboard items
    Cleanup,

    /// Export clipboard data to a backup file
    Export {
        /// Output file path
        output: String,
    },

    /// Import clipboard data from a backup file
    Import {
        /// Input file path
        input: String,
    },

    /// Daemon management commands
    Daemon {
        #[command(subcommand)]
        command: DaemonCommands,
    },
}

#[derive(Subcommand, Debug)]
enum DaemonCommands {
    /// Start the daemon
    Start,
    /// Stop the daemon
    Stop,
    /// Restart the daemon
    Restart,
    /// Check daemon status
    Status,
}

#[derive(Subcommand, Debug)]
enum CollectionCommands {
    /// List all collections
    List,

    /// Create a new collection
    Create {
        /// Collection name
        name: String,
    },

    /// Delete a collection
    Delete {
        /// Collection name
        name: String,
    },

    /// Add an item to a collection
    Add {
        /// Item ID
        id: String,
        /// Collection name
        collection: String,
    },

    /// Remove an item from a collection
    Remove {
        /// Item ID
        id: String,
        /// Collection name
        collection: String,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigCommands {
    /// Show current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Configuration key (db_path, max_items, poll_interval, verbose, hotkey)
        key: String,
        /// Configuration value
        value: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load configuration
    let config = load_config()?;

    // Initialize logging
    let log_level = if cli.verbose || config.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();

    run_command(cli.command, config).await
}

async fn run_command(command: Commands, config: MorselConfig) -> Result<()> {
    match command {
        Commands::List { limit } => {
            cmd_list(limit, config).await?;
        }
        Commands::Search { query, fuzzy, limit } => {
            cmd_search(query, fuzzy, limit, config).await?;
        }
        Commands::Get { id } => {
            cmd_get(id, config).await?;
        }
        Commands::Delete { id } => {
            cmd_delete(id, config).await?;
        }
        Commands::Clear => {
            cmd_clear(config).await?;
        }
        Commands::Stats => {
            cmd_stats(config).await?;
        }
        Commands::Pin { id } => {
            cmd_pin(id, config).await?;
        }
        Commands::Unpin { id } => {
            cmd_unpin(id, config).await?;
        }
        Commands::Favorites { limit } => {
            cmd_favorites(limit, config).await?;
        }
        Commands::Collection { command } => {
            cmd_collection(command, config).await?;
        }
        Commands::Daemon { command } => {
            cmd_daemon(command).await?;
        }
        Commands::Completions { shell } => {
            cmd_completions(shell)?;
        }
        Commands::Config { command } => {
            cmd_config(command)?;
        }
        Commands::Hotkey => {
            cmd_hotkey(config).await?;
        }
        Commands::Cleanup => {
            cmd_cleanup(config).await?;
        }
        Commands::Export { output } => {
            cmd_export(config, output).await?;
        }
        Commands::Import { input } => {
            cmd_import(config, input).await?;
        }
    }

    Ok(())
}

async fn cmd_list(limit: usize, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let items = storage.list().await?;
    let items: Vec<_> = items.into_iter().take(limit).collect();

    if items.is_empty() {
        println!("No clipboard items found.");
        return Ok(());
    }

    println!("Clipboard History ({} items):", items.len());
    println!();

    for (idx, item) in items.iter().enumerate() {
        let preview = if item.content.len() > 50 {
            format!("{}...", &item.content[..50])
        } else {
            item.content.clone()
        };

        println!("{}. [{}] {}", idx + 1, item.id, preview);
        println!("   Type: {} | Size: {} bytes | Favorite: {}",
            item.content_type, item.size, item.is_favorite);
        println!("   Created: {}", item.created_at.format("%Y-%m-%d %H:%M:%S"));
        println!();
    }

    Ok(())
}

async fn cmd_search(query: String, fuzzy: bool, limit: usize, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let items = storage.list().await?;

    let mut engine = SearchEngine::new();
    for item in items {
        engine.index_item(item);
    }

    let search_query = SearchQuery::new(query.clone())
        .fuzzy(fuzzy)
        .limit(limit);

    let results = engine.search(&search_query)?;

    if results.is_empty() {
        println!("No results found for: {}", query);
        return Ok(());
    }

    println!("Search results for '{}':", query);
    println!();

    for (idx, scored) in results.iter().enumerate() {
        let preview = if scored.item.content.len() > 50 {
            format!("{}...", &scored.item.content[..50])
        } else {
            scored.item.content.clone()
        };

        println!("{}. [{}] (score: {}) {}", idx + 1, scored.item.id, scored.score, preview);
        println!();
    }

    Ok(())
}

async fn cmd_get(id: String, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let item_id = morsel_core::ItemId::from_str(&id)
        .context("Invalid item ID format")?;

    let item = storage.get(item_id).await?;

    println!("Item: {}", item.id);
    println!("Content: {}", item.content);
    println!("Type: {}", item.content_type);
    println!("Size: {} bytes", item.size);
    println!("Created: {}", item.created_at.format("%Y-%m-%d %H:%M:%S"));
    println!("Last used: {}", item.last_used_at.format("%Y-%m-%d %H:%M:%S"));
    println!("Favorite: {}", item.is_favorite);
    
    if !item.tags.is_empty() {
        println!("Tags: {}", item.tags.join(", "));
    }

    Ok(())
}

async fn cmd_delete(id: String, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let item_id = morsel_core::ItemId::from_str(&id)
        .context("Invalid item ID format")?;

    storage.delete(item_id).await?;
    println!("Item deleted successfully.");

    Ok(())
}

async fn cmd_clear(config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let count = storage.count().await?;
    storage.clear().await?;
    println!("Cleared {} clipboard items.", count);

    Ok(())
}

async fn cmd_stats(config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let items = storage.list().await?;
    let total_count = items.len();
    let favorite_count = items.iter().filter(|i| i.is_favorite).count();
    let total_size: usize = items.iter().map(|i| i.size).sum();

    let mut type_counts = std::collections::HashMap::new();
    for item in items {
        *type_counts.entry(item.content_type).or_insert(0) += 1;
    }

    println!("Morsel Statistics:");
    println!("  Total items: {}", total_count);
    println!("  Favorites: {}", favorite_count);
    println!("  Total size: {} bytes", total_size);
    println!();
    println!("By content type:");
    for (content_type, count) in type_counts {
        println!("  {}: {}", content_type, count);
    }

    Ok(())
}

async fn cmd_pin(id: String, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let item_id = morsel_core::ItemId::from_str(&id)
        .context("Invalid item ID format")?;

    let mut item = storage.get(item_id).await?;
    item.set_favorite(true);
    storage.update(&item).await?;

    println!("Item pinned successfully.");

    Ok(())
}

async fn cmd_unpin(id: String, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let item_id = morsel_core::ItemId::from_str(&id)
        .context("Invalid item ID format")?;

    let mut item = storage.get(item_id).await?;
    item.set_favorite(false);
    storage.update(&item).await?;

    println!("Item unpinned successfully.");

    Ok(())
}

async fn cmd_favorites(limit: usize, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let items = storage.list().await?;
    let favorites: Vec<_> = items.into_iter()
        .filter(|i| i.is_favorite)
        .take(limit)
        .collect();

    if favorites.is_empty() {
        println!("No favorite items found.");
        return Ok(());
    }

    println!("Favorite Items ({}):", favorites.len());
    println!();

    for (idx, item) in favorites.iter().enumerate() {
        let preview = if item.content.len() > 50 {
            format!("{}...", &item.content[..50])
        } else {
            item.content.clone()
        };

        println!("{}. [{}] {}", idx + 1, item.id, preview);
        println!("   Type: {} | Size: {} bytes", item.content_type, item.size);
        println!("   Created: {}", item.created_at.format("%Y-%m-%d %H:%M:%S"));
        println!();
    }

    Ok(())
}

async fn cmd_collection(command: CollectionCommands, config: MorselConfig) -> Result<()> {
    match command {
        CollectionCommands::List => {
            cmd_collection_list(config).await?;
        }
        CollectionCommands::Create { name } => {
            cmd_collection_create(name, config).await?;
        }
        CollectionCommands::Delete { name } => {
            cmd_collection_delete(name, config).await?;
        }
        CollectionCommands::Add { id, collection } => {
            cmd_collection_add(id, collection, config).await?;
        }
        CollectionCommands::Remove { id, collection } => {
            cmd_collection_remove(id, collection, config).await?;
        }
    }
    Ok(())
}

async fn cmd_collection_list(config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let items = storage.list().await?;
    let mut collections: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for item in items {
        if let Some(collection_id) = &item.collection_id {
            *collections.entry(collection_id.to_string()).or_insert(0) += 1;
        }
    }

    if collections.is_empty() {
        println!("No collections found.");
        return Ok(());
    }

    println!("Collections:");
    for (name, count) in collections {
        println!("  {} ({} items)", name, count);
    }

    Ok(())
}

async fn cmd_collection_create(name: String, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    // Create a placeholder item to represent the collection
    let mut item = morsel_core::ClipboardItem::new(format!("Collection: {}", name));
    let collection_id = morsel_core::ItemId::from_str(&name)
        .unwrap_or_else(|_| morsel_core::ItemId::new());
    item.collection_id = Some(collection_id);
    storage.insert(&item).await?;

    println!("Collection '{}' created successfully.", name);

    Ok(())
}

async fn cmd_collection_delete(name: String, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let collection_id = morsel_core::ItemId::from_str(&name)
        .context("Invalid collection ID format")?;

    let items = storage.list().await?;
    let items_to_delete: Vec<_> = items.into_iter()
        .filter(|i| i.collection_id.as_ref() == Some(&collection_id))
        .map(|i| i.id)
        .collect();

    if items_to_delete.is_empty() {
        println!("Collection '{}' not found or is empty.", name);
        return Ok(());
    }

    let deleted = storage.delete_many(items_to_delete).await?;
    println!("Collection '{}' deleted. Removed {} items.", name, deleted);

    Ok(())
}

async fn cmd_collection_add(id: String, collection: String, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let item_id = morsel_core::ItemId::from_str(&id)
        .context("Invalid item ID format")?;
    let collection_id = morsel_core::ItemId::from_str(&collection)
        .context("Invalid collection ID format")?;

    let mut item = storage.get(item_id).await?;
    item.collection_id = Some(collection_id);
    storage.update(&item).await?;

    println!("Item added to collection '{}'.", collection);

    Ok(())
}

async fn cmd_collection_remove(id: String, collection: String, config: MorselConfig) -> Result<()> {
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;

    let item_id = morsel_core::ItemId::from_str(&id)
        .context("Invalid item ID format")?;
    let collection_id = morsel_core::ItemId::from_str(&collection)
        .context("Invalid collection ID format")?;

    let mut item = storage.get(item_id).await?;
    if item.collection_id.as_ref() == Some(&collection_id) {
        item.collection_id = None;
        storage.update(&item).await?;
        println!("Item removed from collection '{}'.", collection);
    } else {
        println!("Item is not in collection '{}'.", collection);
    }

    Ok(())
}

async fn cmd_daemon_start(interval: u64, db_path: String, _config: MorselConfig) -> Result<()> {
    info!("Starting morsel daemon with interval: {}ms", interval);
    info!("Database path: {}", db_path);

    let storage_config = StorageConfig {
        db_path,
        ..Default::default()
    };

    let storage = Arc::new(SqliteStorage::new(storage_config)?);
    storage.initialize().await?;

    let clipboard = Arc::new(InMemoryClipboard::new());
    let monitor_config = MonitorConfig {
        poll_interval_ms: interval,
        ignore_duplicates: true,
        max_buffer_size: 1000,
        detect_ownership_changes: true,
        max_content_size: 0,
    };

    let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), monitor_config);
    monitor.start().await?;

    info!("Clipboard monitor started. Press Ctrl+C to stop.");

    // Process clipboard events
    loop {
        tokio::select! {
            event = receiver.recv() => {
                match event {
                    Some(morsel_clipboard::ClipboardEvent::NewItem(item)) => {
                        info!("New clipboard item: {}", item.id);
                        if let Err(e) = storage.insert(&item).await {
                            error!("Failed to store item: {}", e);
                        }
                    }
                    Some(morsel_clipboard::ClipboardEvent::Cleared) => {
                        info!("Clipboard cleared");
                    }
                    Some(morsel_clipboard::ClipboardEvent::OwnershipChanged) => {
                        debug!("Clipboard ownership changed");
                    }
                    Some(morsel_clipboard::ClipboardEvent::Error(e)) => {
                        error!("Clipboard error: {}", e);
                    }
                    None => {
                        error!("Clipboard monitor channel closed");
                        break;
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Received shutdown signal");
                monitor.stop().await;
                break;
            }
        }
    }

    info!("Morsel daemon stopped");
    Ok(())
}

async fn cmd_daemon(command: DaemonCommands) -> Result<()> {
    match command {
        DaemonCommands::Start => {
            info!("Starting morsel daemon...");
            // Spawn the daemon process
            let result = std::process::Command::new("morseld")
                .spawn()
                .context("Failed to start daemon")?;
            println!("Daemon started with PID: {}", result.id());
        }
        DaemonCommands::Stop => {
            info!("Stopping morsel daemon...");
            // TODO: Implement proper daemon stop via PID file or IPC
            println!("Daemon stop not yet implemented - use Ctrl+C to stop running daemon");
        }
        DaemonCommands::Restart => {
            info!("Restarting morsel daemon...");
            println!("Daemon restart not yet implemented");
        }
        DaemonCommands::Status => {
            info!("Checking daemon status...");
            // TODO: Implement proper daemon status check via PID file or IPC
            println!("Daemon status check not yet implemented");
        }
    }
    Ok(())
}

fn cmd_completions(shell: clap_complete::Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    
    clap_complete::generate(shell, &mut cmd, &name, &mut std::io::stdout());
    
    Ok(())
}

fn cmd_config(command: ConfigCommands) -> Result<()> {
    let mut config = load_config()?;
    
    match command {
        ConfigCommands::Show => {
            println!("Current configuration:");
            println!("  db_path: {}", config.db_path);
            println!("  max_items: {}", config.max_items);
            println!("  poll_interval: {}", config.poll_interval);
            println!("  verbose: {}", config.verbose);
            println!("  hotkey: {}", config.hotkey);
            println!("  sensitive_detection: {}", config.sensitive_detection);
            println!("  auto_delete_sensitive: {}", config.auto_delete_sensitive);
            println!("  enable_encryption: {}", config.enable_encryption);
            println!("  retention_days: {}", config.retention_days);
        }
        ConfigCommands::Set { key, value } => {
            match key.as_str() {
                "db_path" => config.db_path = value.clone(),
                "max_items" => {
                    config.max_items = value.parse()
                        .context("Invalid value for max_items (must be a number)")?;
                }
                "poll_interval" => {
                    config.poll_interval = value.parse()
                        .context("Invalid value for poll_interval (must be a number)")?;
                }
                "verbose" => {
                    config.verbose = value.parse()
                        .context("Invalid value for verbose (must be true or false)")?;
                }
                "hotkey" => {
                    // Validate hotkey format
                    Hotkey::parse(&value)
                        .map_err(|e| anyhow::anyhow!("Invalid hotkey format: {}. Use format like 'Ctrl+Shift+V'", e))?;
                    config.hotkey = value.clone();
                }
                "sensitive_detection" => {
                    config.sensitive_detection = value.parse()
                        .context("Invalid value for sensitive_detection (must be true or false)")?;
                }
                "auto_delete_sensitive" => {
                    config.auto_delete_sensitive = value.parse()
                        .context("Invalid value for auto_delete_sensitive (must be true or false)")?;
                }
                "enable_encryption" => {
                    config.enable_encryption = value.parse()
                        .context("Invalid value for enable_encryption (must be true or false)")?;
                }
                "retention_days" => {
                    config.retention_days = value.parse()
                        .context("Invalid value for retention_days (must be a number)")?;
                }
                _ => {
                    anyhow::bail!("Unknown configuration key: {}", key);
                }
            }
            save_config(&config)?;
            println!("Configuration updated: {} = {}", key, value);
        }
    }
    
    Ok(())
}

async fn cmd_hotkey(config: MorselConfig) -> Result<()> {
    use morsel_platform::HotkeyManager;
    use std::sync::Arc;
    
    info!("Starting hotkey listener with hotkey: {}", config.hotkey);
    
    let hotkey = Hotkey::parse(&config.hotkey)
        .map_err(|e| anyhow::anyhow!("Invalid hotkey configuration: {}", e))?;
    
    let hotkey_manager = HotkeyManager::new();
    let db_path = config.db_path.clone();
    
    let callback = Arc::new(move || {
        info!("Hotkey triggered! Opening TUI...");
        // Spawn TUI process
        if let Err(e) = std::process::Command::new("morsel-tui")
            .env("MORSEL_DB_PATH", &db_path)
            .spawn()
        {
            error!("Failed to spawn TUI: {}", e);
        }
    });
    
    hotkey_manager.register(hotkey, callback)
        .map_err(|e| anyhow::anyhow!("Failed to register hotkey: {}", e))?;
    
    println!("Hotkey listener started. Press {} to open TUI.", config.hotkey);
    println!("Press Ctrl+C to stop.");
    
    // Keep the process alive
    tokio::signal::ctrl_c().await?;
    println!("Hotkey listener stopped.");
    
    Ok(())
}

async fn cmd_cleanup(config: MorselConfig) -> Result<()> {
    use morsel_storage::SqliteStorage;
    
    info!("Starting cleanup with retention policy: {} days", config.retention_days);
    
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;
    
    // Clean up expired items
    let expired_count = storage.cleanup_expired().await?;
    println!("Cleaned up {} expired items", expired_count);
    
    // Clean up items based on retention policy
    if config.retention_days > 0 {
        let retention_count = storage.cleanup_by_retention(config.retention_days).await?;
        println!("Cleaned up {} items older than {} days", retention_count, config.retention_days);
    } else {
        println!("Retention policy is set to unlimited (0 days), skipping retention cleanup");
    }
    
    println!("Cleanup completed successfully");
    
    Ok(())
}

async fn cmd_export(config: MorselConfig, output: String) -> Result<()> {
    use morsel_storage::SqliteStorage;
    
    info!("Exporting clipboard data to {}", output);
    
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;
    
    let backup = storage.export_backup().await?;
    
    let backup_json = serde_json::to_string_pretty(&backup)
        .context("Failed to serialize backup")?;
    
    std::fs::write(&output, backup_json)
        .context("Failed to write backup file")?;
    
    println!("Exported {} items and {} blobs to {}", 
        backup.metadata.item_count, 
        backup.metadata.blob_count, 
        output);
    println!("Total size: {} bytes", backup.metadata.total_size);
    
    Ok(())
}

async fn cmd_import(config: MorselConfig, input: String) -> Result<()> {
    use morsel_storage::SqliteStorage;
    
    info!("Importing clipboard data from {}", input);
    
    let backup_json = std::fs::read_to_string(&input)
        .context("Failed to read backup file")?;
    
    let backup: morsel_core::Backup = serde_json::from_str(&backup_json)
        .context("Failed to parse backup file")?;
    
    // Validate backup
    backup.validate()
        .context("Backup validation failed")?;
    
    println!("Backup info:");
    println!("  Version: {}", backup.version);
    println!("  Created: {}", backup.created_at);
    println!("  Items: {}", backup.metadata.item_count);
    println!("  Blobs: {}", backup.metadata.blob_count);
    println!("  Total size: {} bytes", backup.metadata.total_size);
    println!("  Morsel version: {}", backup.metadata.morsel_version);
    
    let storage_config = StorageConfig {
        db_path: config.db_path,
        ..Default::default()
    };
    let storage = SqliteStorage::new(storage_config)?;
    storage.initialize().await?;
    
    storage.import_backup(backup).await?;
    
    println!("Import completed successfully");
    
    Ok(())
}
