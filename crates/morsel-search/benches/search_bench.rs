//! Search benchmarks for morsel-search.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use morsel_core::ClipboardItem;
use morsel_search::{SearchEngine, SearchQuery};

fn bench_search_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_index");
    
    group.bench_function("single_item", |b| {
        let mut engine = SearchEngine::new();
        let item = ClipboardItem::new("test content".to_string());
        
        b.iter(|| {
            engine.index_item(black_box(item.clone()))
        })
    });
    
    group.finish();
}

fn bench_search_bulk_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_bulk_index");
    
    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let items: Vec<ClipboardItem> = (0..size)
                .map(|i| ClipboardItem::new(format!("content {}", i)))
                .collect();
            
            b.iter(|| {
                let mut engine = SearchEngine::new();
                for item in &items {
                    engine.index_item(item.clone());
                }
            })
        });
    }
    
    group.finish();
}

fn bench_search_exact(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_exact");
    
    // Setup engine with items
    let mut engine = SearchEngine::new();
    for i in 0..1000 {
        engine.index_item(ClipboardItem::new(format!("content {}", i)));
    }
    
    group.bench_function("exact_match", |b| {
        let query = SearchQuery::new("content 500".to_string());
        b.iter(|| {
            engine.search(black_box(&query))
        })
    });
    
    group.bench_function("no_match", |b| {
        let query = SearchQuery::new("nonexistent_xyz".to_string());
        b.iter(|| {
            engine.search(black_box(&query))
        })
    });
    
    group.finish();
}

fn bench_search_fuzzy(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_fuzzy");
    
    // Setup engine with items
    let mut engine = SearchEngine::new();
    for i in 0..1000 {
        engine.index_item(ClipboardItem::new(format!("content {}", i)));
    }
    
    group.bench_function("fuzzy_match", |b| {
        let mut query = SearchQuery::new("cntnt 500".to_string());
        query.fuzzy = true;
        b.iter(|| {
            engine.search(black_box(&query))
        })
    });
    
    group.bench_function("fuzzy_no_match", |b| {
        let mut query = SearchQuery::new("xyzabc".to_string());
        query.fuzzy = true;
        b.iter(|| {
            engine.search(black_box(&query))
        })
    });
    
    group.finish();
}

fn bench_search_case_insensitive(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_case_insensitive");
    
    // Setup engine with items
    let mut engine = SearchEngine::new();
    for i in 0..1000 {
        engine.index_item(ClipboardItem::new(format!("Content {}", i)));
    }
    
    group.bench_function("case_insensitive", |b| {
        let mut query = SearchQuery::new("content 500".to_string());
        query.case_insensitive = true;
        b.iter(|| {
            engine.search(black_box(&query))
        })
    });
    
    group.finish();
}

fn bench_search_with_limit(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_with_limit");
    
    // Setup engine with items
    let mut engine = SearchEngine::new();
    for i in 0..1000 {
        engine.index_item(ClipboardItem::new(format!("content {}", i)));
    }
    
    for limit in [10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(limit), limit, |b, &limit| {
            let mut query = SearchQuery::new("content".to_string());
            query.limit = Some(limit);
            b.iter(|| {
                engine.search(black_box(&query))
            })
        });
    }
    
    group.finish();
}

fn bench_search_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_remove");
    
    group.bench_function("remove_item", |b| {
        let mut engine = SearchEngine::new();
        let item = ClipboardItem::new("test content".to_string());
        engine.index_item(item.clone());
        
        b.iter(|| {
            engine.remove_item(black_box(item.id))
        })
    });
    
    group.finish();
}

fn bench_search_clear(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_clear");
    
    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut engine = SearchEngine::new();
            for i in 0..size {
                engine.index_item(ClipboardItem::new(format!("content {}", i)));
            }
            
            b.iter(|| {
                engine.clear()
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_search_index,
    bench_search_bulk_index,
    bench_search_exact,
    bench_search_fuzzy,
    bench_search_case_insensitive,
    bench_search_with_limit,
    bench_search_remove,
    bench_search_clear
);
criterion_main!(benches);
