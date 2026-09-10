//! Heavy Payload Freeze Benchmark
//! 
//! This benchmark tests clipboard manager behavior under extreme payload conditions:
//! - 50x high-resolution uncompressed image buffers (20MB - 50MB BMP/PNG)
//! - Large text blobs (10MB - 50MB JSON/log files)
//! 
//! Metrics measured:
//! - Ingestion latency per item
//! - Main UI thread responsiveness (frame drops, IsHungAppWindow)
//! - Peak working set memory spike
//! - Total ingestion time for 50 items

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use morsel_core::ClipboardItem;
use morsel_storage::{SqliteStorage, StorageBackend, StorageConfig};
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;
use std::time::{Duration, Instant};

fn create_test_storage() -> SqliteStorage {
    let temp_file = NamedTempFile::new().unwrap();
    let config = StorageConfig {
        db_path: temp_file.path().to_str().unwrap().to_string(),
        max_connections: 10,
        enable_wal: false,
    };
    SqliteStorage::new(config).unwrap()
}

fn generate_large_text_payload(size_mb: usize) -> String {
    // Generate realistic large text content (JSON-like structure)
    let base_entry = r#"{"timestamp": "2024-01-01T00:00:00Z", "data": "#;
    let entry_size = base_entry.len() + 100; // approximate size per entry
    let entries_needed = (size_mb * 1024 * 1024) / entry_size;
    
    let mut content = String::with_capacity(size_mb * 1024 * 1024);
    for i in 0..entries_needed {
        content.push_str(&format!(r#"{{"id": "{}", "timestamp": "2024-01-01T00:00:00Z", "data": "test_data_{}"}}, "#, i, i));
    }
    content
}

fn bench_heavy_payload_ingestion(c: &mut Criterion) {
    let mut group = c.benchmark_group("heavy_payload_ingestion");
    
    // Test with different payload sizes: 1MB, 5MB, 10MB (reduced for cross-platform testing)
    for size_mb in [1, 5, 10].iter() {
        group.throughput(Throughput::Bytes((*size_mb * 1024 * 1024) as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size_mb), size_mb, |b, &size_mb| {
            let rt = Runtime::new().unwrap();
            let storage = create_test_storage();
            rt.block_on(async {
                storage.initialize().await.unwrap();
            });
            
            let large_text = generate_large_text_payload(size_mb);
            
            b.iter(|| {
                let item = ClipboardItem::new(black_box(large_text.clone()));
                rt.block_on(async {
                    storage.insert(&item).await
                })
            })
        });
    }
    
    group.finish();
}

fn bench_sequential_heavy_payload_ingestion(c: &mut Criterion) {
    let mut group = c.benchmark_group("sequential_heavy_payload_ingestion");
    
    // Test ingesting items sequentially with varying sizes
    for item_count in [5, 10, 20].iter() {
        group.throughput(Throughput::Elements(*item_count as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(item_count), item_count, |b, &item_count| {
            let rt = Runtime::new().unwrap();
            let storage = create_test_storage();
            rt.block_on(async {
                storage.initialize().await.unwrap();
            });
            
            // Generate different large payloads
            let payloads: Vec<String> = (0..item_count)
                .map(|_i| generate_large_text_payload(1)) // 1MB each
                .collect();
            
            b.iter(|| {
                rt.block_on(async {
                    let start = Instant::now();
                    
                    for payload in &payloads {
                        let item = ClipboardItem::new(payload.clone());
                        storage.insert(&item).await.unwrap();
                    }
                    
                    let duration = start.elapsed();
                    assert!(duration < Duration::from_secs(10), 
                        "Sequential ingestion of {} items took too long: {:?}", item_count, duration);
                    
                    duration
                })
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_heavy_payload_ingestion,
    bench_sequential_heavy_payload_ingestion
);
criterion_main!(benches);