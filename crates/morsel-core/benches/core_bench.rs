//! Core benchmarks for morsel-core.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use morsel_core::{ClipboardItem, Collection, ItemId};

fn bench_clipboard_item_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("clipboard_item_creation");
    
    group.bench_function("small_content", |b| {
        b.iter(|| {
            ClipboardItem::new(black_box("test content".to_string()))
        })
    });
    
    group.bench_function("medium_content", |b| {
        b.iter(|| {
            ClipboardItem::new(black_box("This is a medium length piece of content that might be typical for clipboard items in a real application.".to_string()))
        })
    });
    
    group.bench_function("large_content", |b| {
        let large_content = "x".repeat(10_000);
        b.iter(|| {
            ClipboardItem::new(black_box(large_content.clone()))
        })
    });
    
    group.finish();
}

fn bench_clipboard_item_tags(c: &mut Criterion) {
    let mut group = c.benchmark_group("clipboard_item_tags");
    
    group.bench_function("add_tag", |b| {
        let mut item = ClipboardItem::new("test content".to_string());
        b.iter(|| {
            item.add_tag(black_box("tag".to_string()));
        })
    });
    
    group.bench_function("has_tag", |b| {
        let mut item = ClipboardItem::new("test content".to_string());
        item.add_tag("existing_tag".to_string());
        b.iter(|| {
            item.has_tag(black_box("existing_tag"))
        })
    });
    
    group.bench_function("remove_tag", |b| {
        let mut item = ClipboardItem::new("test content".to_string());
        item.add_tag("tag_to_remove".to_string());
        b.iter(|| {
            item.remove_tag(black_box("tag_to_remove"));
        })
    });
    
    group.bench_function("clear_tags", |b| {
        let mut item = ClipboardItem::new("test content".to_string());
        for i in 0..10 {
            item.add_tag(format!("tag_{}", i));
        }
        b.iter(|| {
            item.clear_tags();
        })
    });
    
    group.finish();
}

fn bench_clipboard_item_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");
    
    let item = ClipboardItem::new("test content for serialization".to_string());
    
    group.bench_function("serialize", |b| {
        b.iter(|| {
            serde_json::to_string(black_box(&item))
        })
    });
    
    let serialized = serde_json::to_string(&item).unwrap();
    
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            serde_json::from_str::<ClipboardItem>(black_box(&serialized))
        })
    });
    
    group.finish();
}

fn bench_collection_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("collection_operations");
    
    group.bench_function("create_collection", |b| {
        b.iter(|| {
            Collection::new(black_box("Test Collection".to_string()))
        })
    });
    
    group.bench_function("add_item", |b| {
        let mut collection = Collection::new("Test Collection".to_string());
        let item_id = ItemId::new();
        b.iter(|| {
            collection.add_item(black_box(item_id));
        })
    });
    
    group.bench_function("has_item", |b| {
        let mut collection = Collection::new("Test Collection".to_string());
        let item_id = ItemId::new();
        collection.add_item(item_id);
        b.iter(|| {
            collection.has_item(black_box(item_id))
        })
    });
    
    group.bench_function("remove_item", |b| {
        let mut collection = Collection::new("Test Collection".to_string());
        let item_id = ItemId::new();
        collection.add_item(item_id);
        b.iter(|| {
            collection.remove_item(black_box(item_id));
        })
    });
    
    group.finish();
}

fn bench_content_type_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("content_type_detection");
    
    group.bench_function("url", |b| {
        b.iter(|| {
            morsel_core::ContentType::detect(black_box("https://example.com/path"))
        })
    });
    
    group.bench_function("email", |b| {
        b.iter(|| {
            morsel_core::ContentType::detect(black_box("test@example.com"))
        })
    });
    
    group.bench_function("sql", |b| {
        b.iter(|| {
            morsel_core::ContentType::detect(black_box("SELECT * FROM users WHERE id = 1"))
        })
    });
    
    group.bench_function("code", |b| {
        b.iter(|| {
            morsel_core::ContentType::detect(black_box("fn main() { println!(\"Hello\"); }"))
        })
    });
    
    group.bench_function("json", |b| {
        b.iter(|| {
            morsel_core::ContentType::detect(black_box("{\"key\": \"value\"}"))
        })
    });
    
    group.bench_function("plain_text", |b| {
        b.iter(|| {
            morsel_core::ContentType::detect(black_box("Just plain text content"))
        })
    });
    
    group.finish();
}

fn bench_sensitive_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("sensitive_detection");
    
    let config = morsel_core::SensitiveDetectionConfig::default();
    
    group.bench_function("no_sensitive", |b| {
        b.iter(|| {
            morsel_core::detect_sensitive_with_config(black_box("Just regular content"), black_box(&config))
        })
    });
    
    group.bench_function("with_api_key", |b| {
        b.iter(|| {
            morsel_core::detect_sensitive_with_config(black_box("api_key=sk-1234567890"), black_box(&config))
        })
    });
    
    group.bench_function("with_password", |b| {
        b.iter(|| {
            morsel_core::detect_sensitive_with_config(black_box("password=secret123"), black_box(&config))
        })
    });
    
    group.finish();
}

fn bench_item_id_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("item_id");
    
    group.bench_function("new", |b| {
        b.iter(|| {
            ItemId::new()
        })
    });
    
    group.bench_function("to_string", |b| {
        let id = ItemId::new();
        b.iter(|| {
            id.to_string()
        })
    });
    
    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    group.bench_function("single_item", |b| {
        b.iter(|| {
            let item = ClipboardItem::new(black_box("test content".to_string()));
            std::mem::size_of_val(&item)
        })
    });
    
    group.bench_function("item_with_tags", |b| {
        let mut item = ClipboardItem::new("test content".to_string());
        for i in 0..10 {
            item.add_tag(format!("tag_{}", i));
        }
        b.iter(|| {
            std::mem::size_of_val(&item)
        })
    });
    
    group.bench_function("large_item", |b| {
        let large_content = "x".repeat(100_000);
        let item = ClipboardItem::new(large_content);
        b.iter(|| {
            std::mem::size_of_val(&item)
        })
    });
    
    group.bench_function("collection_with_items", |b| {
        let mut collection = Collection::new("Test Collection".to_string());
        for _ in 0..100 {
            collection.add_item(ItemId::new());
        }
        b.iter(|| {
            std::mem::size_of_val(&collection)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_clipboard_item_creation,
    bench_clipboard_item_tags,
    bench_clipboard_item_serialization,
    bench_collection_operations,
    bench_content_type_detection,
    bench_sensitive_detection,
    bench_item_id_generation,
    bench_memory_usage
);
criterion_main!(benches);
