//! Clipboard benchmarks for morsel-clipboard.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use morsel_clipboard::{ClipboardMonitor, ClipboardProvider, InMemoryClipboard, MonitorConfig};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn bench_clipboard_monitor_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("clipboard_monitor_creation");
    
    group.bench_function("default_config", |b| {
        let clipboard = Arc::new(InMemoryClipboard::new());
        b.iter(|| {
            ClipboardMonitor::new(black_box(clipboard.clone()), black_box(MonitorConfig::default()))
        })
    });
    
    group.finish();
}

fn bench_in_memory_clipboard_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("in_memory_clipboard");
    
    group.bench_function("set_text", |b| {
        let rt = Runtime::new().unwrap();
        let clipboard = Arc::new(InMemoryClipboard::new());
        b.iter(|| {
            rt.block_on(async {
                clipboard.set_text(black_box("test content".to_string())).await
            })
        })
    });
    
    group.bench_function("get_text", |b| {
        let rt = Runtime::new().unwrap();
        let clipboard = Arc::new(InMemoryClipboard::new());
        rt.block_on(async {
            clipboard.set_text("test content".to_string()).await.unwrap();
        });
        b.iter(|| {
            rt.block_on(async {
                clipboard.get_text().await
            })
        })
    });
    
    group.bench_function("clear", |b| {
        let rt = Runtime::new().unwrap();
        let clipboard = Arc::new(InMemoryClipboard::new());
        b.iter(|| {
            rt.block_on(async {
                clipboard.clear().await
            })
        })
    });
    
    group.finish();
}

fn bench_clipboard_content_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("clipboard_content_sizes");
    
    for size in [100, 1000, 10000, 100000].iter() {
        let content = "x".repeat(*size);
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            let rt = Runtime::new().unwrap();
            let clipboard = Arc::new(InMemoryClipboard::new());
            b.iter(|| {
                rt.block_on(async {
                    clipboard.set_text(black_box(content.clone())).await
                })
            })
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_clipboard_monitor_creation,
    bench_in_memory_clipboard_operations,
    bench_clipboard_content_sizes
);
criterion_main!(benches);
