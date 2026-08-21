//! # morsel-search
//!
//! Search functionality for the morsel clipboard manager.
//!
//! This crate provides text normalization, tokenization, full-text indexing,
//! and fuzzy search capabilities for clipboard items.

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use morsel_core::{ClipboardItem, ContentType, CoreError, ItemId};
use std::collections::HashMap;
use thiserror::Error;
use tracing::debug;

/// Result type alias for search operations.
pub type SearchResult<T> = std::result::Result<T, SearchError>;

/// Error types for search operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SearchError {
    #[error("Invalid search query: {0}")]
    InvalidQuery(String),

    #[error("Core error: {0}")]
    CoreError(#[from] CoreError),
}

/// Search query configuration.
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    /// The search text.
    pub text: String,
    /// Whether to perform case-insensitive search.
    pub case_insensitive: bool,
    /// Whether to perform fuzzy matching.
    pub fuzzy: bool,
    /// Filter by content type.
    pub content_type: Option<ContentType>,
    /// Filter by favorite status.
    pub favorite_only: bool,
    /// Maximum number of results to return.
    pub limit: Option<usize>,
}

impl SearchQuery {
    /// Create a new search query.
    pub fn new(text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }

    /// Set case-insensitive flag.
    pub fn case_insensitive(mut self, value: bool) -> Self {
        self.case_insensitive = value;
        self
    }

    /// Set fuzzy matching flag.
    pub fn fuzzy(mut self, value: bool) -> Self {
        self.fuzzy = value;
        self
    }

    /// Set content type filter.
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// Set favorite-only filter.
    pub fn favorite_only(mut self, value: bool) -> Self {
        self.favorite_only = value;
        self
    }

    /// Set result limit.
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Search result with score.
#[derive(Debug, Clone)]
pub struct ScoredItem {
    /// The clipboard item.
    pub item: ClipboardItem,
    /// The search score (higher is better).
    pub score: i64,
}

/// Search engine for clipboard items.
pub struct SearchEngine {
    /// In-memory index of clipboard items.
    index: HashMap<ItemId, ClipboardItem>,
    /// Fuzzy matcher for fuzzy search.
    fuzzy_matcher: SkimMatcherV2,
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchEngine {
    /// Create a new search engine.
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            fuzzy_matcher: SkimMatcherV2::default(),
        }
    }

    /// Add an item to the search index.
    pub fn index_item(&mut self, item: ClipboardItem) {
        debug!("Indexing item: {}", item.id);
        self.index.insert(item.id, item);
    }

    /// Remove an item from the search index.
    pub fn remove_item(&mut self, id: ItemId) -> Option<ClipboardItem> {
        debug!("Removing item from index: {}", id);
        self.index.remove(&id)
    }

    /// Clear the search index.
    pub fn clear(&mut self) {
        debug!("Clearing search index");
        self.index.clear();
    }

    /// Get the number of indexed items.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Check if the index is empty.
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Perform a search.
    pub fn search(&self, query: &SearchQuery) -> SearchResult<Vec<ScoredItem>> {
        if query.text.is_empty() && query.content_type.is_none() && !query.favorite_only {
            return Ok(Vec::new());
        }

        let search_text = if query.case_insensitive {
            query.text.to_lowercase()
        } else {
            query.text.clone()
        };

        let mut results: Vec<ScoredItem> = Vec::new();

        for item in self.index.values() {
            // Apply content type filter
            if let Some(ref filter_type) = query.content_type {
                if &item.content_type != filter_type {
                    continue;
                }
            }

            // Apply favorite filter
            if query.favorite_only && !item.is_favorite {
                continue;
            }

            let item_content = if query.case_insensitive {
                item.content.to_lowercase()
            } else {
                item.content.clone()
            };

            let score = if query.text.is_empty() {
                100
            } else if query.fuzzy {
                // Fuzzy matching
                self.fuzzy_matcher
                    .fuzzy_match(&item_content, &search_text)
                    .unwrap_or(0)
            } else {
                // Exact/partial matching
                if item_content.contains(&search_text) {
                    100 // Base score for exact match
                } else {
                    0
                }
            };

            if score > 0 {
                results.push(ScoredItem {
                    item: item.clone(),
                    score,
                });
            }
        }

        // Sort by score (descending)
        results.sort_by_key(|b| std::cmp::Reverse(b.score));

        // Apply limit
        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        debug!("Search returned {} results", results.len());
        Ok(results)
    }

    /// Perform a simple exact search.
    pub fn search_exact(&self, text: &str) -> SearchResult<Vec<ClipboardItem>> {
        let query = SearchQuery::new(text.to_string());
        let scored = self.search(&query)?;
        Ok(scored.into_iter().map(|s| s.item).collect())
    }

    /// Perform a fuzzy search.
    pub fn search_fuzzy(&self, text: &str) -> SearchResult<Vec<ClipboardItem>> {
        let query = SearchQuery::new(text.to_string()).fuzzy(true);
        let scored = self.search(&query)?;
        Ok(scored.into_iter().map(|s| s.item).collect())
    }

    /// Search by content type.
    pub fn search_by_type(&self, content_type: ContentType) -> SearchResult<Vec<ClipboardItem>> {
        let query = SearchQuery::new(String::new()).content_type(content_type);
        let scored = self.search(&query)?;
        Ok(scored.into_iter().map(|s| s.item).collect())
    }

    /// Search favorites only.
    pub fn search_favorites(&self) -> SearchResult<Vec<ClipboardItem>> {
        let query = SearchQuery::new(String::new()).favorite_only(true);
        let scored = self.search(&query)?;
        Ok(scored.into_iter().map(|s| s.item).collect())
    }
}

/// Text normalizer for search indexing.
pub struct TextNormalizer;

impl TextNormalizer {
    /// Normalize text for search (lowercase, trim whitespace).
    pub fn normalize(text: &str) -> String {
        text.to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Tokenize text into individual words.
    pub fn tokenize(text: &str) -> Vec<String> {
        Self::normalize(text)
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_item(content: &str) -> ClipboardItem {
        ClipboardItem::new(content.to_string())
    }

    #[test]
    fn test_search_query_builder() {
        let query = SearchQuery::new("test".to_string())
            .case_insensitive(true)
            .fuzzy(true)
            .limit(10);

        assert_eq!(query.text, "test");
        assert!(query.case_insensitive);
        assert!(query.fuzzy);
        assert_eq!(query.limit, Some(10));
    }

    #[test]
    fn test_search_engine_index() {
        let mut engine = SearchEngine::new();
        assert!(engine.is_empty());

        let item = create_test_item("test content");
        engine.index_item(item.clone());

        assert_eq!(engine.len(), 1);
        assert!(!engine.is_empty());
    }

    #[test]
    fn test_search_engine_remove() {
        let mut engine = SearchEngine::new();
        let item = create_test_item("test content");
        let id = item.id;

        engine.index_item(item);
        assert_eq!(engine.len(), 1);

        engine.remove_item(id);
        assert!(engine.is_empty());
    }

    #[test]
    fn test_search_engine_clear() {
        let mut engine = SearchEngine::new();
        engine.index_item(create_test_item("content 1"));
        engine.index_item(create_test_item("content 2"));

        assert_eq!(engine.len(), 2);

        engine.clear();
        assert!(engine.is_empty());
    }

    #[test]
    fn test_search_exact() {
        let mut engine = SearchEngine::new();
        engine.index_item(create_test_item("hello world"));
        engine.index_item(create_test_item("hello rust"));
        engine.index_item(create_test_item("goodbye world"));

        let results = engine.search_exact("hello").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_fuzzy() {
        let mut engine = SearchEngine::new();
        engine.index_item(create_test_item("clipboard manager"));
        engine.index_item(create_test_item("clip manager"));
        engine.index_item(create_test_item("board manager"));

        let results = engine.search_fuzzy("clpmgr").unwrap();
        // Should match items with similar patterns
        assert!(!results.is_empty());
    }

    #[test]
    fn test_search_by_type() {
        let mut engine = SearchEngine::new();
        
        let mut text_item = create_test_item("plain text");
        text_item.content_type = ContentType::Text;
        engine.index_item(text_item);

        let mut url_item = create_test_item("https://example.com");
        url_item.content_type = ContentType::Url;
        engine.index_item(url_item);

        let results = engine.search_by_type(ContentType::Url).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content_type, ContentType::Url);
    }

    #[test]
    fn test_search_favorites() {
        let mut engine = SearchEngine::new();
        
        let mut fav_item = create_test_item("favorite content");
        fav_item.set_favorite(true);
        engine.index_item(fav_item);

        let mut normal_item = create_test_item("normal content");
        normal_item.set_favorite(false);
        engine.index_item(normal_item);

        let results = engine.search_favorites().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].is_favorite);
    }

    #[test]
    fn test_search_with_limit() {
        let mut engine = SearchEngine::new();
        for i in 0..10 {
            let item = create_test_item(&format!("content {}", i));
            engine.index_item(item);
        }

        let query = SearchQuery::new("content".to_string()).limit(5);
        let results = engine.search(&query).unwrap();
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_search_with_content_type_filter() {
        let mut engine = SearchEngine::new();
        
        let mut text_item = create_test_item("hello");
        text_item.content_type = ContentType::Text;
        engine.index_item(text_item);

        let mut url_item = create_test_item("hello");
        url_item.content_type = ContentType::Url;
        engine.index_item(url_item);

        let query = SearchQuery::new("hello".to_string()).content_type(ContentType::Text);
        let results = engine.search(&query).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].item.content_type, ContentType::Text);
    }

    #[test]
    fn test_text_normalizer() {
        let normalized = TextNormalizer::normalize("  Hello   World  ");
        assert_eq!(normalized, "hello world");
    }

    #[test]
    fn test_text_tokenizer() {
        let tokens = TextNormalizer::tokenize("Hello World Test");
        assert_eq!(tokens, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_empty_search_query() {
        let engine = SearchEngine::new();
        let query = SearchQuery::new(String::new());
        let results = engine.search(&query).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_scored_results() {
        let mut engine = SearchEngine::new();
        engine.index_item(create_test_item("exact match test"));
        engine.index_item(create_test_item("partial match"));
        engine.index_item(create_test_item("no match here"));

        let query = SearchQuery::new("test".to_string());
        let results = engine.search(&query).unwrap();
        
        // Results should be sorted by score
        assert!(!results.is_empty());
        // First result should have highest score
        if results.len() > 1 {
            assert!(results[0].score >= results[1].score);
        }
    }
}
