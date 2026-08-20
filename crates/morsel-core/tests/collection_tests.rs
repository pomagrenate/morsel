//! Collection tests for morsel-core.

use morsel_core::{ClipboardItem, Collection, ItemId};

#[test]
fn test_collection_creation() {
    let collection = Collection::new("Test Collection".to_string());
    
    assert_eq!(collection.name, "Test Collection");
    assert!(collection.description.is_none());
    assert!(collection.tags.is_empty());
    assert!(!collection.is_favorite);
    assert!(collection.item_ids.is_empty());
}

#[test]
fn test_collection_with_description() {
    let collection = Collection::new("Test Collection".to_string());
    let collection = collection.with_description("A test collection".to_string());
    
    assert_eq!(collection.description, Some("A test collection".to_string()));
}

#[test]
fn test_collection_add_item() {
    let mut collection = Collection::new("Test Collection".to_string());
    let item_id = ItemId::new();
    
    collection.add_item(item_id);
    
    assert_eq!(collection.item_ids.len(), 1);
    assert!(collection.item_ids.contains(&item_id));
}

#[test]
fn test_collection_remove_item() {
    let mut collection = Collection::new("Test Collection".to_string());
    let item_id1 = ItemId::new();
    let item_id2 = ItemId::new();
    
    collection.add_item(item_id1);
    collection.add_item(item_id2);
    
    collection.remove_item(item_id1);
    
    assert_eq!(collection.item_ids.len(), 1);
    assert!(!collection.item_ids.contains(&item_id1));
    assert!(collection.item_ids.contains(&item_id2));
}

#[test]
fn test_collection_has_item() {
    let mut collection = Collection::new("Test Collection".to_string());
    let item_id = ItemId::new();
    
    collection.add_item(item_id);
    
    assert!(collection.has_item(item_id));
    assert!(!collection.has_item(ItemId::new()));
}

#[test]
fn test_collection_item_count() {
    let mut collection = Collection::new("Test Collection".to_string());
    
    assert_eq!(collection.item_count(), 0);
    
    collection.add_item(ItemId::new());
    collection.add_item(ItemId::new());
    
    assert_eq!(collection.item_count(), 2);
}

#[test]
fn test_collection_clear_items() {
    let mut collection = Collection::new("Test Collection".to_string());
    
    collection.add_item(ItemId::new());
    collection.add_item(ItemId::new());
    
    collection.clear_items();
    
    assert!(collection.item_ids.is_empty());
}

#[test]
fn test_collection_duplicate_items() {
    let mut collection = Collection::new("Test Collection".to_string());
    let item_id = ItemId::new();
    
    collection.add_item(item_id);
    collection.add_item(item_id);
    
    // Should not add duplicate
    assert_eq!(collection.item_ids.len(), 1);
}

#[test]
fn test_collection_set_favorite() {
    let mut collection = Collection::new("Test Collection".to_string());
    
    collection.set_favorite(true);
    assert!(collection.is_favorite);
    
    collection.set_favorite(false);
    assert!(!collection.is_favorite);
}

#[test]
fn test_collection_add_tag() {
    let mut collection = Collection::new("Test Collection".to_string());
    
    collection.add_tag("work".to_string());
    assert!(collection.tags.contains(&"work".to_string()));
}

#[test]
fn test_collection_remove_tag() {
    let mut collection = Collection::new("Test Collection".to_string());
    
    collection.add_tag("work".to_string());
    collection.add_tag("personal".to_string());
    
    collection.remove_tag("work");
    
    assert!(!collection.tags.contains(&"work".to_string()));
    assert!(collection.tags.contains(&"personal".to_string()));
}

#[test]
fn test_clipboard_item_set_collection() {
    let mut item = ClipboardItem::new("test content".to_string());
    let collection_id = ItemId::new();
    
    item.set_collection(Some(collection_id));
    
    assert_eq!(item.collection_id, Some(collection_id));
}

#[test]
fn test_clipboard_item_clear_collection() {
    let mut item = ClipboardItem::new("test content".to_string());
    let collection_id = ItemId::new();
    
    item.set_collection(Some(collection_id));
    item.set_collection(None);
    
    assert!(item.collection_id.is_none());
}

#[test]
fn test_collection_empty_name() {
    let collection = Collection::new("".to_string());
    
    assert_eq!(collection.name, "");
}

#[test]
fn test_collection_unicode_name() {
    let collection = Collection::new("测试集合".to_string());
    
    assert_eq!(collection.name, "测试集合");
}

#[test]
fn test_collection_multiple_items() {
    let mut collection = Collection::new("Test Collection".to_string());
    
    for _ in 0..10 {
        collection.add_item(ItemId::new());
    }
    
    assert_eq!(collection.item_count(), 10);
}

#[test]
fn test_collection_is_empty() {
    let collection = Collection::new("Test Collection".to_string());
    
    assert!(collection.is_empty());
    
    let mut collection = Collection::new("Test Collection".to_string());
    collection.add_item(ItemId::new());
    
    assert!(!collection.is_empty());
}
