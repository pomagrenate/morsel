# Get-SystemPerformanceStats.ps1
# Comprehensive system performance statistics collection for benchmarking
# Captures CPU, memory, disk, network, and thermal metrics

param(
    [Parameter(Mandatory=$false)]
    [string]$OutputDir = ".\benchmark-results",
    
    [Parameter(Mandatory=$false)]
    [int]$SampleIntervalSeconds = 5,
    
    [Parameter(Mandatory=$false)]
    [int]$DurationSeconds = 300
)

# Ensure output directory exists
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$outputPath = Join-Path $OutputDir "system_performance_${timestamp}.csv"

Write-Host "Starting system performance statistics collection..." -ForegroundColor Green
Write-Host "Output: $outputPath" -ForegroundColor Cyan
Write-Host "Interval: $SampleIntervalSeconds seconds" -ForegroundColor Cyan
Write-Host "Duration: $DurationSeconds seconds" -ForegroundColor Cyan

# Initialize performance counters
$cpuCounters = @(
    "\Processor(_Total)\% Processor Time",
    "\Processor(_Total)\% User Time",
    "\Processor(_Total)\% Privileged Time",
    "\System\Processor Queue Length"
)

$memoryCounters = @(
    "\Memory\Available MBytes",
    "\Memory\Committed Bytes",
    "\Memory\Pages/sec",
    "\Memory\Page Faults/sec"
)

$diskCounters = @(
    "\PhysicalDisk(_Total)\% Disk Time",
    "\PhysicalDisk(_Total)\Avg. Disk sec/Read",
    "\PhysicalDisk(_Total)\Avg. Disk sec/Write",
    "\PhysicalDisk(_Total)\Disk Reads/sec",
    "\PhysicalDisk(_Total)\Disk Writes/sec"
)

$networkCounters = @(
    "\Network Interface(*)\Bytes Total/sec",
    "\Network Interface(*)\Packets/sec"
)

$thermalCounters = @(
    "\Thermal Zone Information(*)\Temperature"
)

$allMetrics = @()
$startTime = Get-Date
$endTime = $startTime.AddSeconds($DurationSeconds)

try {
    Write-Host "Collecting system metrics..." -ForegroundColor Yellow
    
    while ((Get-Date) -lt $endTime) {
        $timestamp = Get-Date
        
        # Collect CPU metrics
        $cpuData = Get-Counter $cpuCounters -ErrorAction SilentlyContinue
        $cpuMetrics = if ($cpuData) {
            [PSCustomObject]@{
                CPU_Total_Percent = ($cpuData.CounterSamples | Where-Object { $_.Path -like "*% Processor Time*" }).CookedValue
                CPU_User_Percent = ($cpuData.CounterSamples | Where-Object { $_.Path -like "*% User Time*" }).CookedValue
                CPU_Privileged_Percent = ($cpuData.CounterSamples | Where-Object { $_.Path -like "*% Privileged Time*" }).CookedValue
                ProcessorQueueLength = ($cpuData.CounterSamples | Where-Object { $_.Path -like "*Processor Queue Length*" }).CookedValue
            }
        } else {
            [PSCustomObject]@{
                CPU_Total_Percent = 0
                CPU_User_Percent = 0
                CPU_Privileged_Percent = 0
                ProcessorQueueLength = 0
            }
        }
        
        # Collect memory metrics
        $memoryData = Get-Counter $memoryCounters -ErrorAction SilentlyContinue
        $memoryMetrics = if ($memoryData) {
            [PSCustomObject]@{
                Memory_Available_MB = ($memoryData.CounterSamples | Where-Object { $_.Path -like "*Available MBytes*" }).CookedValue
                Memory_Committed_Bytes = ($memoryData.CounterSamples | Where-Object { $_.Path -like "*Committed Bytes*" }).CookedValue
                Memory_Pages_PerSec = ($memoryData.CounterSamples | Where-Object { $_.Path -like "*Pages/sec*" }).CookedValue
                Memory_PageFaults_PerSec = ($memoryData.CounterSamples | Where-Object { $_.Path -like "*Page Faults/sec*" }).CookedValue
            }
        } else {
            [PSCustomObject]@{
                Memory_Available_MB = 0
                Memory_Committed_Bytes = 0
                Memory_Pages_PerSec = 0
                Memory_PageFaults_PerSec = 0
            }
        }
        
        # Collect disk metrics
        $diskData = Get-Counter $diskCounters -ErrorAction SilentlyContinue
        $diskMetrics = if ($diskData) {
            [PSCustomObject]@{
                Disk_Time_Percent = ($diskData.CounterSamples | Where-Object { $_.Path -like "*% Disk Time*" }).CookedValue
                Disk_Read_Latency_ms = ($diskData.CounterSamples | Where-Object { $_.Path -like "*Avg. Disk sec/Read*" }).CookedValue * 1000
                Disk_Write_Latency_ms = ($diskData.CounterSamples | Where-Object { $_.Path -like "*Avg. Disk sec/Write*" }).CookedValue * 1000
                Disk_Reads_PerSec = ($diskData.CounterSamples | Where-Object { $_.Path -like "*Disk Reads/sec*" }).CookedValue
                Disk_Writes_PerSec = ($diskData.CounterSamples | Where-Object { $_.Path -like "*Disk Writes/sec*" }).CookedValue
            }
        } else {
            [PSCustomObject]@{
                Disk_Time_Percent = 0
                Disk_Read_Latency_ms = 0
                Disk_Write_Latency_ms = 0
                Disk_Reads_PerSec = 0
                Disk_Writes_PerSec = 0
            }
        }
        
        # Collect network metrics
        $networkData = Get-Counter $networkCounters -ErrorAction SilentlyContinue
        $networkMetrics = if ($networkData) {
            $totalBytes = ($networkData.CounterSamples | Where-Object { $_.Path -like "*Bytes Total/sec*" } | Measure-Object -Property CookedValue -Sum).Sum
            $totalPackets = ($networkData.CounterSamples | Where-Object { $_.Path -like "*Packets/sec*" } | Measure-Object -Property CookedValue -Sum).Sum
            [PSCustomObject]@{
                Network_Bytes_PerSec = $totalBytes
                Network_Packets_PerSec = $totalPackets
            }
        } else {
            [PSCustomObject]@{
                Network_Bytes_PerSec = 0
                Network_Packets_PerSec = 0
            }
        }
        
        # Collect thermal metrics if available
        $thermalData = Get-Counter $thermalCounters -ErrorAction SilentlyContinue
        $thermalMetrics = if ($thermalData) {
            $avgTemp = ($thermalData.CounterSamples | Where-Object { $_.Path -like "*Temperature*" } | Measure-Object -Property CookedValue -Average).Average
            [PSCustomObject]@{
                Thermal_Temperature_C = if ($avgTemp) { $avgTemp - 273.15 } else { 0 } # Convert Kelvin to Celsius
            }
        } else {
            [PSCustomObject]@{
                Thermal_Temperature_C = 0
            }
        }
        
        # Combine all metrics
        $combinedMetrics = [PSCustomObject]@{
            Timestamp = $timestamp
            CPU_Total_Percent = $cpuMetrics.CPU_Total_Percent
            CPU_User_Percent = $cpuMetrics.CPU_User_Percent
            CPU_Privileged_Percent = $cpuMetrics.CPU_Privileged_Percent
            ProcessorQueueLength = $cpuMetrics.ProcessorQueueLength
            Memory_Available_MB = $memoryMetrics.Memory_Available_MB
            Memory_Committed_Bytes = $memoryMetrics.Memory_Committed_Bytes
            Memory_Pages_PerSec = $memoryMetrics.Memory_Pages_PerSec
            Memory_PageFaults_PerSec = $memoryMetrics.Memory_PageFaults_PerSec
            Disk_Time_Percent = $diskMetrics.Disk_Time_Percent
            Disk_Read_Latency_ms = $diskMetrics.Disk_Read_Latency_ms
            Disk_Write_Latency_ms = $diskMetrics.Disk_Write_Latency_ms
            Disk_Reads_PerSec = $diskMetrics.Disk_Reads_PerSec
            Disk_Writes_PerSec = $diskMetrics.Disk_Writes_PerSec
            Network_Bytes_PerSec = $networkMetrics.Network_Bytes_PerSec
            Network_Packets_PerSec = $networkMetrics.Network_Packets_PerSec
            Thermal_Temperature_C = $thermalMetrics.Thermal_Temperature_C
        }
        
        $allMetrics += $combinedMetrics
        
        # Display real-time metrics
        Write-Host "`r[$($allMetrics.Count)] CPU: $($cpuMetrics.CPU_Total_Percent)% | MEM: $($memoryMetrics.Memory_Available_MB)MB | Disk: $($diskMetrics.Disk_Time_Percent)% | Temp: $($thermalMetrics.Thermal_Temperature_C)°C" -NoNewline
        
        Start-Sleep -Seconds $SampleIntervalSeconds
    }
    
    Write-Host "`nSystem metrics collection completed." -ForegroundColor Green
    
    # Export to CSV
    $allMetrics | Export-Csv -Path $outputPath -NoTypeInformation
    Write-Host "Metrics saved to: $outputPath" -ForegroundColor Green
    
    # Generate summary statistics
    Write-Host "`n=== Summary Statistics ===" -ForegroundColor Cyan
    
    if ($allMetrics.Count -gt 0) {
        $avgCpu = ($allMetrics | Measure-Object -Property CPU_Total_Percent -Average).Average
        $maxCpu = ($allMetrics | Measure-Object -Property CPU_Total_Percent -Maximum).Maximum
        $avgMem = ($allMetrics | Measure-Object -Property Memory_Available_MB -Average).Average
        $minMem = ($allMetrics | Measure-Object -Property Memory_Available_MB -Minimum).Minimum
        $avgDisk = ($allMetrics | Measure-Object -Property Disk_Time_Percent -Average).Average
        $maxDisk = ($allMetrics | Measure-Object -Property Disk_Time_Percent -Maximum).Maximum
        $avgTemp = ($allMetrics | Measure-Object -Property Thermal_Temperature_C -Average).Average
        $maxTemp = ($allMetrics | Measure-Object -Property Thermal_Temperature_C -Maximum).Maximum
        
        Write-Host "CPU: Avg: $([math]::Round($avgCpu, 2))% | Max: $([math]::Round($maxCpu, 2))%"
        Write-Host "Memory: Avg: $([math]::Round($avgMem, 2))MB | Min: $([math]::Round($minMem, 2))MB"
        Write-Host "Disk: Avg: $([math]::Round($avgDisk, 2))% | Max: $([math]::Round($maxDisk, 2))%"
        Write-Host "Temperature: Avg: $([math]::Round($avgTemp, 1))°C | Max: $([math]::Round($maxTemp, 1))°C"
        
        # Save summary to file
        $summary = @"
System Performance Summary
Generated: $(Get-Date)
Duration: $DurationSeconds seconds
Samples: $($allMetrics.Count)

CPU Performance:
  Average: $([math]::Round($avgCpu, 2))%
  Maximum: $([math]::Round($maxCpu, 2))%

Memory Performance:
  Average Available: $([math]::Round($avgMem, 2))MB
  Minimum Available: $([math]::Round($minMem, 2))MB

Disk Performance:
  Average Utilization: $([math]::Round($avgDisk, 2))%
  Maximum Utilization: $([math]::Round($maxDisk, 2))%

Thermal Performance:
  Average Temperature: $([math]::Round($avgTemp, 1))°C
  Maximum Temperature: $([math]::Round($maxTemp, 1))°C
"@
        
        $summaryPath = Join-Path $OutputDir "system_performance_summary_${timestamp}.txt"
        $summary | Out-File -FilePath $summaryPath
        Write-Host "Summary saved to: $summaryPath" -ForegroundColor Green
    }
    
} catch {
    Write-Error "Error during system metrics collection: $_"
    exit 1
}

Write-Host "System performance statistics collection completed successfully!" -ForegroundColor Green