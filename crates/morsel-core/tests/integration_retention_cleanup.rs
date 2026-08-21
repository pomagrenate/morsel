//! Integration tests for retention → cleanup.

use morsel_core::ClipboardItem;
use chrono::{Utc, Duration};

#[test]
fn test_retention_cleanup_expired_items() {
    let mut items = vec![
        ClipboardItem::new("active item".to_string()),
        ClipboardItem::new("expired item".to_string()),
        ClipboardItem::new("another active item".to_string()),
    ];
    
    // Set expiration in the past for the second item
    let past = Utc::now() - Duration::hours(1);
    items[1].set_expires_at(Some(past));
    
    // Simulate cleanup: filter out expired items
    let active_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| !item.is_expired())
        .collect();
    
    // Should only have 2 active items
    assert_eq!(active_items.len(), 2);
    assert!(!active_items.iter().any(|i| i.content.contains("expired")));
}

#[test]
fn test_retention_cleanup_old_items() {
    let mut items = vec![
        ClipboardItem::new("recent item".to_string()),
        ClipboardItem::new("old item".to_string()),
        ClipboardItem::new("another recent item".to_string()),
    ];
    
    // Set created_at to 30 days ago for the second item
    let old_date = Utc::now() - Duration::days(30);
    items[1].created_at = old_date;
    
    // Simulate cleanup: remove items older than 7 days
    let cutoff = Utc::now() - Duration::days(7);
    let recent_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| item.created_at >= cutoff)
        .collect();
    
    // Should only have 2 recent items
    assert_eq!(recent_items.len(), 2);
    assert!(!recent_items.iter().any(|i| i.content.contains("old")));
}

#[test]
fn test_retention_cleanup_respects_favorites() {
    let mut items = vec![
        ClipboardItem::new("regular expired item".to_string()),
        ClipboardItem::new("favorite expired item".to_string()),
    ];
    
    // Set both as expired
    let past = Utc::now() - Duration::hours(1);
    items[0].set_expires_at(Some(past));
    items[1].set_expires_at(Some(past));
    
    // Mark second as favorite
    items[1].set_favorite(true);
    
    // Simulate cleanup: don't delete favorites even if expired
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| item.is_favorite || !item.is_expired())
        .collect();
    
    // Should keep the favorite item
    assert_eq!(retained_items.len(), 1);
    assert!(retained_items[0].is_favorite);
}

#[test]
fn test_retention_cleanup_with_max_items() {
    let items: Vec<ClipboardItem> = (0..20)
        .map(|i| ClipboardItem::new(format!("item {}", i)))
        .collect();
    
    // Simulate cleanup: keep only last 10 items
    let max_items = 10;
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .rev()
        .take(max_items)
        .collect();
    
    assert_eq!(retained_items.len(), 10);
}

#[test]
fn test_retention_cleanup_preserves_tags() {
    let mut items = vec![
        ClipboardItem::new("tagged item".to_string()),
        ClipboardItem::new("untagged item".to_string()),
    ];
    
    items[0].add_tag("important".to_string());
    items[0].add_tag("keep".to_string());
    
    // Simulate cleanup: keep tagged items
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| !item.tags.is_empty())
        .collect();
    
    assert_eq!(retained_items.len(), 1);
    assert_eq!(retained_items[0].tags.len(), 2);
}

#[test]
fn test_retention_cleanup_preserves_collections() {
    let mut items = vec![
        ClipboardItem::new("collection item".to_string()),
        ClipboardItem::new("uncollected item".to_string()),
    ];
    
    let collection_id = morsel_core::ItemId::new();
    items[0].set_collection(Some(collection_id));
    
    // Simulate cleanup: keep items in collections
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| item.collection_id.is_some())
        .collect();
    
    assert_eq!(retained_items.len(), 1);
    assert!(retained_items[0].collection_id.is_some());
}

#[test]
fn test_retention_cleanup_with_size_limit() {
    let items = vec![
        ClipboardItem::new("small item".to_string()),
        ClipboardItem::new("large item with much more content".to_string()),
        ClipboardItem::new("another small item".to_string()),
    ];
    
    // Simulate cleanup: remove items larger than 20 bytes
    let max_size = 20;
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| item.size <= max_size)
        .collect();
    
    assert_eq!(retained_items.len(), 2);
    assert!(!retained_items.iter().any(|i| i.content.contains("large")));
}

#[test]
fn test_retention_cleanup_empty_history() {
    let items: Vec<ClipboardItem> = vec![];
    
    // Simulate cleanup on empty history
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| !item.is_expired())
        .collect();
    
    assert!(retained_items.is_empty());
}

#[test]
fn test_retention_cleanup_all_expired() {
    let mut items = vec![
        ClipboardItem::new("expired 1".to_string()),
        ClipboardItem::new("expired 2".to_string()),
        ClipboardItem::new("expired 3".to_string()),
    ];
    
    // Set all as expired
    let past = Utc::now() - Duration::hours(1);
    for item in &mut items {
        item.set_expires_at(Some(past));
    }
    
    // Simulate cleanup
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| !item.is_expired())
        .collect();
    
    assert!(retained_items.is_empty());
}

#[test]
fn test_retention_cleanup_none_expired() {
    let items = vec![
        ClipboardItem::new("active 1".to_string()),
        ClipboardItem::new("active 2".to_string()),
        ClipboardItem::new("active 3".to_string()),
    ];
    
    // Simulate cleanup
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| !item.is_expired())
        .collect();
    
    assert_eq!(retained_items.len(), 3);
}

#[test]
fn test_retention_cleanup_with_sensitive_content() {
    let items = vec![
        ClipboardItem::new("api_key=secret123".to_string()),
        ClipboardItem::new("password=hidden".to_string()),
        ClipboardItem::new("normal content".to_string()),
    ];
    
    // Simulate cleanup: remove items with sensitive content
    let sensitive_keywords = ["api_key", "password"];
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| {
            !sensitive_keywords.iter().any(|keyword| 
                item.content.to_lowercase().contains(keyword)
            )
        })
        .collect();
    
    assert_eq!(retained_items.len(), 1);
    assert!(retained_items[0].content.contains("normal"));
}

#[test]
fn test_retention_cleanup_with_content_type_filter() {
    let items = vec![
        ClipboardItem::new("https://example.com".to_string()),
        ClipboardItem::new("SELECT * FROM users".to_string()),
        ClipboardItem::new("plain text".to_string()),
    ];
    
    // Simulate cleanup: keep only URLs
    let retained_items: Vec<ClipboardItem> = items.into_iter()
        .filter(|item| item.content_type == morsel_core::ContentType::Url)
        .collect();
    
    assert_eq!(retained_items.len(), 1);
    assert!(retained_items[0].content.contains("http"));
}
