# Morsel Benchmarking Framework - Complete Methodology & Execution Guide

## Overview

This guide provides comprehensive documentation for the publication-grade benchmarking framework designed to systematically evaluate **Morsel (Rust)** against **Windows Native Clipboard History** and **Ditto (C++/MFC/SQLite)**. The framework is engineered to expose architectural strengths and weaknesses through adversarial benchmark scenarios.

**Framework Version:** 1.0  
**Target Platform:** Windows 10/11  
**Required Privileges:** Administrator (for system-level profiling)  

---

## 🎯 Benchmark Objectives

### Primary Goals
1. **Expose Architectural Flaws:** Identify UI thread blocking, unbounded memory consumption, and inefficient search algorithms in competitors
2. **Highlight Rust Strengths:** Demonstrate zero-cost abstractions, lock-free IPC, instant fuzzy retrieval, and zero-idle resource usage
3. **Provide Reproducible Results:** Ensure consistent, statistically significant measurements across test runs
4. **Enable Competitive Analysis:** Generate publication-ready comparative data

### Success Criteria
- **Sub-millisecond search** across 100K+ items
- **< 5MB idle memory footprint**  
- **Zero UI thread blocking** during heavy operations
- **< 50ms cold start** latency
- **Statistical significance** with 95% confidence intervals

---

## 📁 Framework Structure

```
morsel/
├── crates/
│   └── morsel-benchmarks/          # Rust benchmark suites
│       ├── Cargo.toml
│       └── benches/
│           ├── heavy_payload_freeze.rs      # Heavy payload ingestion
│           ├── fuzzy_search_scalability.rs  # Search performance
│           ├── idle_resource_consumption.rs # Resource usage
│           └── cold_start_latency.rs        # Startup performance
├── scripts/
│   └── benchmarking/
│       ├── Initialize-BenchmarkEnvironment.ps1    # Environment setup
│       ├── Restore-BenchmarkEnvironment.ps1        # Environment cleanup
│       ├── Start-WprBenchmark.ps1                  # WPR profiling
│       ├── Get-ProcessMetrics.ps1                 # Process monitoring
│       ├── Start-EtwClipboardTrace.ps1             # ETW tracing
│       ├── Get-SystemPerformanceStats.ps1         # System metrics
│       └── Invoke-ComparativeBenchmark.ps1        # Comparative runner
└── docs/
    └── benchmarking/
        ├── BENCHMARKING_GUIDE.md                   # This document
        └── COMPARATIVE_MATRIX.md                   # Results template
```

---

## 🚀 Quick Start Guide

### Prerequisites

1. **Administrator Privileges:** Required for system-level profiling and environment configuration
2. **Windows Performance Toolkit:** Install Windows Performance Recorder (WPR) and Windows Performance Analyzer (WPA)
3. **Rust Toolchain:** Rust 1.75+ with Criterion benchmarking framework
4. **PowerShell Execution Policy:** Set to allow script execution

```powershell
# Install Windows Performance Toolkit (if not available)
# Download from: https://aka.ms/windtpr

# Set PowerShell execution policy
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Verify Rust installation
rustc --version
cargo --version
```

### Basic Execution

```powershell
# 1. Initialize benchmark environment (requires admin)
.\scripts\benchmarking\Initialize-BenchmarkEnvironment.ps1

# 2. Run all Rust benchmarks
cargo bench --package morsel-benchmarks

# 3. Run comparative benchmarks (against Windows Native and Ditto)
.\scripts\benchmarking\Invoke-ComparativeBenchmark.ps1 -BenchmarkType All -IncludeWpr

# 4. Restore environment after benchmarking
.\scripts\benchmarking\Restore-BenchmarkEnvironment.ps1
```

---

## 🔧 Detailed Benchmark Scenarios

### 1. Heavy Payload Freeze Benchmark

**Objective:** Test clipboard manager behavior under extreme payload conditions and UI thread responsiveness.

**Implementation:** `crates/morsel-benchmarks/benches/heavy_payload_freeze.rs`

**Test Scenarios:**
- **Single Large Payload:** Ingest 10MB, 25MB, 50MB text blobs
- **Sequential Heavy Ingestion:** 50 items × 10MB each (500MB total)
- **UI Thread Blocking Detection:** Simultaneous UI rendering during heavy ingestion
- **Memory Spike Analysis:** Peak working set measurement during operations

**Key Metrics:**
- Ingestion latency per item (ms)
- UI thread blocking percentage
- Peak memory spike (MB)
- `IsHungAppWindow` events
- Frame drops during ingestion

**Execution:**
```powershell
# Run heavy payload benchmarks
cargo bench --package morsel-benchmarks -- heavy_payload

# With WPR profiling
.\scripts\benchmarking\Start-WprBenchmark.ps1 -BenchmarkName "HeavyPayload" -DurationSeconds 300
cargo bench --package morsel-benchmarks -- heavy_payload
```

**Expected Results (Morsel):**
- Ingestion latency: < 50ms per 50MB item
- UI thread blocking: 0%
- Memory spike: < 150MB
- Zero frame drops

---

### 2. 100K Items Fuzzy Search Benchmark

**Objective:** Measure search performance and scalability with large datasets.

**Implementation:** `crates/morsel-benchmarks/benches/fuzzy_search_scalability.rs`

**Test Scenarios:**
- **Indexing Scalability:** 1K, 10K, 50K, 100K items
- **Search Types:** Exact match, prefix match, scattered fuzzy, cold cache
- **Result Rendering:** Time to display top 20 results
- **Memory Footprint:** Memory usage during active search

**Key Metrics:**
- Query response latency (p50, p95, p99)
- Indexing time for dataset
- Rendering time for results
- Memory footprint during search
- Search accuracy and ranking quality

**Execution:**
```powershell
# Run fuzzy search benchmarks
cargo bench --package morsel-benchmarks -- fuzzy_search

# With process monitoring
.\scripts\benchmarking\Get-ProcessMetrics.ps1 -ProcessName "morsel-daemon" -DurationSeconds 300
cargo bench --package morsel-benchmarks -- fuzzy_search
```

**Expected Results (Morsel):**
- Search latency: < 2ms (100K items)
- Indexing time: < 5s (100K items)
- Rendering time: < 1ms (20 results)
- Memory footprint: < 20MB

---

### 3. 24-Hour Battery & Idle Resource Consumption

**Objective:** Measure background overhead under prolonged idle conditions.

**Implementation:** `crates/morsel-benchmarks/benches/idle_resource_consumption.rs`

**Test Scenarios:**
- **Idle Memory Stability:** 10-minute idle with memory variance measurement
- **CPU Wakeups:** Context switches and CPU time during idle
- **Prolonged Stability:** Extended idle periods (up to 24 hours)
- **Power State Impact:** Effect on CPU power state transitions

**Key Metrics:**
- CPU wakeups per second
- Voluntary/involuntary context switches
- Private working set memory
- Baseline committed memory
- Power consumption impact

**Execution:**
```powershell
# Run idle consumption benchmarks
cargo bench --package morsel-benchmarks -- idle_consumption

# With comprehensive system monitoring
.\scripts\benchmarking\Get-SystemPerformanceStats.ps1 -DurationSeconds 3600
.\scripts\benchmarking\Start-EtwClipboardTrace.ps1 -DurationSeconds 3600
```

**Expected Results (Morsel):**
- CPU wakeups: < 0.1 per second
- Memory footprint: < 5MB idle
- Context switches: < 20 per second
- Power impact: Negligible

---

### 4. Cold-Start & UI Invocation Latency

**Objective:** Measure startup performance and UI responsiveness from user action to visible interface.

**Implementation:** `crates/morsel-benchmarks/benches/cold_start_latency.rs`

**Test Scenarios:**
- **Cold-Start Scaling:** Empty DB, 100, 1K, 10K, 50K items
- **Hotkey Registration:** Time to register and respond to hotkeys
- **Window Creation:** Time from window creation to first frame
- **End-to-End Latency:** Total time from user action to UI visible

**Key Metrics:**
- Cold-start latency (by DB size)
- Hotkey registration time
- Hotkey response time
- Window creation time
- First frame presentation time
- `DwmFlush` present time

**Execution:**
```powershell
# Run cold start benchmarks
cargo bench --package morsel-benchmarks -- cold_start

# With high-resolution timing
$env:RUST_LOG="trace"
cargo bench --package morsel-benchmarks -- cold_start
```

**Expected Results (Morsel):**
- Cold-start (empty): < 15ms
- Cold-start (50K items): < 50ms
- Hotkey response: < 10ms
- Total latency: < 30ms

---

## 🛠️ Windows Observability Tools

### Windows Performance Recorder (WPR)

**Purpose:** Capture detailed system performance metrics including thread scheduling, context switches, and CPU utilization.

**Usage:**
```powershell
# Start WPR capture
.\scripts\benchmarking\Start-WprBenchmark.ps1 -BenchmarkName "TestRun" -DurationSeconds 300

# Manual WPR usage
wpr -start GeneralProfile
# Run your benchmarks
wpr -stop output.etl

# Analyze with WPA
wpa output.etl
```

**Key WPR Metrics:**
- Thread scheduling and CPU utilization
- Context switches (voluntary/involuntary)
- Disk I/O patterns
- Memory allocation patterns
- System call frequency

### ETW (Event Tracing for Windows)

**Purpose:** Capture clipboard-specific events and system-level operations.

**Usage:**
```powershell
# Start ETW clipboard tracing
.\scripts\benchmarking\Start-EtwClipboardTrace.ps1 -DurationSeconds 300 -IncludeProcessEvents

# Manual ETW usage
logman create trace ClipboardTrace -o output.etl -p Microsoft-Windows-Kernel-Process 0xFFFFFFFF 7
logman start ClipboardTrace
# Run benchmarks
logman stop ClipboardTrace
```

**Key ETW Events:**
- Clipboard data transfer events
- Process creation/termination
- File I/O operations
- Registry access patterns

### PowerShell Performance Counters

**Purpose:** Real-time process and system metrics collection.

**Usage:**
```powershell
# Monitor specific process
.\scripts\benchmarking\Get-ProcessMetrics.ps1 -ProcessName "morsel-daemon" -DurationSeconds 300

# Monitor system-wide performance
.\scripts\benchmarking\Get-SystemPerformanceStats.ps1 -DurationSeconds 300

# Manual counter usage
Get-Counter "\Process(morsel-daemon)\% Processor Time" -SampleInterval 1 -MaxSamples 60
Get-Counter "\Memory\Available MBytes" -SampleInterval 1 -MaxSamples 60
```

**Key Counters:**
- Process CPU, memory, thread, handle counts
- System-wide CPU, memory, disk, network metrics
- Thermal and power metrics

---

## 🌍 Environment Standardization

### System Configuration

The `Initialize-BenchmarkEnvironment.ps1` script configures the following:

**Power Settings:**
- Ultimate Performance power plan activation
- USB selective suspend disabled
- PCI Express power management disabled
- Display timeout disabled

**CPU Configuration:**
- CPU affinity optimization (reserve 2 cores for system)
- Process priority configuration
- Foreground boost optimization

**Background Services:**
- Windows Search disabled
- Superfetch/SysMain disabled
- Windows Update temporarily disabled
- Telemetry services disabled

**Security Exclusions:**
- Windows Defender exclusions for benchmark directories
- Process exclusions for benchmark executables

**Visual Effects:**
- Set to "Best Performance"
- Disable animations and transitions
- Optimize DWM settings

**Network Optimization:**
- Disable Nagle's algorithm
- TCP ACK frequency optimization

### Execution

```powershell
# Full environment initialization
.\scripts\benchmarking\Initialize-BenchmarkEnvironment.ps1 -Verbose

# Selective initialization (skip certain optimizations)
.\scripts\benchmarking\Initialize-BenchmarkEnvironment.ps1 -SkipDefenderExclusions -SkipVisualEffects

# Restore environment after benchmarking
.\scripts\benchmarking\Restore-BenchmarkEnvironment.ps1
```

---

## 📊 Comparative Benchmark Execution

### Head-to-Head Testing

The `Invoke-ComparativeBenchmark.ps1` script automates testing against all three clipboard managers:

```powershell
# Run all comparative benchmarks
.\scripts\benchmarking\Invoke-ComparativeBenchmark.ps1 -BenchmarkType All -IncludeWpr -IncludeEtw

# Run specific benchmark category
.\scripts\benchmarking\Invoke-ComparativeBenchmark.ps1 -BenchmarkType FuzzySearch -IncludeWpr

# Specify executable paths
.\scripts\benchmarking\Invoke-ComparativeBenchmark.ps1 `
    -MorselPath ".\target\release\morsel-daemon.exe" `
    -DittoPath "C:\Program Files\Ditto\Ditto.exe" `
    -BenchmarkType All
```

### Test Categories

**HeavyPayload:** Tests ingestion of large payloads and UI responsiveness
**FuzzySearch:** Tests search performance with large datasets  
**IdleConsumption:** Tests resource usage during idle conditions
**ColdStart:** Tests startup latency and UI invocation

---

## 📈 Data Analysis & Reporting

### Result Collection

Benchmark results are automatically collected in the `benchmark-results/` directory:

```
benchmark-results/
├── comparative_20240910-143022_results.json    # Comparative results
├── HeavyPayload_20240910-143022.etl            # WPR trace files
├── HeavyPayload_20240910-143022_process_metrics.csv
├── clipboard_events_20240910-143022.etl        # ETW trace files
├── clipboard_events_20240910-143022.csv
├── system_performance_20240910-143022.csv
└── morsel-daemon_metrics_20240910-143022.csv
```

### Analysis Tools

**Windows Performance Analyzer (WPA):**
```powershell
# Open WPR trace file
wpa benchmark-results\HeavyPayload_20240910-143022.etl

# Key WPA graphs to analyze:
# - CPU Usage (Precise)
# - Context Switches
# - Thread Lifecycle
# - Disk I/O
# - Memory Allocation
```

**CSV Analysis:**
```powershell
# Load process metrics in PowerShell
$metrics = Import-Csv benchmark-results\morsel-daemon_metrics_20240910-143022.csv

# Calculate statistics
$avgCpu = ($metrics | Measure-Object -Property CPU_Percent -Average).Average
$maxMem = ($metrics | Measure-Object -Property WorkingSet_MB -Maximum).Maximum

# Visualize with Excel or Python
```

### Result Compilation

Use the `COMPARATIVE_MATRIX.md` template to compile final results:

```powershell
# The template includes sections for:
# - Overall performance leadership
# - Detailed benchmark results
# - Architectural deep dive
# - Scalability analysis
# - Security comparison
# - Recommendation summary
```

---

## 🔬 Statistical Methodology

### Sample Size & Significance

- **Iterations per benchmark:** 10 runs
- **Statistical method:** Median with Interquartile Range (IQR)
- **Outlier removal:** IQR method (1.5 × IQR)
- **Confidence level:** 95%
- **Confidence intervals:** Calculated using bootstrap method

### Performance Targets

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Search latency (100K items) | < 2ms | Criterion `bench_function` |
| Idle memory footprint | < 5MB | PowerShell `Get-Process` |
| Cold-start latency | < 50ms | High-resolution timer |
| UI thread blocking | 0% | `IsHungAppWindow` detection |
| CPU wakeups/sec | < 0.1 | WPR context switch analysis |

### Data Validation

1. **Baseline Verification:** Run benchmarks on known-good system
2. **Environmental Consistency:** Use same power plan, CPU affinity
3. **Thermal Throttling Check:** Monitor CPU frequency during tests
4. **Background Activity:** Ensure minimal background processes
5. **Result Reproducibility:** Verify results across multiple runs

---

## 🚨 Troubleshooting

### Common Issues

**WPR not available:**
```powershell
# Install Windows Performance Toolkit
# Download from: https://aka.ms/windtpr
```

**PowerShell execution policy:**
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

**Administrator privileges required:**
```powershell
# Run PowerShell as Administrator
# Right-click PowerShell -> "Run as Administrator"
```

**High variance in results:**
```powershell
# Ensure environment is properly initialized
.\scripts\benchmarking\Initialize-BenchmarkEnvironment.ps1

# Close unnecessary applications
# Check for thermal throttling
# Verify power plan is active
```

**Benchmarks timing out:**
```powershell
# Reduce dataset size in benchmark code
# Increase timeout in Criterion configuration
# Check for system resource constraints
```

---

## 📝 Best Practices

### Before Benchmarking
1. **Reboot system** to ensure clean state
2. **Close unnecessary applications** (browsers, IDEs, etc.)
3. **Run environment initialization** script
4. **Verify power plan** is set to Ultimate Performance
5. **Check thermal state** of CPU (not throttling)

### During Benchmarking
1. **Minimize system interaction** during test runs
2. **Monitor system resources** to ensure consistency
3. **Run multiple iterations** for statistical significance
4. **Document environmental conditions** (temperature, background processes)
5. **Verify results are reproducible** across runs

### After Benchmarking
1. **Restore environment** to normal state
2. **Archive benchmark results** with timestamp
3. **Analyze WPR traces** for thread scheduling patterns
4. **Compile comparative matrix** with all results
5. **Document any anomalies** or unexpected results

---

## 🎓 Advanced Usage

### Custom Benchmark Development

To add custom benchmarks:

1. **Create new benchmark file:**
```rust
// crates/morsel-benchmarks/benches/custom_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_custom_operation(c: &mut Criterion) {
    c.bench_function("custom_operation", |b| {
        b.iter(|| {
            // Your benchmark code here
            my_operation(black_box(input))
        })
    });
}

criterion_group!(benches, bench_custom_operation);
criterion_main!(benches);
```

2. **Add to Cargo.toml:**
```toml
[[bench]]
name = "custom_benchmark"
harness = false
```

3. **Run custom benchmark:**
```bash
cargo bench --package morsel-benchmarks -- custom_benchmark
```

### Custom PowerShell Monitoring

Create custom monitoring scripts:

```powershell
# Custom monitoring script
$processName = "morsel-daemon"
$outputFile = "custom_metrics.csv"

while ($true) {
    $process = Get-Process -Name $processName -ErrorAction SilentlyContinue
    if ($process) {
        $metric = [PSCustomObject]@{
            Timestamp = Get-Date
            CPU = $process.CPU
            Memory = $process.WorkingSet64
            Threads = $process.Threads.Count
        }
        $metric | Export-Csv -Path $outputFile -Append -NoTypeInformation
    }
    Start-Sleep -Seconds 1
}
```

---

## 📚 Additional Resources

### Internal Documentation
- [Architecture Documentation](../architecture.md) - System architecture details
- [Existing Benchmarks](../benchmarks.md) - Current Rust benchmark suite
- [Development Guide](../development.md) - Development setup and practices

### External Resources
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/) - Rust benchmarking framework
- [Windows Performance Toolkit](https://docs.microsoft.com/en-us/windows-hardware/test/wpt/) - Windows profiling tools
- [ETW Documentation](https://docs.microsoft.com/en-us/windows/win32/etw/event-tracing-portal) - Event Tracing for Windows
- [PowerShell Performance Counters](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.diagnostics/get-counter) - Performance monitoring

---

## 🏆 Success Metrics

The benchmarking framework is considered successful when:

1. **Performance Targets Met:** All benchmarks meet or exceed defined targets
2. **Reproducible Results:** Variance < 5% across multiple runs
3. **Statistical Significance:** 95% confidence intervals achieved
4. **Competitive Advantage:** Clear performance superiority demonstrated
5. **Publication Ready:** Results suitable for technical publication

---

## 📞 Support & Contributing

For issues or improvements to the benchmarking framework:

1. **Report Issues:** Use GitHub issue tracker
2. **Submit Improvements:** Fork repository and submit PR
3. **Documentation Updates:** Improve this guide with findings
4. **Benchmark Extensions:** Add new scenarios as needed

---

**Framework Version:** 1.0  
**Last Updated:** 2024-09-10  
**Maintained By:** Morsel Development Team