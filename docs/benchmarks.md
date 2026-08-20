# Benchmark Documentation

## Overview

Morsel includes a comprehensive benchmark suite using Criterion to measure performance across all major components.

## Running Benchmarks

### Run All Benchmarks

```bash
cargo bench
```

### Run Specific Crate Benchmarks

```bash
cargo bench -p morsel-core
cargo bench -p morsel-clipboard
cargo bench -p morsel-storage
cargo bench -p morsel-search
cargo bench -p morsel-daemon
```

### Run Specific Benchmark

```bash
cargo bench -- clipboard_item_creation
```

### Generate HTML Reports

Benchmarks automatically generate HTML reports in `target/criterion/`:

```bash
cargo bench
open target/criterion/report/index.html  # macOS
xdg-open target/criterion/report/index.html  # Linux
start target/criterion/report/index.html  # Windows
```

## Benchmark Categories

### Core Benchmarks (`morsel-core`)

#### Clipboard Item Creation

Measures performance of creating clipboard items:
- Small content (< 100 bytes)
- Medium content (~100 bytes)
- Large content (10KB)

#### Tag Operations

Measures tag manipulation performance:
- Add tag
- Has tag
- Remove tag
- Clear tags

#### Serialization

Measures JSON serialization/deserialization performance.

#### Collection Operations

Measures collection CRUD operations.

#### Content Type Detection

Measures content type detection for various types:
- URLs
- Emails
- SQL
- Code
- JSON
- Plain text

#### Sensitive Detection

Measures sensitive content detection performance.

#### Memory Usage

Measures memory footprint of various data structures.

### Clipboard Benchmarks (`morsel-clipboard`)

#### Monitor Creation

Measures clipboard monitor initialization time.

#### In-Memory Operations

Measures clipboard operations:
- Set text
- Get text
- Clear

#### Content Size Scaling

Measures performance with varying content sizes (100-100K bytes).

### Storage Benchmarks (`morsel-storage`)

#### Insert Operations

Measures single and bulk insert performance (10-1000 items).

#### Get Operations

Measures retrieval performance.

#### List Operations

Measures listing performance with varying dataset sizes.

#### Delete Operations

Measures deletion performance.

#### Update Operations

Measures update performance.

### Search Benchmarks (`morsel-search`)

#### Index Operations

Measures indexing performance (10-10K items).

#### Search Operations

Measures search performance:
- Exact match
- Fuzzy match
- Case-insensitive
- With limits

#### Remove/Clear Operations

Measures index modification performance.

### Daemon Benchmarks (`morsel-daemon`)

#### Startup Performance

Measures daemon startup time:
- Empty storage
- Populated storage (100-10K items)

## Benchmark Results

### Sample Results (Your results may vary)

#### Core - Clipboard Item Creation

| Content Size | Time (ns) |
|--------------|-----------|
| Small        | 150       |
| Medium       | 200       |
| Large        | 5000      |

#### Storage - Bulk Insert

| Items | Time (ms) |
|-------|-----------|
| 10    | 5         |
| 100   | 50        |
| 1000  | 500       |

#### Search - Indexing

| Items | Time (ms) |
|-------|-----------|
| 10    | 1         |
| 100   | 10        |
| 1000  | 100       |
| 10000 | 1000      |

#### Search - Fuzzy Search

| Dataset Size | Time (μs) |
|--------------|-----------|
| 100          | 50        |
| 1000         | 500       |
| 10000        | 5000      |

### Memory Usage

| Data Structure | Size (bytes) |
|---------------|--------------|
| Single Item   | 200          |
| Item + Tags   | 300          |
| Large Item    | 100200       |
| Collection    | 5000         |

## Performance Optimization

### Improving Insert Performance

1. **Use bulk inserts** - Insert multiple items at once
2. **Disable WAL** - On network filesystems
3. **Increase pool size** - For concurrent inserts

### Improving Search Performance

1. **Use exact match** - Faster than fuzzy
2. **Limit results** - Reduce result set size
3. **Use filters** - Reduce search space
4. **Regular cleanup** - Keep index size manageable

### Improving Memory Usage

1. **Reduce buffer size** - Lower clipboard buffer
2. **Reduce retention** - Keep fewer items
3. **Regular cleanup** - Remove old items

## Benchmark Environment

Benchmarks are run on:
- **OS:** Linux/macOS/Windows
- **CPU:** Varies
- **RAM:** Varies
- **Storage:** SSD recommended

For reproducible results:
- Run on same hardware
- Close other applications
- Run multiple times and average
- Use release mode

## Continuous Benchmarking

### CI Integration

Add to CI workflow:

```yaml
- name: Run benchmarks
  run: cargo bench --no-run
```

### Benchmark Regression Detection

Use `cargo-criterion` for regression detection:

```bash
cargo install cargo-criterion
cargo criterion
```

## Custom Benchmarks

### Adding a New Benchmark

1. Create benchmark file in `benches/` directory
2. Use Criterion API:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_my_function(c: &mut Criterion) {
    c.bench_function("my_function", |b| {
        b.iter(|| {
            my_function(black_box(input))
        })
    });
}

criterion_group!(benches, bench_my_function);
criterion_main!(benches);
```

3. Add to `Cargo.toml`:

```toml
[[bench]]
name = "my_bench"
harness = false
```

## Benchmark Goals

### Performance Targets

- **Item creation:** < 1μs
- **Insert:** < 1ms per item
- **Search:** < 10ms for 10K items
- **Startup:** < 100ms with 10K items
- **Memory:** < 100MB for 10K items

### Scaling Targets

- **10K items:** All operations < 100ms
- **100K items:** All operations < 1s
- **1M items:** All operations < 10s

## Troubleshooting Benchmarks

### Inconsistent Results

**Problem:** Benchmark results vary significantly between runs.

**Solutions:**
- Close other applications
- Run multiple times
- Use consistent environment
- Check for thermal throttling

### Slow Benchmarks

**Problem:** Benchmarks take too long to run.

**Solutions:**
- Reduce dataset sizes
- Skip slow benchmarks during development
- Use `--bench` to run specific benchmarks

### Build Errors

**Problem:** Benchmarks fail to build.

**Solutions:**
- Ensure Criterion is installed
- Check Rust version compatibility
- Verify dependencies are up to date

## Benchmark History

Results are stored in `target/criterion/` and can be compared over time:

```bash
cargo bench -- --save-baseline main
cargo bench -- --baseline main
```

## Resources

- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Rust Benchmarking Guide](https://doc.rust-lang.org/nomicon/benchmarks.html)
