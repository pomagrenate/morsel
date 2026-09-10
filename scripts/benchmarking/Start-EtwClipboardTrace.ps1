# Start-EtwClipboardTrace.ps1
# ETW (Event Tracing for Windows) automation for clipboard event monitoring
# Captures clipboard-related system events during benchmarking

param(
    [Parameter(Mandatory=$false)]
    [string]$OutputDir = ".\benchmark-results",
    
    [Parameter(Mandatory=$false)]
    [int]$DurationSeconds = 300,
    
    [Parameter(Mandatory=$false)]
    [switch]$IncludeProcessEvents
)

# Ensure output directory exists
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$etlOutputPath = Join-Path $OutputDir "clipboard_events_${timestamp}.etl"
$csvOutputPath = Join-Path $OutputDir "clipboard_events_${timestamp}.csv"

Write-Host "Starting ETW clipboard event tracing..." -ForegroundColor Green
Write-Host "Output: $etlOutputPath" -ForegroundColor Cyan
Write-Host "Duration: $DurationSeconds seconds" -ForegroundColor Cyan

# Check if logman is available
if (-not (Get-Command logman -ErrorAction SilentlyContinue)) {
    Write-Error "logman command not found. ETW tracing requires Windows Performance Toolkit."
    exit 1
}

try {
    # Create ETW trace session for clipboard events
    $sessionName = "ClipboardBenchmarkTrace"
    
    # Stop any existing session with the same name
    logman stop $sessionName -ets 2>$null | Out-Null
    logman delete $sessionName -ets 2>$null | Out-Null
    
    # Start new trace session
    Write-Host "Creating ETW trace session..." -ForegroundColor Yellow
    
    $providers = @(
        "Microsoft-Windows-Kernel-Process",      # Process creation/termination
        "Microsoft-Windows-Kernel-File",         # File I/O
        "Microsoft-Windows-Kernel-Memory",       # Memory operations
        "Microsoft-Windows-Kernel-Registry",     # Registry operations
        "Microsoft-Windows-Application-Experience" # Application events
    )
    
    if ($IncludeProcessEvents) {
        $providers += "Microsoft-Windows-Win32k"
    }
    
    # Build provider arguments
    $providerArgs = $providers | ForEach-Object { "-p $_ 0xFFFFFFFF 7" }
    
    # Start the trace session
    $logmanArgs = @(
        "create", "trace", $sessionName,
        "-o", $etlOutputPath,
        "-f", "bincirc",    # Circular binary file
        "-max", "1024",     # 1024MB max file size
        "-bs", "64",        # 64MB buffer size
        "-nb", "16",        # 16 buffers
        "-mode", "Global"   # Global trace mode
    ) + $providerArgs
    
    & logman @logmanArgs -ets
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to create ETW trace session. Exit code: $LASTEXITCODE"
        exit 1
    }
    
    Write-Host "ETW trace session started successfully." -ForegroundColor Green
    Write-Host "Tracing clipboard events for $DurationSeconds seconds..." -ForegroundColor Yellow
    
    # Monitor for clipboard events during benchmark
    $startTime = Get-Date
    $eventCount = 0
    
    while ((Get-Date) - $startTime -lt [TimeSpan]::FromSeconds($DurationSeconds)) {
        # Get real-time clipboard events if possible
        try {
            $clipboardEvents = Get-WinEvent -FilterHashtable @{
                LogName = 'Microsoft-Windows-Kernel-Process/Analytic'
                Level = 4  # Informational
            } -MaxEvents 10 -ErrorAction SilentlyContinue
            
            if ($clipboardEvents) {
                $eventCount += $clipboardEvents.Count
                Write-Host "`rClipboard events captured: $eventCount" -NoNewline
            }
        } catch {
            # Event log might not be accessible
        }
        
        Start-Sleep -Seconds 5
    }
    
    Write-Host "`nStopping ETW trace session..." -ForegroundColor Yellow
    
    # Stop the trace session
    logman stop $sessionName -ets
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to stop ETW trace session. Exit code: $LASTEXITCODE"
        exit 1
    }
    
    # Convert ETL to CSV for analysis
    Write-Host "Converting ETW trace to CSV..." -ForegroundColor Yellow
    
    try {
        # Use tracerpt to convert ETL to CSV
        tracerpt $etlOutputPath -o $csvOutputPath -of CSV -y
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "CSV export saved to: $csvOutputPath" -ForegroundColor Green
        } else {
            Write-Warning "Failed to convert ETL to CSV. The ETL file is still available for analysis."
        }
    } catch {
        Write-Warning "tracerpt not available. The ETL file is still available for analysis."
    }
    
    # Cleanup trace session
    logman delete $sessionName -ets 2>$null | Out-Null
    
    Write-Host "ETW clipboard event tracing completed successfully!" -ForegroundColor Green
    Write-Host "ETL file: $etlOutputPath" -ForegroundColor Cyan
    
    # Display event statistics
    Write-Host "`n=== Event Statistics ===" -ForegroundColor Cyan
    Write-Host "Total clipboard events captured: $eventCount"
    
} catch {
    Write-Error "Error during ETW tracing: $_"
    
    # Cleanup on error
    logman stop $sessionName -ets 2>$null | Out-Null
    logman delete $sessionName -ets 2>$null | Out-Null
    
    exit 1
}