# Benchmarking Scripts

This directory contains PowerShell automation scripts for comprehensive clipboard manager benchmarking on Windows.

## Overview

These scripts provide automated, publication-grade benchmarking capabilities for comparing **Morsel (Rust)** against **Windows Native Clipboard History** and **Ditto (C++/MFC/SQLite)**.

## Available Scripts

### Environment Management

- **`Initialize-BenchmarkEnvironment.ps1`** - System environment standardization for reproducible benchmarking
- **`Restore-BenchmarkEnvironment.ps1`** - Restores system settings after benchmarking

### Performance Profiling

- **`Start-WprBenchmark.ps1`** - Windows Performance Recorder automation for detailed system metrics
- **`Start-EtwClipboardTrace.ps1`** - ETW tracing for clipboard-specific events
- **`Get-ProcessMetrics.ps1`** - Detailed process metrics collection (CPU, memory, handles, threads)
- **`Get-SystemPerformanceStats.ps1`** - System-wide performance statistics

### Comparative Testing

- **`Invoke-ComparativeBenchmark.ps1`** - Head-to-head comparative benchmark runner

## Quick Start

```powershell
# 1. Initialize environment (requires admin)
.\Initialize-BenchmarkEnvironment.ps1

# 2. Run comparative benchmarks
.\Invoke-ComparativeBenchmark.ps1 -BenchmarkType All -IncludeWpr

# 3. Restore environment
.\Restore-BenchmarkEnvironment.ps1
```

## Prerequisites

- **Administrator Privileges:** Required for system-level profiling
- **Windows Performance Toolkit:** Install WPR/WPA from [Microsoft](https://aka.ms/windtpr)
- **PowerShell Execution Policy:** Set to allow script execution
- **Rust Toolchain:** For running Rust benchmarks

## Script Details

### Initialize-BenchmarkEnvironment.ps1

Configures the system for reproducible benchmarking:

- Sets Ultimate Performance power plan
- Optimizes CPU affinity
- Disables background services (Windows Search, Superfetch)
- Adds Windows Defender exclusions
- Optimizes visual effects and DWM settings
- Configures network and system priority settings

**Usage:**
```powershell
.\Initialize-BenchmarkEnvironment.ps1 -Verbose
```

### Restore-BenchmarkEnvironment.ps1

Reverses all changes made by the initialization script:

- Restores Balanced power plan
- Re-enables background services
- Removes Defender exclusions
- Restores visual effects
- Resets network and priority settings

**Usage:**
```powershell
.\Restore-BenchmarkEnvironment.ps1
```

### Start-WprBenchmark.ps1

Captures detailed system performance metrics using Windows Performance Recorder:

- Thread scheduling and context switches
- CPU utilization patterns
- Disk I/O operations
- Memory allocation patterns
- System call frequency

**Usage:**
```powershell
.\Start-WprBenchmark.ps1 -BenchmarkName "TestRun" -DurationSeconds 300 -TargetProcess "morsel-daemon"
```

### Get-ProcessMetrics.ps1

Collects detailed process metrics during benchmark execution:

- CPU usage percentage
- Working set and private memory
- Thread and handle counts
- Context switches
- Processor time breakdown

**Usage:**
```powershell
.\Get-ProcessMetrics.ps1 -ProcessName "morsel-daemon" -DurationSeconds 300 -SampleIntervalSeconds 1
```

### Start-EtwClipboardTrace.ps1

Captures clipboard-specific events using ETW:

- Clipboard data transfer events
- Process creation/termination
- File I/O operations
- Registry access patterns

**Usage:**
```powershell
.\Start-EtwClipboardTrace.ps1 -DurationSeconds 300 -IncludeProcessEvents
```

### Get-SystemPerformanceStats.ps1

Collects system-wide performance statistics:

- CPU utilization (total, user, privileged)
- Memory usage (available, committed, page faults)
- Disk performance (utilization, latency, IOPS)
- Network activity
- Thermal metrics

**Usage:**
```powershell
.\Get-SystemPerformanceStats.ps1 -DurationSeconds 300 -SampleIntervalSeconds 5
```

### Invoke-ComparativeBenchmark.ps1

Automates head-to-head testing against multiple clipboard managers:

- Runs identical test scenarios on Morsel, Windows Native, and Ditto
- Supports WPR and ETW profiling
- Generates comparative results in JSON format
- Provides summary statistics

**Usage:**
```powershell
.\Invoke-ComparativeBenchmark.ps1 -BenchmarkType All -IncludeWpr -IncludeEtw
```

**Benchmark Types:**
- `HeavyPayload` - Large payload ingestion and UI responsiveness
- `FuzzySearch` - Search performance with large datasets
- `IdleConsumption` - Resource usage during idle conditions
- `ColdStart` - Startup latency and UI invocation
- `All` - Run all benchmark categories

## Output Directory

All benchmark results are saved to `.\benchmark-results\` with timestamps:

```
benchmark-results/
├── comparative_20240910-143022_results.json
├── HeavyPayload_20240910-143022.etl
├── HeavyPayload_20240910-143022_process_metrics.csv
├── clipboard_events_20240910-143022.etl
├── system_performance_20240910-143022.csv
└── morsel-daemon_metrics_20240910-143022.csv
```

## Analysis Tools

### Windows Performance Analyzer (WPA)

Open `.etl` files with WPA for detailed analysis:

```powershell
wpa benchmark-results\HeavyPayload_20240910-143022.etl
```

Key WPA graphs to analyze:
- CPU Usage (Precise)
- Context Switches
- Thread Lifecycle
- Disk I/O
- Memory Allocation

### CSV Analysis

Import CSV files into PowerShell or Excel for analysis:

```powershell
$metrics = Import-Csv benchmark-results\morsel-daemon_metrics_20240910-143022.csv
$metrics | Measure-Object -Property CPU_Percent -Average
```

## Troubleshooting

### Administrator Privileges

Most scripts require administrator privileges. Run PowerShell as Administrator:

```powershell
# Right-click PowerShell -> "Run as Administrator"
```

### Execution Policy

If scripts won't run, set execution policy:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### WPR Not Available

Install Windows Performance Toolkit from [Microsoft](https://aka.ms/windtpr).

### High Result Variance

Ensure environment is properly initialized and close unnecessary applications:

```powershell
.\Initialize-BenchmarkEnvironment.ps1
# Close browsers, IDEs, etc.
# Then run benchmarks
```

## Best Practices

1. **Always initialize environment** before benchmarking
2. **Run as Administrator** for system-level profiling
3. **Close unnecessary applications** to reduce noise
4. **Use consistent hardware** for reproducible results
5. **Restore environment** after benchmarking
6. **Archive results** with timestamps for comparison
7. **Analyze WPR traces** for detailed insights

## Integration with Rust Benchmarks

These PowerShell scripts work alongside the Rust benchmark suites in `crates/morsel-benchmarks/`:

```powershell
# Initialize environment
.\Initialize-BenchmarkEnvironment.ps1

# Start WPR profiling
.\Start-WprBenchmark.ps1 -BenchmarkName "RustBenchmarks" -DurationSeconds 600

# Run Rust benchmarks
cargo bench --package morsel-benchmarks

# Restore environment
.\Restore-BenchmarkEnvironment.ps1
```

## Documentation

- [Complete Benchmarking Guide](../../docs/benchmarking/BENCHMARKING_GUIDE.md)
- [Comparative Matrix Template](../../docs/benchmarking/COMPARATIVE_MATRIX.md)
- [Architecture Documentation](../../docs/architecture.md)

## Support

For issues or questions about the benchmarking framework, refer to the main project documentation or create an issue in the repository.