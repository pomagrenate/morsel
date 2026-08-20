//! Daemon benchmarks for morsel-daemon.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use morsel_core::ClipboardItem;
use morsel_storage::{SqliteStorage, StorageBackend, StorageConfig};
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;

fn create_test_storage() -> SqliteStorage {
    let temp_file = NamedTempFile::new().unwrap();
    let config = StorageConfig {
        db_path: temp_file.path().to_str().unwrap().to_string(),
        enable_wal: false,
        ..Default::default()
    };
    let storage = SqliteStorage::new(config).unwrap();
    storage
}

fn bench_startup_with_empty_storage(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup");
    
    group.bench_function("empty_storage", |b| {
        let rt = Runtime::new().unwrap();
        b.iter(|| {
            let storage = create_test_storage();
            rt.block_on(async {
                storage.initialize().await
            })
        })
    });
    
    group.finish();
}

fn bench_startup_with_populated_storage(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(criterion::BenchmarkId::from_parameter(size), size, |b, &size| {
            let rt = Runtime::new().unwrap();
            
            // Pre-populate storage
            let storage = create_test_storage();
            rt.block_on(async {
                storage.initialize().await.unwrap();
                for i in 0..size {
                    let item = ClipboardItem::new(format!("content {}", i));
                    storage.insert(&item).await.unwrap();
                }
            });
            
            b.iter(|| {
                let storage = create_test_storage();
                rt.block_on(async {
                    storage.initialize().await
                })
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_startup_with_empty_storage,
    bench_startup_with_populated_storage
);
criterion_main!(benches);
