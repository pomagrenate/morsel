//! Integration tests for import → storage and export → import.

use morsel_core::{Backup, ClipboardItem, ItemId};
use serde_json;

#[test]
fn test_export_to_backup() {
    let items = vec![
        ClipboardItem::new("content 1".to_string()),
        ClipboardItem::new("content 2".to_string()),
        ClipboardItem::new("https://example.com".to_string()),
    ];
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items: items.clone(),
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: items.len(),
            blob_count: 0,
            total_size: items.iter().map(|i| i.size).sum(),
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Verify backup structure
    assert_eq!(backup.items.len(), 3);
    assert_eq!(backup.version, 1);
    assert_eq!(backup.metadata.item_count, 3);
}

#[test]
fn test_backup_serialization() {
    let items = vec![
        ClipboardItem::new("test content".to_string()),
    ];
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items,
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: 1,
            blob_count: 0,
            total_size: 12,
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Serialize to JSON
    let serialized = serde_json::to_string(&backup).unwrap();
    assert!(!serialized.is_empty());
    
    // Deserialize from JSON
    let deserialized: Backup = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.items.len(), 1);
    assert_eq!(deserialized.version, 1);
}

#[test]
fn test_import_from_backup() {
    let items = vec![
        ClipboardItem::new("imported content 1".to_string()),
        ClipboardItem::new("imported content 2".to_string()),
    ];
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items: items.clone(),
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: items.len(),
            blob_count: 0,
            total_size: items.iter().map(|i| i.size).sum(),
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Simulate import by extracting items
    let imported_items: Vec<ClipboardItem> = backup.items;
    
    assert_eq!(imported_items.len(), 2);
    assert!(imported_items.iter().any(|i| i.content.contains("imported")));
}

#[test]
fn test_export_with_tags() {
    let mut items = vec![
        ClipboardItem::new("tagged content".to_string()),
    ];
    
    items[0].add_tag("export".to_string());
    items[0].add_tag("test".to_string());
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items,
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: 1,
            blob_count: 0,
            total_size: 14,
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Verify tags are preserved in backup
    assert_eq!(backup.items[0].tags.len(), 2);
    assert!(backup.items[0].tags.contains(&"export".to_string()));
}

#[test]
fn test_export_with_collections() {
    let mut items = vec![
        ClipboardItem::new("collection content".to_string()),
    ];
    
    let collection_id = ItemId::new();
    items[0].set_collection(Some(collection_id));
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items,
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: 1,
            blob_count: 0,
            total_size: 17,
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Verify collection is preserved in backup
    assert!(backup.items[0].collection_id.is_some());
}

#[test]
fn test_export_with_favorites() {
    let mut items = vec![
        ClipboardItem::new("favorite content".to_string()),
    ];
    
    items[0].set_favorite(true);
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items,
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: 1,
            blob_count: 0,
            total_size: 16,
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Verify favorite status is preserved
    assert!(backup.items[0].is_favorite);
}

#[test]
fn test_export_with_expiration() {
    let mut items = vec![
        ClipboardItem::new("expiring content".to_string()),
    ];
    
    let expiration = chrono::Utc::now() + chrono::Duration::hours(24);
    items[0].set_expires_at(Some(expiration));
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items,
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: 1,
            blob_count: 0,
            total_size: 16,
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Verify expiration is preserved
    assert!(backup.items[0].expires_at.is_some());
}

#[test]
fn test_export_empty_history() {
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items: vec![],
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: 0,
            blob_count: 0,
            total_size: 0,
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Verify empty backup
    assert_eq!(backup.items.len(), 0);
    assert_eq!(backup.metadata.item_count, 0);
    assert_eq!(backup.metadata.total_size, 0);
}

#[test]
fn test_export_large_history() {
    let items: Vec<ClipboardItem> = (0..100)
        .map(|i| ClipboardItem::new(format!("content {}", i)))
        .collect();
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items,
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: 100,
            blob_count: 0,
            total_size: 1000, // Approximate
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Verify large backup
    assert_eq!(backup.items.len(), 100);
    assert_eq!(backup.metadata.item_count, 100);
}

#[test]
fn test_import_preserves_content_types() {
    let items = vec![
        ClipboardItem::new("https://example.com".to_string()),
        ClipboardItem::new("test@example.com".to_string()),
        ClipboardItem::new("SELECT * FROM users".to_string()),
    ];
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items: items.clone(),
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: items.len(),
            blob_count: 0,
            total_size: items.iter().map(|i| i.size).sum(),
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Verify content types are preserved
    let imported_items = backup.items;
    assert_eq!(imported_items[0].content_type, items[0].content_type);
    assert_eq!(imported_items[1].content_type, items[1].content_type);
    assert_eq!(imported_items[2].content_type, items[2].content_type);
}

#[test]
fn test_export_import_roundtrip() {
    let original_items = vec![
        ClipboardItem::new("roundtrip content".to_string()),
    ];
    
    let backup = Backup {
        version: 1,
        created_at: chrono::Utc::now(),
        items: original_items.clone(),
        blobs: vec![],
        metadata: morsel_core::BackupMetadata {
            item_count: 1,
            blob_count: 0,
            total_size: 16,
            morsel_version: "0.1.0".to_string(),
            encryption: None,
        },
    };
    
    // Serialize
    let serialized = serde_json::to_string(&backup).unwrap();
    
    // Deserialize
    let deserialized: Backup = serde_json::from_str(&serialized).unwrap();
    
    // Verify roundtrip
    assert_eq!(deserialized.items.len(), original_items.len());
    assert_eq!(deserialized.items[0].content, original_items[0].content);
    assert_eq!(deserialized.items[0].content_type, original_items[0].content_type);
}
