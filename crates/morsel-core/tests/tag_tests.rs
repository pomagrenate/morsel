//! Tag tests for morsel-core.

use morsel_core::ClipboardItem;

#[test]
fn test_clipboard_item_add_tag() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("tag1".to_string());
    assert_eq!(item.tags.len(), 1);
    assert!(item.tags.contains(&"tag1".to_string()));
}

#[test]
fn test_clipboard_item_remove_tag() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("tag1".to_string());
    item.add_tag("tag2".to_string());
    
    item.remove_tag("tag1");
    assert_eq!(item.tags.len(), 1);
    assert!(!item.tags.contains(&"tag1".to_string()));
    assert!(item.tags.contains(&"tag2".to_string()));
}

#[test]
fn test_clipboard_item_has_tag() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("tag1".to_string());
    assert!(item.has_tag("tag1"));
    assert!(!item.has_tag("tag2"));
}

#[test]
fn test_clipboard_item_clear_tags() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("tag1".to_string());
    item.add_tag("tag2".to_string());
    
    item.clear_tags();
    assert!(item.tags.is_empty());
}

#[test]
fn test_clipboard_item_duplicate_tags() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("tag1".to_string());
    item.add_tag("tag1".to_string());
    
    // Should not add duplicate
    assert_eq!(item.tags.len(), 1);
}

#[test]
fn test_clipboard_item_empty_tag() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("".to_string());
    
    // Empty tags are allowed (implementation doesn't filter them)
    assert!(item.tags.contains(&"".to_string()));
}

#[test]
fn test_clipboard_item_tag_with_spaces() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("tag with spaces".to_string());
    assert!(item.tags.contains(&"tag with spaces".to_string()));
}

#[test]
fn test_clipboard_item_tag_with_special_chars() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("tag-with-special_chars".to_string());
    assert!(item.tags.contains(&"tag-with-special_chars".to_string()));
}

#[test]
fn test_clipboard_item_unicode_tags() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("标签".to_string());
    item.add_tag("タグ".to_string());
    
    assert_eq!(item.tags.len(), 2);
    assert!(item.tags.contains(&"标签".to_string()));
    assert!(item.tags.contains(&"タグ".to_string()));
}

#[test]
fn test_clipboard_item_multiple_tags() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("tag1".to_string());
    item.add_tag("tag2".to_string());
    item.add_tag("tag3".to_string());
    
    assert_eq!(item.tags.len(), 3);
}

#[test]
fn test_clipboard_item_tag_case_sensitivity() {
    let mut item = ClipboardItem::new("test content".to_string());
    
    item.add_tag("Tag1".to_string());
    item.add_tag("tag1".to_string());
    
    // Tags are case-sensitive
    assert_eq!(item.tags.len(), 2);
}
