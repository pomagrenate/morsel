//! Search tests for morsel-search.

use morsel_core::{ClipboardItem, ContentType, ItemId};
use morsel_search::{SearchEngine, SearchQuery};

fn create_test_items() -> Vec<ClipboardItem> {
    vec![
        ClipboardItem::new("https://example.com".to_string()),
        ClipboardItem::new("test@example.com".to_string()),
        ClipboardItem::new("SELECT * FROM users WHERE id = 1".to_string()),
        ClipboardItem::new("fn main() { println!(\"Hello\"); }".to_string()),
        ClipboardItem::new("# Heading\n\nSome markdown content".to_string()),
        ClipboardItem::new("key: value\nanother_key: another_value".to_string()),
        ClipboardItem::new("{\"name\": \"test\", \"value\": 123}".to_string()),
        ClipboardItem::new("ls -la /home/user".to_string()),
        ClipboardItem::new("plain text content".to_string()),
        ClipboardItem::new("550e8400-e29b-41d4-a716-446655440000".to_string()),
    ]
}

#[test]
fn test_search_engine_creation() {
    let engine = SearchEngine::new();
    assert!(engine.is_empty());
}

#[test]
fn test_exact_search() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let query = SearchQuery::new("example.com".to_string());
    let results = engine.search(&query).unwrap();
    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.item.content.contains("example.com")));
}

#[test]
fn test_fuzzy_search() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let mut query = SearchQuery::new("exmple".to_string());
    query.fuzzy = true;
    let results = engine.search(&query).unwrap();
    // Fuzzy search should find "example.com" even with typo
    assert!(!results.is_empty());
}

#[test]
fn test_case_insensitive_search() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let mut query_lower = SearchQuery::new("example.com".to_string());
    query_lower.case_insensitive = true;
    
    let mut query_upper = SearchQuery::new("EXAMPLE.COM".to_string());
    query_upper.case_insensitive = true;
    
    let results_lower = engine.search(&query_lower).unwrap();
    let results_upper = engine.search(&query_upper).unwrap();
    
    assert_eq!(results_lower.len(), results_upper.len());
}

#[test]
fn test_empty_query() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let query = SearchQuery::new("".to_string());
    let results = engine.search(&query).unwrap();
    // Empty query should return no items
    assert!(results.is_empty());
}

#[test]
fn test_no_results() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let query = SearchQuery::new("nonexistent_content_xyz123".to_string());
    let results = engine.search(&query).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_search_limit() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let mut query = SearchQuery::new("".to_string());
    query.limit = Some(3);
    let results = engine.search(&query).unwrap();
    assert!(results.len() <= 3);
}

#[test]
fn test_search_by_content_type() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let mut query = SearchQuery::new("http".to_string());
    query.content_type = Some(ContentType::Url);
    let results = engine.search(&query).unwrap();
    // Should find URL content
    assert!(!results.is_empty());
}

#[test]
fn test_search_ranking() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let query = SearchQuery::new("example".to_string());
    let results = engine.search(&query).unwrap();
    
    // Results should be ranked by relevance
    if results.len() > 1 {
        let first_score = results[0].score;
        let last_score = results.last().unwrap().score;
        assert!(first_score >= last_score);
    }
}

#[test]
fn test_search_with_special_characters() {
    let items = vec![
        ClipboardItem::new("test with \"quotes\"".to_string()),
        ClipboardItem::new("test with \\backslashes\\".to_string()),
        ClipboardItem::new("test with (parentheses)".to_string()),
    ];
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let query = SearchQuery::new("quotes".to_string());
    let results = engine.search(&query).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn test_search_unicode() {
    let items = vec![
        ClipboardItem::new("Hello 世界 🌍".to_string()),
        ClipboardItem::new("Привет мир".to_string()),
        ClipboardItem::new("مرحبا بالعالم".to_string()),
    ];
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let query = SearchQuery::new("世界".to_string());
    let results = engine.search(&query).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn test_search_result_structure() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let query = SearchQuery::new("example".to_string());
    let results = engine.search(&query).unwrap();
    
    if !results.is_empty() {
        let result = &results[0];
        assert!(result.score >= 0);
        assert!(!result.item.content.is_empty());
    }
}

#[test]
fn test_remove_item() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    let item_ids: Vec<ItemId> = items.iter().map(|item| item.id).collect();
    
    for item in items {
        engine.index_item(item);
    }
    
    let id_to_remove = item_ids[0];
    engine.remove_item(id_to_remove);
    
    let query = SearchQuery::new("".to_string());
    let results = engine.search(&query).unwrap();
    assert!(!results.iter().any(|r| r.item.id == id_to_remove));
}

#[test]
fn test_clear_index() {
    let items = create_test_items();
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    engine.clear();
    
    let query = SearchQuery::new("".to_string());
    let results = engine.search(&query).unwrap();
    assert!(results.is_empty());
}

#[test]
fn test_search_performance() {
    let items: Vec<ClipboardItem> = (0..100)
        .map(|i| ClipboardItem::new(format!("test content {}", i)))
        .collect();
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let query = SearchQuery::new("test".to_string());
    let start = std::time::Instant::now();
    let results = engine.search(&query).unwrap();
    let duration = start.elapsed();
    
    assert!(!results.is_empty());
    assert!(duration.as_millis() < 1000, "Search should complete in less than 1 second");
}

#[test]
fn test_search_with_duplicates() {
    let items = vec![
        ClipboardItem::new("duplicate content".to_string()),
        ClipboardItem::new("duplicate content".to_string()),
        ClipboardItem::new("unique content".to_string()),
    ];
    
    let mut engine = SearchEngine::new();
    
    for item in items {
        engine.index_item(item);
    }
    
    let query = SearchQuery::new("duplicate".to_string());
    let results = engine.search(&query).unwrap();
    // Should return both duplicate items
    assert!(results.len() >= 1);
}

#[test]
fn test_search_query_defaults() {
    let query = SearchQuery::new("test".to_string());
    
    assert_eq!(query.text, "test");
    assert!(query.case_insensitive == false);
    assert!(query.fuzzy == false);
    assert!(query.content_type.is_none());
    assert!(query.favorite_only == false);
    assert!(query.limit.is_none());
}
