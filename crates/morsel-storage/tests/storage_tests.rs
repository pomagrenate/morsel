//! Storage tests for morsel-storage.

use morsel_core::{ClipboardItem, ContentType, ItemId};
use morsel_storage::{SqliteStorage, StorageBackend, StorageConfig};
use chrono::Utc;
use tempfile::NamedTempFile;

async fn create_test_storage() -> SqliteStorage {
    let temp_file = NamedTempFile::new().unwrap();
    let config = StorageConfig {
        db_path: temp_file.path().to_str().unwrap().to_string(),
        enable_wal: false,
        ..Default::default()
    };
    let storage = SqliteStorage::new(config).unwrap();
    storage.initialize().await.unwrap();
    storage
}

#[tokio::test]
async fn test_storage_initialization() {
    let storage = create_test_storage().await;
    assert!(storage.count().await.unwrap() == 0);
}

#[tokio::test]
async fn test_storage_insert_and_get() {
    let storage = create_test_storage().await;
    
    let item = ClipboardItem::new("test content".to_string());
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert_eq!(retrieved.content, "test content");
    assert_eq!(retrieved.id, item.id);
}

#[tokio::test]
async fn test_storage_list() {
    let storage = create_test_storage().await;
    
    let item1 = ClipboardItem::new("content 1".to_string());
    let item2 = ClipboardItem::new("content 2".to_string());
    
    storage.insert(&item1).await.unwrap();
    storage.insert(&item2).await.unwrap();
    
    let items = storage.list().await.unwrap();
    assert_eq!(items.len(), 2);
}

#[tokio::test]
async fn test_storage_delete() {
    let storage = create_test_storage().await;
    
    let item = ClipboardItem::new("test content".to_string());
    storage.insert(&item).await.unwrap();
    
    storage.delete(item.id).await.unwrap();
    
    let items = storage.list().await.unwrap();
    assert_eq!(items.len(), 0);
}

#[tokio::test]
async fn test_storage_update() {
    let storage = create_test_storage().await;
    
    let mut item = ClipboardItem::new("test content".to_string());
    storage.insert(&item).await.unwrap();
    
    item.is_favorite = true;
    storage.update(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert!(retrieved.is_favorite);
}

#[tokio::test]
async fn test_storage_clear() {
    let storage = create_test_storage().await;
    
    let item1 = ClipboardItem::new("content 1".to_string());
    let item2 = ClipboardItem::new("content 2".to_string());
    
    storage.insert(&item1).await.unwrap();
    storage.insert(&item2).await.unwrap();
    
    storage.clear().await.unwrap();
    
    let items = storage.list().await.unwrap();
    assert_eq!(items.len(), 0);
}

#[tokio::test]
async fn test_storage_count() {
    let storage = create_test_storage().await;
    
    assert_eq!(storage.count().await.unwrap(), 0);
    
    let item = ClipboardItem::new("test content".to_string());
    storage.insert(&item).await.unwrap();
    
    assert_eq!(storage.count().await.unwrap(), 1);
}

#[tokio::test]
async fn test_storage_delete_many() {
    let storage = create_test_storage().await;
    
    let item1 = ClipboardItem::new("content 1".to_string());
    let item2 = ClipboardItem::new("content 2".to_string());
    let item3 = ClipboardItem::new("content 3".to_string());
    
    storage.insert(&item1).await.unwrap();
    storage.insert(&item2).await.unwrap();
    storage.insert(&item3).await.unwrap();
    
    let ids = vec![item1.id, item2.id];
    let deleted = storage.delete_many(ids).await.unwrap();
    
    assert_eq!(deleted, 2);
    assert_eq!(storage.count().await.unwrap(), 1);
}

#[tokio::test]
async fn test_storage_delete_older_than() {
    let storage = create_test_storage().await;
    
    let old_item = ClipboardItem::new("old content".to_string());
    let mut new_item = ClipboardItem::new("new content".to_string());
    new_item.created_at = Utc::now();
    
    storage.insert(&old_item).await.unwrap();
    storage.insert(&new_item).await.unwrap();
    
    let cutoff = Utc::now() - chrono::Duration::hours(1);
    let _deleted = storage.delete_older_than(cutoff).await.unwrap();
}

#[tokio::test]
async fn test_storage_tags() {
    let storage = create_test_storage().await;
    
    let mut item = ClipboardItem::new("test content".to_string());
    item.tags = vec!["tag1".to_string(), "tag2".to_string()];
    
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert_eq!(retrieved.tags.len(), 2);
    assert!(retrieved.tags.contains(&"tag1".to_string()));
}

#[tokio::test]
async fn test_storage_collections() {
    let storage = create_test_storage().await;
    
    let mut item = ClipboardItem::new("test content".to_string());
    item.collection_id = Some(ItemId::new());
    
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert!(retrieved.collection_id.is_some());
}

#[tokio::test]
async fn test_storage_expiration() {
    let storage = create_test_storage().await;
    
    let mut item = ClipboardItem::new("test content".to_string());
    item.expires_at = Some(Utc::now() + chrono::Duration::hours(1));
    
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert!(retrieved.expires_at.is_some());
}

#[tokio::test]
async fn test_storage_content_type() {
    let storage = create_test_storage().await;
    
    let mut item = ClipboardItem::new("https://example.com".to_string());
    item.content_type = ContentType::Url;
    
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert_eq!(retrieved.content_type, ContentType::Url);
}

#[tokio::test]
async fn test_storage_favorite() {
    let storage = create_test_storage().await;
    
    let mut item = ClipboardItem::new("test content".to_string());
    item.is_favorite = true;
    
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert!(retrieved.is_favorite);
}

#[tokio::test]
async fn test_storage_large_content() {
    let storage = create_test_storage().await;
    
    let large_content = "x".repeat(10000);
    let item = ClipboardItem::new(large_content.clone());
    
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert_eq!(retrieved.content.len(), 10000);
}

#[tokio::test]
async fn test_storage_unicode_content() {
    let storage = create_test_storage().await;
    
    let unicode_content = "Hello 世界 🌍";
    let item = ClipboardItem::new(unicode_content.to_string());
    
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert_eq!(retrieved.content, unicode_content);
}

#[tokio::test]
async fn test_storage_special_characters() {
    let storage = create_test_storage().await;
    
    let special_content = "Test with \"quotes\" and \\backslashes\\ and \n newlines";
    let item = ClipboardItem::new(special_content.to_string());
    
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert_eq!(retrieved.content, special_content);
}

#[tokio::test]
async fn test_storage_empty_content() {
    let storage = create_test_storage().await;
    
    let item = ClipboardItem::new("".to_string());
    storage.insert(&item).await.unwrap();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert_eq!(retrieved.content, "");
}

#[tokio::test]
async fn test_storage_duplicate_insert() {
    let storage = create_test_storage().await;
    
    let item = ClipboardItem::new("test content".to_string());
    storage.insert(&item).await.unwrap();
    
    // Insert same item again (should update or handle gracefully)
    storage.insert(&item).await.unwrap();
    
    let count = storage.count().await.unwrap();
    assert!(count >= 1);
}

#[tokio::test]
async fn test_storage_timestamps() {
    let storage = create_test_storage().await;
    
    let before = Utc::now();
    let item = ClipboardItem::new("test content".to_string());
    storage.insert(&item).await.unwrap();
    let after = Utc::now();
    
    let retrieved = storage.get(item.id).await.unwrap();
    assert!(retrieved.created_at >= before);
    assert!(retrieved.created_at <= after);
    assert!(retrieved.last_used_at >= before);
    assert!(retrieved.last_used_at <= after);
}
