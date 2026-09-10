# Start-WprBenchmark.ps1
# Windows Performance Recorder automation for clipboard manager benchmarking
# This script captures detailed system performance metrics during benchmark execution

param(
    [Parameter(Mandatory=$true)]
    [string]$BenchmarkName,
    
    [Parameter(Mandatory=$false)]
    [string]$OutputDir = ".\benchmark-results",
    
    [Parameter(Mandatory=$false)]
    [string]$TargetProcess = "morsel",
    
    [Parameter(Mandatory=$false)]
    [int]$DurationSeconds = 300
)

# Ensure output directory exists
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$wprOutputPath = Join-Path $OutputDir "${BenchmarkName}_${timestamp}.etl"

Write-Host "Starting WPR benchmark capture..." -ForegroundColor Green
Write-Host "Benchmark: $BenchmarkName" -ForegroundColor Cyan
Write-Host "Output: $wprOutputPath" -ForegroundColor Cyan
Write-Host "Duration: $DurationSeconds seconds" -ForegroundColor Cyan

# Check if WPR is available
if (-not (Get-Command wpr -ErrorAction SilentlyContinue)) {
    Write-Error "Windows Performance Recorder (WPR) not found. Please install Windows Performance Toolkit."
    exit 1
}

# Create custom WPR profile for clipboard manager benchmarking
$wprProfile = @"
<?xml version="1.0" encoding="utf-8"?>
<Profiles>
  <Profile Name="ClipboardManagerBenchmark" Id="{GUID}" Description="Custom profile for clipboard manager benchmarking" Tag="Custom" LoggingMode="File" DetailLevel="Verbose">
    <Collectors>
      <SystemCollector Id="SystemCollector" Name="NT Kernel Logger">
        <Keyword Value="ProcessThread"/>
        <Keyword Value="Loader"/>
        <Keyword Value="CSwitch"/>
        <Keyword Value="Dispatcher"/>
        <Keyword Value="DiskIO"/>
        <Keyword Value="FileIO"/>
        <Keyword Value="Memory"/>
        <Keyword Value="Network"/>
      </SystemCollector>
      <EventCollector Id="EventCollector" Name="Windows Event Log">
        <Keyword Value="Microsoft-Windows-Kernel-Process"/>
        <Keyword Value="Microsoft-Windows-Kernel-Memory"/>
        <Keyword Value="Microsoft-Windows-Kernel-File"/>
        <Keyword Value="Microsoft-Windows-Kernel-Network"/>
      </EventCollector>
    </Collectors>
  </Profile>
</Profiles>
"@

# Save custom profile
$profilePath = Join-Path $OutputDir "custom_wpr_profile.wprp"
$wprProfile | Out-File -FilePath $profilePath -Encoding UTF8

try {
    # Start WPR capture with custom profile
    Write-Host "Starting WPR capture..." -ForegroundColor Yellow
    wpr -start $profilePath -filemode
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to start WPR capture. Exit code: $LASTEXITCODE"
        exit 1
    }
    
    Write-Host "WPR capture started successfully." -ForegroundColor Green
    Write-Host "Running benchmark for $DurationSeconds seconds..." -ForegroundColor Yellow
    
    # Monitor target process during benchmark
    $processMetrics = @()
    $startTime = Get-Date
    
    while ((Get-Date) - $startTime -lt [TimeSpan]::FromSeconds($DurationSeconds)) {
        $process = Get-Process -Name $TargetProcess -ErrorAction SilentlyContinue
        if ($process) {
            $metric = [PSCustomObject]@{
                Timestamp = Get-Date
                ProcessName = $process.ProcessName
                CPU = $process.CPU
                WorkingSet = $process.WorkingSet64
                PrivateMemory = $process.PrivateMemorySize64
                ThreadCount = $process.Threads.Count
                HandleCount = $process.HandleCount
            }
            $processMetrics += $metric
        }
        
        Start-Sleep -Seconds 1
    }
    
    # Save process metrics
    $metricsPath = Join-Path $OutputDir "${BenchmarkName}_${timestamp}_process_metrics.csv"
    $processMetrics | Export-Csv -Path $metricsPath -NoTypeInformation
    Write-Host "Process metrics saved to: $metricsPath" -ForegroundColor Green
    
    # Stop WPR capture
    Write-Host "Stopping WPR capture..." -ForegroundColor Yellow
    wpr -stop $wprOutputPath
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to stop WPR capture. Exit code: $LASTEXITCODE"
        exit 1
    }
    
    Write-Host "WPR capture saved to: $wprOutputPath" -ForegroundColor Green
    Write-Host "Benchmark capture completed successfully!" -ForegroundColor Green
    
    # Optionally convert to readable format
    Write-Host "Converting ETL to readable format..." -ForegroundColor Yellow
    $csvOutputPath = Join-Path $OutputDir "${BenchmarkName}_${timestamp}_cpu.csv"
    wpa -export $wprOutputPath -csvout $csvOutputPath
    
    Write-Host "CSV export saved to: $csvOutputPath" -ForegroundColor Green
    
} catch {
    Write-Error "Error during WPR benchmark: $_"
    
    # Ensure WPR is stopped on error
    wpr -cancel 2>$null
    
    exit 1
} finally {
    # Cleanup temporary profile
    if (Test-Path $profilePath) {
        Remove-Item $profilePath -Force
    }
}

Write-Host "Use Windows Performance Analyzer (WPA) to open: $wprOutputPath" -ForegroundColor Cyan