//! Storage benchmarks for morsel-storage.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use morsel_core::ClipboardItem;
use morsel_storage::{SqliteStorage, StorageBackend, StorageConfig};
use tempfile::NamedTempFile;
use tokio::runtime::Runtime;

fn create_test_storage() -> SqliteStorage {
    let temp_file = NamedTempFile::new().unwrap();
    let config = StorageConfig {
        db_path: temp_file.path().to_str().unwrap().to_string(),
        max_connections: 10,
        enable_wal: false,
    };
    SqliteStorage::new(config).unwrap()
}

fn bench_storage_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage_insert");
    
    group.bench_function("single_item", |b| {
        let rt = Runtime::new().unwrap();
        let storage = create_test_storage();
        rt.block_on(async {
            storage.initialize().await.unwrap();
        });
        
        b.iter(|| {
            let item = ClipboardItem::new(black_box("test content".to_string()));
            rt.block_on(async {
                storage.insert(&item).await
            })
        })
    });
    
    group.finish();
}

fn bench_storage_bulk_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage_bulk_insert");
    
    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let rt = Runtime::new().unwrap();
            let storage = create_test_storage();
            rt.block_on(async {
                storage.initialize().await.unwrap();
            });
            
            let items: Vec<ClipboardItem> = (0..size)
                .map(|i| ClipboardItem::new(format!("content {}", i)))
                .collect();
            
            b.iter(|| {
                rt.block_on(async {
                    for item in &items {
                        storage.insert(item).await.unwrap();
                    }
                })
            })
        });
    }
    
    group.finish();
}

fn bench_storage_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage_get");
    
    group.bench_function("single_item", |b| {
        let rt = Runtime::new().unwrap();
        let storage = create_test_storage();
        rt.block_on(async {
            storage.initialize().await.unwrap();
            let item = ClipboardItem::new("test content".to_string());
            storage.insert(&item).await.unwrap();
        });
        
        let id = rt.block_on(async {
            storage.list().await.unwrap()[0].id
        });
        
        b.iter(|| {
            rt.block_on(async {
                storage.get(black_box(id)).await
            })
        })
    });
    
    group.finish();
}

fn bench_storage_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage_list");
    
    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let rt = Runtime::new().unwrap();
            let storage = create_test_storage();
            rt.block_on(async {
                storage.initialize().await.unwrap();
                for i in 0..size {
                    let item = ClipboardItem::new(format!("content {}", i));
                    storage.insert(&item).await.unwrap();
                }
            });
            
            b.iter(|| {
                rt.block_on(async {
                    storage.list().await
                })
            })
        });
    }
    
    group.finish();
}

fn bench_storage_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage_delete");
    
    group.bench_function("single_item", |b| {
        let rt = Runtime::new().unwrap();
        let storage = create_test_storage();
        rt.block_on(async {
            storage.initialize().await.unwrap();
            let item = ClipboardItem::new("test content".to_string());
            storage.insert(&item).await.unwrap();
        });
        
        let id = rt.block_on(async {
            storage.list().await.unwrap()[0].id
        });
        
        b.iter(|| {
            rt.block_on(async {
                storage.delete(black_box(id)).await
            })
        })
    });
    
    group.finish();
}

fn bench_storage_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("storage_update");
    
    group.bench_function("single_item", |b| {
        let rt = Runtime::new().unwrap();
        let storage = create_test_storage();
        rt.block_on(async {
            storage.initialize().await.unwrap();
            let item = ClipboardItem::new("test content".to_string());
            storage.insert(&item).await.unwrap();
        });
        
        let mut item = rt.block_on(async {
            storage.list().await.unwrap()[0].clone()
        });
        
        b.iter(|| {
            item.content = black_box("updated content".to_string());
            rt.block_on(async {
                storage.update(&item).await
            })
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_storage_insert,
    bench_storage_bulk_insert,
    bench_storage_get,
    bench_storage_list,
    bench_storage_delete,
    bench_storage_update
);
criterion_main!(benches);
