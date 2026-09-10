# Generate-Charts.ps1
# Creates visualizations from benchmark CSV data

param(
    [Parameter(Mandatory=$false)]
    [string]$CsvPath = ".\benchmark-results\system_performance_*.csv",
    
    [Parameter(Mandatory=$false)]
    [string]$OutputDir = ".\benchmark-results\charts"
)

# Ensure output directory exists
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

Write-Host "Generating charts from CSV data..." -ForegroundColor Green

# Get the latest CSV file
$csvFiles = Get-ChildItem -Path $CsvPath | Sort-Object LastWriteTime -Descending
if ($csvFiles.Count -eq 0) {
    Write-Error "No CSV files found matching pattern: $CsvPath"
    exit 1
}

$latestCsv = $csvFiles[0]
Write-Host "Processing: $($latestCsv.Name)" -ForegroundColor Cyan

# Import the CSV data
$data = Import-Csv -Path $latestCsv.FullName

if ($data.Count -eq 0) {
    Write-Error "CSV file is empty"
    exit 1
}

Write-Host "Found $($data.Count) data points" -ForegroundColor Cyan

# Get the first row for demonstration
$row = $data[0]

# Create simple text-based charts since we don't have charting libraries
# Generate a CPU performance chart
$cpuChart = @"
CPU Performance Over Time
==========================
Timestamp: $($row.Timestamp)
CPU Usage: $($row.CPU_Total_Percent)%
  User: $($row.CPU_User_Percent)%
  System: $($row.CPU_Privileged_Percent)%
Processor Queue Length: $($row.ProcessorQueueLength)
"@

# Generate memory chart
$memoryChart = @"
Memory Performance
===================
Available Memory: $($row.Memory_Available_MB) MB
Committed Memory: $([math]::Round([double]$row.Memory_Committed_Bytes / 1GB, 2)) GB
Pages/sec: $($row.Memory_Pages_PerSec)
Page Faults/sec: $($row.Memory_PageFaults_PerSec)
"@

# Generate disk chart
$diskChart = @"
Disk Performance
=================
Disk Time: $($row.Disk_Time_Percent)%
Read Latency: $($row.Disk_Read_Latency_ms) ms
Write Latency: $($row.Disk_Write_Latency_ms) ms
Read IOPS: $($row.Disk_Reads_PerSec)
Write IOPS: $($row.Disk_Writes_PerSec)
"@

# Generate network chart
$networkChart = @"
Network Performance
====================
Bytes/sec: $($row.Network_Bytes_PerSec)
Packets/sec: $($row.Network_Packets_PerSec)
"@

# Save charts to files
$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$cpuChart | Out-File -FilePath "$OutputDir\cpu_performance_$timestamp.txt" -Encoding UTF8
$memoryChart | Out-File -FilePath "$OutputDir\memory_performance_$timestamp.txt" -Encoding UTF8
$diskChart | Out-File -FilePath "$OutputDir\disk_performance_$timestamp.txt" -Encoding UTF8
$networkChart | Out-File -FilePath "$OutputDir\network_performance_$timestamp.txt" -Encoding UTF8

Write-Host "Charts generated in: $OutputDir" -ForegroundColor Green
Write-Host "Files created:" -ForegroundColor Cyan
Get-ChildItem -Path $OutputDir -Filter "*_$timestamp.txt" | ForEach-Object {
    Write-Host "  - $($_.Name)"
}

# Also create a simple ASCII bar chart for CPU usage
$cpuPercent = [double]$row.CPU_Total_Percent
$barLength = [math]::Floor($cpuPercent / 2)
$bar = "█" * $barLength

$cpuBarChart = @"
CPU Usage Visualization
=======================
$($row.CPU_Total_Percent)% CPU Usage
$bar
"@

$cpuBarChart | Out-File -FilePath "$OutputDir\cpu_bar_chart_$timestamp.txt" -Encoding UTF8

Write-Host "ASCII bar chart created" -ForegroundColor Green

# Return the output directory
$OutputDir