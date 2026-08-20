//! Integration tests for storage → search.

use morsel_core::ClipboardItem;
use morsel_search::{SearchEngine, SearchQuery};

#[test]
fn test_storage_items_to_search_index() {
    let items = vec![
        ClipboardItem::new("https://example.com".to_string()),
        ClipboardItem::new("test@example.com".to_string()),
        ClipboardItem::new("SELECT * FROM users".to_string()),
        ClipboardItem::new("plain text content".to_string()),
    ];
    
    let mut engine = SearchEngine::new();
    
    // Index items (simulating retrieval from storage)
    for item in items {
        engine.index_item(item);
    }
    
    // Verify search works
    let query = SearchQuery::new("example".to_string());
    let results = engine.search(&query).unwrap();
    
    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.item.content.contains("example")));
}

#[test]
fn test_storage_large_dataset_to_search() {
    let items: Vec<ClipboardItem> = (0..100)
        .map(|i| ClipboardItem::new(format!("test content {}", i)))
        .collect();
    
    let mut engine = SearchEngine::new();
    
    // Index all items
    for item in items {
        engine.index_item(item);
    }
    
    // Verify search performance
    let query = SearchQuery::new("test".to_string());
    let start = std::time::Instant::now();
    let results = engine.search(&query).unwrap();
    let duration = start.elapsed();
    
    assert!(!results.is_empty());
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_storage_search_with_content_types() {
    let items = vec![
        ClipboardItem::new("https://example.com".to_string()),
        ClipboardItem::new("test@example.com".to_string()),
        ClipboardItem::new("SELECT * FROM users".to_string()),
        ClipboardItem::new("fn main() {}".to_string()),
    ];
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    // Search with content type filter
    let mut query = SearchQuery::new("http".to_string());
    query.content_type = Some(morsel_core::ContentType::Url);
    let results = engine.search(&query).unwrap();
    
    assert!(!results.is_empty());
}

#[test]
fn test_storage_search_with_tags() {
    let mut items = vec![
        ClipboardItem::new("work document".to_string()),
        ClipboardItem::new("personal note".to_string()),
        ClipboardItem::new("project idea".to_string()),
    ];
    
    items[0].add_tag("work".to_string());
    items[1].add_tag("personal".to_string());
    items[2].add_tag("project".to_string());
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    // Search for content
    let query = SearchQuery::new("document".to_string());
    let results = engine.search(&query).unwrap();
    
    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.item.content.contains("document")));
}

#[test]
fn test_storage_search_with_favorites() {
    let mut items = vec![
        ClipboardItem::new("important item".to_string()),
        ClipboardItem::new("regular item".to_string()),
        ClipboardItem::new("another regular item".to_string()),
    ];
    
    items[0].set_favorite(true);
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    // Search with favorite filter
    let mut query = SearchQuery::new("item".to_string());
    query.favorite_only = true;
    let results = engine.search(&query).unwrap();
    
    // Should only return favorite items
    assert!(results.iter().all(|r| r.item.is_favorite));
}

#[test]
fn test_storage_search_update_and_reindex() {
    let mut item = ClipboardItem::new("original content".to_string());
    
    let mut engine = SearchEngine::new();
    engine.index_item(item.clone());
    
    // Update item
    item.content = "updated content".to_string();
    
    // Re-index
    engine.index_item(item.clone());
    
    // Search for updated content
    let query = SearchQuery::new("updated".to_string());
    let results = engine.search(&query).unwrap();
    
    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.item.content.contains("updated")));
}

#[test]
fn test_storage_search_delete_from_index() {
    let items = vec![
        ClipboardItem::new("item to keep".to_string()),
        ClipboardItem::new("item to delete".to_string()),
    ];
    
    let mut engine = SearchEngine::new();
    
    let id_to_delete = items[1].id;
    
    for item in items {
        engine.index_item(item);
    }
    
    // Remove item from index
    engine.remove_item(id_to_delete);
    
    // Verify it's gone
    let query = SearchQuery::new("delete".to_string());
    let results = engine.search(&query).unwrap();
    
    assert!(results.iter().all(|r| r.item.id != id_to_delete));
}

#[test]
fn test_storage_search_unicode_content() {
    let items = vec![
        ClipboardItem::new("Hello 世界".to_string()),
        ClipboardItem::new("Привет мир".to_string()),
        ClipboardItem::new("مرحبا بالعالم".to_string()),
    ];
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    // Search for unicode content
    let query = SearchQuery::new("世界".to_string());
    let results = engine.search(&query).unwrap();
    
    assert!(!results.is_empty());
}

#[test]
fn test_storage_search_empty_results() {
    let items = vec![
        ClipboardItem::new("content 1".to_string()),
        ClipboardItem::new("content 2".to_string()),
    ];
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    // Search for non-existent content
    let query = SearchQuery::new("nonexistent_xyz123".to_string());
    let results = engine.search(&query).unwrap();
    
    assert!(results.is_empty());
}

#[test]
fn test_storage_search_limit_results() {
    let items: Vec<ClipboardItem> = (0..20)
        .map(|i| ClipboardItem::new(format!("content {}", i)))
        .collect();
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    // Search with limit
    let mut query = SearchQuery::new("content".to_string());
    query.limit = Some(5);
    let results = engine.search(&query).unwrap();
    
    assert!(results.len() <= 5);
}
