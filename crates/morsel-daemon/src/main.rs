//! # morsel daemon
//!
//! Background daemon process for clipboard monitoring.

mod ipc;

use anyhow::Result;
use ipc::{IpcRequest, IpcResponse, IpcServer};
use morsel_clipboard::{ClipboardMonitor, MonitorConfig};
use morsel_platform::PlatformClipboard;
use morsel_storage::{SqliteStorage, StorageBackend, StorageConfig};
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Morsel daemon starting...");

    // Initialize storage
    let storage_config = StorageConfig {
        db_path: dirs::data_local_dir()
            .map(|p| p.join("morsel").join("morsel.db"))
            .unwrap_or_else(|| "morsel.db".into())
            .to_string_lossy()
            .to_string(),
        ..Default::default()
    };
    
    let storage = Arc::new(SqliteStorage::new(storage_config)?);
    storage.initialize().await?;

    // Initialize clipboard provider
    let clipboard = PlatformClipboard::new();

    // Initialize clipboard monitor
    let config = MonitorConfig {
        poll_interval_ms: 500,
        ..Default::default()
    };
    let (mut monitor, mut event_rx) = ClipboardMonitor::new(clipboard, config);

    // Start clipboard monitor in background
    let monitor_handle = tokio::spawn(async move {
        if let Err(e) = monitor.start().await {
            error!("Clipboard monitor error: {}", e);
        }
    });

    // Handle clipboard events
    let storage_clone = storage.clone();
    let event_handler = tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            if let Err(e) = handle_clipboard_event(&storage_clone, event).await {
                error!("Failed to handle clipboard event: {}", e);
            }
        }
    });

    // Start IPC server
    let ipc_addr = "127.0.0.1:54321";
    let ipc_server = IpcServer::new(ipc_addr)?;
    info!("IPC server listening on {}", ipc_addr);
    
    let storage_for_ipc = storage.clone();
    let ipc_handler = tokio::spawn(async move {
        if let Err(e) = handle_ipc_connections(ipc_server, storage_for_ipc).await {
            error!("IPC server error: {}", e);
        }
    });

    // Wait for shutdown signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down...");
        }
        _ = monitor_handle => {
            info!("Clipboard monitor stopped");
        }
        _ = event_handler => {
            info!("Event handler stopped");
        }
        _ = ipc_handler => {
            info!("IPC server stopped");
        }
    }

    info!("Morsel daemon stopped");
    Ok(())
}

async fn handle_clipboard_event(storage: &Arc<SqliteStorage>, event: morsel_clipboard::ClipboardEvent) -> Result<()> {
    match event {
        morsel_clipboard::ClipboardEvent::NewItem(item) => {
            storage.insert(&item).await?;
        }
        morsel_clipboard::ClipboardEvent::Cleared => {
            info!("Clipboard was cleared");
        }
        morsel_clipboard::ClipboardEvent::OwnershipChanged => {
            info!("Clipboard ownership changed");
        }
        morsel_clipboard::ClipboardEvent::Error(e) => {
            error!("Clipboard error: {}", e);
        }
    }
    Ok(())
}

async fn handle_ipc_connections(server: IpcServer, storage: Arc<SqliteStorage>) -> Result<()> {
    loop {
        match server.accept() {
            Ok(mut client) => {
                info!("Accepted connection from {}", client.addr());
                
                let storage_clone = storage.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_client(&mut client, storage_clone).await {
                        error!("Client handler error: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}

async fn handle_client(client: &mut ipc::ClientConnection, storage: Arc<SqliteStorage>) -> Result<()> {
    let request = client.read_request()?;
    let response = match request {
        IpcRequest::Status => {
            IpcResponse::Success {
                data: Some(serde_json::json!({
                    "status": "running",
                    "version": env!("CARGO_PKG_VERSION")
                }))
            }
        }
        IpcRequest::Stop => {
            IpcResponse::Success {
                data: Some(serde_json::json!({"message": "Stopping daemon"}))
            }
        }
        IpcRequest::GetHistory { limit } => {
            let items = storage.list().await?;
            let items: Vec<_> = items.into_iter().take(limit).collect();
            let items_json = serde_json::to_value(items)?;
            IpcResponse::Success {
                data: Some(items_json)
            }
        }
        IpcRequest::Search { query, limit } => {
            let items = storage.list().await?;
            let items: Vec<_> = items.into_iter()
                .filter(|item| item.content.to_lowercase().contains(&query.to_lowercase()))
                .take(limit)
                .collect();
            let items_json = serde_json::to_value(items)?;
            IpcResponse::Success {
                data: Some(items_json)
            }
        }
        IpcRequest::GetItem { id } => {
            use morsel_core::ItemId;
            match ItemId::from_str(&id) {
                Ok(item_id) => {
                    match storage.get(item_id).await {
                        Ok(item) => {
                            let item_json = serde_json::to_value(item)?;
                            IpcResponse::Success {
                                data: Some(item_json)
                            }
                        }
                        Err(e) => {
                            IpcResponse::Error {
                                message: format!("Failed to get item: {}", e)
                            }
                        }
                    }
                }
                Err(_) => {
                    IpcResponse::Error {
                        message: format!("Invalid item ID: {}", id)
                    }
                }
            }
        }
        IpcRequest::AddItem { content } => {
            use morsel_core::{ClipboardItem, ContentType, ItemId};
            use chrono::Utc;
            
            let item = ClipboardItem {
                id: ItemId::new(),
                content,
                content_type: ContentType::Text,
                created_at: Utc::now(),
                last_used_at: Utc::now(),
                size: 0,
                is_favorite: false,
                tags: vec![],
                collection_id: None,
                expires_at: None,
                source: Some("ipc".to_string()),
            };
            
            match storage.insert(&item).await {
                Ok(_) => {
                    IpcResponse::Success {
                        data: Some(serde_json::json!({"id": item.id.to_string()}))
                    }
                }
                Err(e) => {
                    IpcResponse::Error {
                        message: format!("Failed to add item: {}", e)
                    }
                }
            }
        }
        IpcRequest::DeleteItem { id } => {
            use morsel_core::ItemId;
            match ItemId::from_str(&id) {
                Ok(item_id) => {
                    match storage.delete(item_id).await {
                        Ok(_) => {
                            IpcResponse::Success {
                                data: Some(serde_json::json!({"message": "Item deleted"}))
                            }
                        }
                        Err(e) => {
                            IpcResponse::Error {
                                message: format!("Failed to delete item: {}", e)
                            }
                        }
                    }
                }
                Err(_) => {
                    IpcResponse::Error {
                        message: format!("Invalid item ID: {}", id)
                    }
                }
            }
        }
        IpcRequest::ClearHistory => {
            match storage.clear().await {
                Ok(_) => {
                    IpcResponse::Success {
                        data: Some(serde_json::json!({"message": "History cleared"}))
                    }
                }
                Err(e) => {
                    IpcResponse::Error {
                        message: format!("Failed to clear history: {}", e)
                    }
                }
            }
        }
    };
    
    client.write_response(&response)?;
    Ok(())
}
