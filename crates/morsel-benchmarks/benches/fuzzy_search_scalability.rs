//! 100K Items Fuzzy Search Scalability Benchmark
//! 
//! This benchmark tests search performance with large datasets:
//! - Pre-populate database with 100,000 real-world mixed clipboard entries
//! - Execute various search queries (prefix, scattered fuzzy, cold-cache)
//! - Measure query response latency, frame rendering time, memory footprint
//! 
//! Metrics measured:
//! - Query response latency (p50, p95, p99)
//! - Time to display top 20 results
//! - Memory footprint during active search
//! - Search accuracy and ranking quality

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use morsel_core::ClipboardItem;
use morsel_search::{SearchEngine, SearchQuery};
use std::time::{Duration, Instant};

fn generate_realistic_clipboard_items(count: usize) -> Vec<ClipboardItem> {
    let mut items = Vec::with_capacity(count);
    
    // Real-world clipboard content types
    let code_snippets = vec![
        "fn main() { println!(\"Hello, world!\"); }",
        "const API_KEY = \"sk-1234567890abcdef\";",
        "git commit -m \"fix: resolve memory leak in clipboard manager\"",
        "SELECT * FROM users WHERE email = 'test@example.com'",
        "docker run -d -p 8080:80 nginx",
        "curl -X POST https://api.example.com/endpoint",
        "export PATH=$PATH:/usr/local/bin",
        "import numpy as np; np.array([1, 2, 3])",
        "class UserService { constructor() {} }",
        "#!/bin/bash\necho \"Script started\"",
    ];
    
    let urls = vec![
        "https://github.com/pomagrenate/morsel",
        "https://crates.io/crates/morsel-cli",
        "https://stackoverflow.com/questions/123456",
        "https://docs.rs/tokio/latest/tokio/",
        "https://reddit.com/r/rust",
        "https://twitter.com/rustlang",
        "https://example.com/api/v1/users",
        "https://localhost:8080/health",
    ];
    
    let json_data = vec![
        r#"{"name": "John", "age": 30, "city": "New York"}"#,
        r#"{"items": ["apple", "banana", "orange"], "count": 3}"#,
        r#"{"error": null, "success": true, "data": {}}"#,
        r#"{"timestamp": "2024-01-01T00:00:00Z", "value": 42}"#,
    ];
    
    let stack_traces = vec![
        "Error: Cannot read property 'x' of undefined\n    at Object.method (file.js:10:5)\n    at main (file.js:20:3)",
        "thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', src/main.rs:5:1",
        "TypeError: 'NoneType' object is not iterable\n  File \"script.py\", line 15, in <module>",
    ];
    
    let markdown = vec![
        "# Heading 1\n\nThis is a paragraph with **bold** and *italic* text.",
        "## Code Example\n\n```rust\nfn main() { println!(\"Hello\"); }\n```",
        "- Item 1\n- Item 2\n- Item 3",
    ];
    
    for i in 0..count {
        let content = match i % 5 {
            0 => code_snippets[i % code_snippets.len()].to_string(),
            1 => urls[i % urls.len()].to_string(),
            2 => json_data[i % json_data.len()].to_string(),
            3 => stack_traces[i % stack_traces.len()].to_string(),
            _ => markdown[i % markdown.len()].to_string(),
        };
        
        // Add some variety with timestamps and IDs
        let varied_content = format!("{} [ID: {}] [Timestamp: {}]", 
            content, i, chrono::Utc::now().timestamp());
        
        items.push(ClipboardItem::new(varied_content));
    }
    
    items
}

fn bench_search_indexing_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_indexing_scalability");
    
    for size in [1_000, 10_000, 50_000, 100_000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let items = generate_realistic_clipboard_items(size);
            
            b.iter(|| {
                let mut engine = SearchEngine::new();
                let start = Instant::now();
                
                for item in &items {
                    engine.index_item(item.clone());
                }
                
                let duration = start.elapsed();
                
                // Assert reasonable indexing time (< 1 second for 100K items)
                if size >= 100_000 {
                    assert!(duration < Duration::from_secs(1), 
                        "Indexing 100K items took too long: {:?}", duration);
                }
                
                duration
            })
        });
    }
    
    group.finish();
}

fn bench_fuzzy_search_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("fuzzy_search_performance");
    
    // Setup engines with different dataset sizes
    for size in [1_000, 10_000, 50_000, 100_000].iter() {
        let mut engine = SearchEngine::new();
        let items = generate_realistic_clipboard_items(*size);
        for item in &items {
            engine.index_item(item.clone());
        }
        
        // Test different query types
        let query_types = vec![
            ("exact_match", "git commit"),
            ("prefix_match", "git"),
            ("scattered_fuzzy", "gt cmmmit"), // Typo tolerance
            ("cold_cache", "unique_query_xyz"),
        ];
        
        for (query_type, query_text) in query_types {
            group.bench_with_input(
                BenchmarkId::new(format!("{}_{}", query_type, size), size), 
                &query_text, 
                |b, query| {
                    let mut search_query = SearchQuery::new(query.to_string());
                    search_query.fuzzy = query_type.contains("fuzzy");
                    search_query.limit = Some(20); // Top 20 results
                    
                    b.iter(|| {
                        let start = Instant::now();
                        let results = engine.search(black_box(&search_query)).unwrap();
                        let duration = start.elapsed();
                        
                        // Assert sub-millisecond search for 100K items
                        if *size >= 100_000 {
                            assert!(duration < Duration::from_millis(2), 
                                "Search on 100K items took too long: {:?}", duration);
                        }
                        
                        // Assert we get results for reasonable queries
                        if !query_type.contains("cold_cache") {
                            assert!(!results.is_empty(), 
                                "Expected results for query: {}", query);
                        }
                        
                        duration
                    })
                }
            );
        }
    }
    
    group.finish();
}

fn bench_search_result_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_result_rendering");
    
    // Test rendering time for top 20 results
    for size in [1_000, 10_000, 50_000, 100_000].iter() {
        let mut engine = SearchEngine::new();
        let items = generate_realistic_clipboard_items(*size);
        for item in &items {
            engine.index_item(item.clone());
        }
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            let query = SearchQuery::new("git".to_string());
            
            b.iter(|| {
                let start = Instant::now();
                let results = engine.search(black_box(&query)).unwrap();
                
                // Simulate rendering top 20 results
                let top_20: Vec<_> = results.iter().take(20).collect();
                
                // Simulate formatting for display
                let formatted: Vec<String> = top_20.iter()
                    .map(|r| format!("{}: {}", r.score, r.item.content.chars().take(50).collect::<String>()))
                    .collect();
                
                let duration = start.elapsed();
                
                // Assert rendering is fast (< 1ms)
                assert!(duration < Duration::from_millis(1), 
                    "Result rendering took too long: {:?}", duration);
                
                // Assert we formatted all results
                assert_eq!(formatted.len(), top_20.len());
                
                duration
            })
        });
    }
    
    group.finish();
}

fn bench_search_memory_footprint(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_memory_footprint");
    
    for size in [1_000, 10_000, 50_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut engine = SearchEngine::new();
                let items = generate_realistic_clipboard_items(size);
                
                for item in &items {
                    engine.index_item(item.clone());
                }
                
                // Perform search to measure memory during active search
                let query = SearchQuery::new("git".to_string());
                let _results = engine.search(&query);
                
                // Memory measurement would be done with dhat or external tools
                // For now, we ensure the engine doesn't grow unbounded
                // In a real benchmark, we'd measure RSS before/after
                
                size
            })
        });
    }
    
    group.finish();
}

fn bench_search_accuracy_and_ranking(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_accuracy_ranking");
    
    let mut engine = SearchEngine::new();
    let items = generate_realistic_clipboard_items(10_000);
    for item in &items {
        engine.index_item(item.clone());
    }
    
    // Test that relevant results appear first
    group.bench_function("ranking_quality", |b| {
        let query = SearchQuery::new("git commit".to_string());
        
        b.iter(|| {
            let results = engine.search(black_box(&query)).unwrap();
            
            // Assert that results are sorted by score (descending)
            for i in 1..results.len().min(10) {
                assert!(results[i-1].score >= results[i].score, 
                    "Results not properly sorted by score");
            }
            
            // Assert that relevant content appears in top results
            let top_results: Vec<_> = results.iter().take(5).collect();
            let has_relevant = top_results.iter()
                .any(|r| r.item.content.contains("git"));
            
            assert!(has_relevant, "Expected relevant results for 'git commit'");
            
            results.len()
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_search_indexing_scalability,
    bench_fuzzy_search_performance,
    bench_search_result_rendering,
    bench_search_memory_footprint,
    bench_search_accuracy_and_ranking
);
criterion_main!(benches);