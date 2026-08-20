//! Retention tests for morsel-core.

use morsel_core::{ClipboardItem, ItemId};
use chrono::{Utc, Duration};

#[test]
fn test_clipboard_item_expiration() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Item without expiration should not be expired
    assert!(!item.is_expired());
    
    // Set expiration in the future
    let future = Utc::now() + Duration::hours(1);
    item.set_expires_at(Some(future));
    assert!(!item.is_expired());
    
    // Set expiration in the past
    let past = Utc::now() - Duration::hours(1);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_exact() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set expiration to exactly now
    let now = Utc::now();
    item.set_expires_at(Some(now));
    
    // Should be expired since now >= expiration time
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_clear() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set expiration
    let future = Utc::now() + Duration::hours(1);
    item.set_expires_at(Some(future));
    assert!(!item.is_expired());
    
    // Clear expiration
    item.set_expires_at(None);
    assert!(!item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_with_seconds() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set expiration 30 seconds in the future
    let future = Utc::now() + Duration::seconds(30);
    item.set_expires_at(Some(future));
    assert!(!item.is_expired());
    
    // Set expiration 30 seconds in the past
    let past = Utc::now() - Duration::seconds(30);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_with_days() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set expiration 7 days in the future
    let future = Utc::now() + Duration::days(7);
    item.set_expires_at(Some(future));
    assert!(!item.is_expired());
    
    // Set expiration 7 days in the past
    let past = Utc::now() - Duration::days(7);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_zero_duration() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set expiration to now (zero duration from now)
    let now = Utc::now();
    item.set_expires_at(Some(now));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_negative_duration() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set expiration in the past (negative duration)
    let past = Utc::now() - Duration::milliseconds(1);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_very_future() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set expiration far in the future (1 year)
    let future = Utc::now() + Duration::days(365);
    item.set_expires_at(Some(future));
    assert!(!item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_very_past() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set expiration far in the past (1 year)
    let past = Utc::now() - Duration::days(365);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_persistence() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    let expiration = Utc::now() + Duration::hours(2);
    item.set_expires_at(Some(expiration));
    
    // Verify the expiration is set correctly
    assert_eq!(item.expires_at, Some(expiration));
}

#[test]
fn test_clipboard_item_expiration_with_content_type() {
    let mut item = ClipboardItem::new("https://example.com".to_string());
    
    // Expiration should work regardless of content type
    let past = Utc::now() - Duration::hours(1);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_with_tags() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("important".to_string());
    
    // Expiration should work regardless of tags
    let past = Utc::now() - Duration::hours(1);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_with_favorite() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.set_favorite(true);
    
    // Expiration should work regardless of favorite status
    let past = Utc::now() - Duration::hours(1);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_with_collection() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    let collection_id = ItemId::new();
    item.set_collection(Some(collection_id));
    
    // Expiration should work regardless of collection
    let past = Utc::now() - Duration::hours(1);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_multiple_updates() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set multiple expirations
    item.set_expires_at(Some(Utc::now() + Duration::hours(1)));
    assert!(!item.is_expired());
    
    item.set_expires_at(Some(Utc::now() + Duration::hours(2)));
    assert!(!item.is_expired());
    
    item.set_expires_at(Some(Utc::now() - Duration::hours(1)));
    assert!(item.is_expired());
}

#[test]
fn test_clipboard_item_expiration_edge_case() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    // Set expiration 1 nanosecond in the past
    let past = Utc::now() - Duration::nanoseconds(1);
    item.set_expires_at(Some(past));
    assert!(item.is_expired());
}
