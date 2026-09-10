# Get-ProcessMetrics.ps1
# Detailed process metrics collection for clipboard manager benchmarking
# Captures CPU, memory, handle counts, thread activity, and more

param(
    [Parameter(Mandatory=$true)]
    [string]$ProcessName,
    
    [Parameter(Mandatory=$false)]
    [string]$OutputDir = ".\benchmark-results",
    
    [Parameter(Mandatory=$false)]
    [int]$SampleIntervalSeconds = 1,
    
    [Parameter(Mandatory=$false)]
    [int]$DurationSeconds = 60
)

# Ensure output directory exists
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$outputPath = Join-Path $OutputDir "${ProcessName}_metrics_${timestamp}.csv"

Write-Host "Starting process metrics collection..." -ForegroundColor Green
Write-Host "Process: $ProcessName" -ForegroundColor Cyan
Write-Host "Output: $outputPath" -ForegroundColor Cyan
Write-Host "Interval: $SampleIntervalSeconds seconds" -ForegroundColor Cyan
Write-Host "Duration: $DurationSeconds seconds" -ForegroundColor Cyan

# Initialize performance counters
$cpuCounter = "\Processor(_Total)\% Processor Time"
$processCpuCounter = "\Process(${ProcessName})\% Processor Time"
$memoryCounter = "\Process(${ProcessName})\Working Set"
$privateMemoryCounter = "\Process(${ProcessName})\Private Bytes"
$threadCounter = "\Process(${ProcessName})\Thread Count"
$handleCounter = "\Process(${ProcessName})\Handle Count"

$metrics = @()
$startTime = Get-Date
$endTime = $startTime.AddSeconds($DurationSeconds)

try {
    while ((Get-Date) -lt $endTime) {
        $process = Get-Process -Name $ProcessName -ErrorAction SilentlyContinue
        
        if ($process) {
            # Get performance counter values
            $cpuTime = (Get-Counter $processCpuCounter -ErrorAction SilentlyContinue).CounterSamples.CookedValue
            $systemCpu = (Get-Counter $cpuCounter -ErrorAction SilentlyContinue).CounterSamples.CookedValue
            
            # Calculate context switches if available
            $contextSwitches = 0
            try {
                $processInfo = Get-CimInstance Win32_Process -Filter "Name = '$ProcessName.exe'" -ErrorAction SilentlyContinue
                if ($processInfo) {
                    $contextSwitches = $processInfo.ContextSwitches
                }
            } catch {
                # Context switches not available on all systems
            }
            
            $metric = [PSCustomObject]@{
                Timestamp = Get-Date
                ProcessName = $process.ProcessName
                ProcessId = $process.Id
                CPU_Percent = if ($cpuTime) { [math]::Round($cpuTime, 2) } else { 0 }
                System_CPU_Percent = if ($systemCpu) { [math]::Round($systemCpu, 2) } else { 0 }
                WorkingSet_MB = [math]::Round($process.WorkingSet64 / 1MB, 2)
                PrivateMemory_MB = [math]::Round($process.PrivateMemorySize64 / 1MB, 2)
                ThreadCount = $process.Threads.Count
                HandleCount = $process.HandleCount
                ContextSwitches = $contextSwitches
                StartTime = $process.StartTime
                TotalProcessorTime = $process.TotalProcessorTime
                UserProcessorTime = $process.UserProcessorTime
                PrivilegedProcessorTime = $process.PrivilegedProcessorTime
                NonpagedSystemMemory_MB = [math]::Round($process.NonpagedSystemMemorySize64 / 1MB, 2)
                PagedMemory_MB = [math]::Round($process.PagedMemorySize64 / 1MB, 2)
                PeakWorkingSet_MB = [math]::Round($process.PeakWorkingSet64 / 1MB, 2)
                PeakVirtualMemory_MB = [math]::Round($process.PeakVirtualMemorySize64 / 1MB, 2)
            }
            
            $metrics += $metric
            
            # Display real-time metrics
            Write-Host "`r[$($metrics.Count)] CPU: $($metric.CPU_Percent)% | MEM: $($metric.WorkingSet_MB)MB | Threads: $($metric.ThreadCount) | Handles: $($metric.HandleCount)" -NoNewline
        } else {
            Write-Host "`rProcess $ProcessName not found. Waiting..." -NoNewline -ForegroundColor Yellow
        }
        
        Start-Sleep -Seconds $SampleIntervalSeconds
    }
    
    Write-Host "`nMetrics collection completed." -ForegroundColor Green
    
    # Export to CSV
    $metrics | Export-Csv -Path $outputPath -NoTypeInformation
    Write-Host "Metrics saved to: $outputPath" -ForegroundColor Green
    
    # Generate summary statistics
    Write-Host "`n=== Summary Statistics ===" -ForegroundColor Cyan
    
    if ($metrics.Count -gt 0) {
        $avgCpu = ($metrics | Measure-Object -Property CPU_Percent -Average).Average
        $maxCpu = ($metrics | Measure-Object -Property CPU_Percent -Maximum).Maximum
        $avgMem = ($metrics | Measure-Object -Property WorkingSet_MB -Average).Average
        $maxMem = ($metrics | Measure-Object -Property WorkingSet_MB -Maximum).Maximum
        $avgThreads = ($metrics | Measure-Object -Property ThreadCount -Average).Average
        $maxThreads = ($metrics | Measure-Object -Property ThreadCount -Maximum).Maximum
        $avgHandles = ($metrics | Measure-Object -Property HandleCount -Average).Average
        $maxHandles = ($metrics | Measure-Object -Property HandleCount -Maximum).Maximum
        
        Write-Host "CPU Usage: Avg: $([math]::Round($avgCpu, 2))% | Max: $([math]::Round($maxCpu, 2))%"
        Write-Host "Memory: Avg: $([math]::Round($avgMem, 2))MB | Max: $([math]::Round($maxMem, 2))MB"
        Write-Host "Threads: Avg: $([math]::Round($avgThreads, 2)) | Max: $maxThreads"
        Write-Host "Handles: Avg: $([math]::Round($avgHandles, 2)) | Max: $maxHandles"
        
        # Save summary to file
        $summary = @"
Process Metrics Summary - $ProcessName
Generated: $(Get-Date)
Duration: $DurationSeconds seconds
Samples: $($metrics.Count)

CPU Usage:
  Average: $([math]::Round($avgCpu, 2))%
  Maximum: $([math]::Round($maxCpu, 2))%

Memory Usage:
  Average: $([math]::Round($avgMem, 2))MB
  Maximum: $([math]::Round($maxMem, 2))MB

Thread Count:
  Average: $([math]::Round($avgThreads, 2))
  Maximum: $maxThreads

Handle Count:
  Average: $([math]::Round($avgHandles, 2))
  Maximum: $maxHandles
"@
        
        $summaryPath = Join-Path $OutputDir "${ProcessName}_summary_${timestamp}.txt"
        $summary | Out-File -FilePath $summaryPath
        Write-Host "Summary saved to: $summaryPath" -ForegroundColor Green
    }
    
} catch {
    Write-Error "Error during metrics collection: $_"
    exit 1
}

Write-Host "Process metrics collection completed successfully!" -ForegroundColor Green