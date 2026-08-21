//! Integration tests for clipboard → storage.

use morsel_clipboard::{ClipboardMonitor, ClipboardProvider, InMemoryClipboard, MonitorConfig};
use morsel_storage::{SqliteStorage, StorageBackend, StorageConfig};
use std::sync::Arc;
use tempfile::NamedTempFile;
use tokio::time::{sleep, Duration};

async fn create_test_storage() -> SqliteStorage {
    let temp_file = NamedTempFile::new().unwrap();
    let config = StorageConfig {
        db_path: temp_file.path().to_str().unwrap().to_string(),
        max_connections: 10,
        enable_wal: false,
    };
    let storage = SqliteStorage::new(config).unwrap();
    storage.initialize().await.unwrap();
    storage
}

#[tokio::test]
async fn test_clipboard_event_to_storage() {
    let storage = create_test_storage().await;
    let clipboard = Arc::new(InMemoryClipboard::new());
    
    let config = MonitorConfig {
        poll_interval_ms: 100,
        ignore_duplicates: false,
        ..Default::default()
    };
    
    let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
    monitor.start().await.unwrap();
    
    // Simulate clipboard change
    clipboard.set_text("test content".to_string()).await.unwrap();
    sleep(Duration::from_millis(150)).await;
    
    // Receive clipboard event
    let event = receiver.recv().await.unwrap();
    
    // Store the clipboard item
    if let morsel_clipboard::ClipboardEvent::NewItem(item) = event {
        storage.insert(&item).await.unwrap();
        
        // Verify it was stored
        let retrieved = storage.get(item.id).await.unwrap();
        assert_eq!(retrieved.content, "test content");
    } else {
        panic!("Expected NewItem event");
    }
    
    monitor.stop().await;
}

#[tokio::test]
async fn test_clipboard_content_type_preserved_in_storage() {
    let storage = create_test_storage().await;
    let clipboard = Arc::new(InMemoryClipboard::new());
    
    let config = MonitorConfig {
        poll_interval_ms: 100,
        ignore_duplicates: false,
        ..Default::default()
    };
    
    let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
    monitor.start().await.unwrap();
    
    // Set URL content
    clipboard.set_text("https://example.com".to_string()).await.unwrap();
    sleep(Duration::from_millis(150)).await;
    
    let event = receiver.recv().await.unwrap();
    
    if let morsel_clipboard::ClipboardEvent::NewItem(item) = event {
        storage.insert(&item).await.unwrap();
        
        let retrieved = storage.get(item.id).await.unwrap();
        assert_eq!(retrieved.content_type, item.content_type);
    } else {
        panic!("Expected NewItem event");
    }
    
    monitor.stop().await;
}

#[tokio::test]
async fn test_clipboard_duplicate_filtering_with_storage() {
    let storage = create_test_storage().await;
    let clipboard = Arc::new(InMemoryClipboard::new());
    
    let config = MonitorConfig {
        poll_interval_ms: 100,
        ignore_duplicates: true,
        ..Default::default()
    };
    
    let (mut monitor, mut receiver) = ClipboardMonitor::new(clipboard.clone(), config);
    monitor.start().await.unwrap();
    
    // Set same content twice
    clipboard.set_text("duplicate content".to_string()).await.unwrap();
    sleep(Duration::from_millis(150)).await;
    
    clipboard.set_text("duplicate content".to_string()).await.unwrap();
    sleep(Duration::from_millis(150)).await;
    
    // Should only receive one event due to duplicate filtering
    let mut event_count = 0;
    while let Ok(event) = receiver.try_recv() {
        if let morsel_clipboard::ClipboardEvent::NewItem(item) = event {
            storage.insert(&item).await.unwrap();
            event_count += 1;
        }
    }
    
    monitor.stop().await;
    
    // Should have stored only one item
    assert_eq!(event_count, 1);
    assert_eq!(storage.count().await.unwrap(), 1);
}
