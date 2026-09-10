# Initialize-BenchmarkEnvironment.ps1
# System environment standardization for reproducible clipboard manager benchmarking
# Configures power settings, CPU affinity, background services, and Windows settings

param(
    [Parameter(Mandatory=$false)]
    [switch]$SkipPowerSettings,
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipServiceOptimization,
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipDefenderExclusions,
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipVisualEffects,
    
    [Parameter(Mandatory=$false)]
    [switch]$Verbose
)

function Write-Verbose-Output {
    param([string]$Message)
    if ($Verbose) {
        Write-Host $Message -ForegroundColor DarkGray
    }
}

function Test-AdminPrivileges {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Set-UltimatePerformancePowerPlan {
    Write-Host "Configuring Ultimate Performance power plan..." -ForegroundColor Yellow
    
    try {
        # Check if Ultimate Performance plan exists
        $ultimatePlan = powercfg -list | Select-String "Ultimate Performance"
        
        if (-not $ultimatePlan) {
            Write-Host "Ultimate Performance plan not found. Creating..." -ForegroundColor Cyan
            # Create Ultimate Performance plan (requires admin)
            powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61
        }
        
        # Get the GUID of Ultimate Performance plan
        $ultimateGuid = powercfg -list | Select-String "Ultimate Performance" | ForEach-Object {
            if ($_ -match '([a-f0-9-]{36})') {
                $matches[1]
            }
        }
        
        if ($ultimateGuid) {
            Write-Host "Activating Ultimate Performance plan ($ultimateGuid)..." -ForegroundColor Cyan
            powercfg -setactive $ultimateGuid
            
            # Disable USB selective suspend
            powercfg -setacvalueindex $ultimateGuid 2a737441-1930-4402-8d77-b2bebba09a3c 48e6b7a6-50f5-4782-a5d4-53b8c75dfe40 0
            powercfg -setdcvalueindex $ultimateGuid 2a737441-1930-4402-8d77-b2bebba09a3c 48e6b7a6-50f5-4782-a5d4-53b8c75dfe40 0
            
            # Disable PCI Express power management
            powercfg -setacvalueindex $ultimateGuid 501a8d37-2fcd-4642-9da8-2b25ee3b3146 ee12f906-d277-404b-b6da-e211fa7e6332 0
            powercfg -setdcvalueindex $ultimateGuid 501a8d37-2fcd-4642-9da8-2b25ee3b3146 ee12f906-d277-404b-b6da-e211fa7e6332 0
            
            # Disable display timeout
            powercfg -setacvalueindex $ultimateGuid 7516b95f-f776-4464-8c53-06167f40cc99 3c0bc021-c8a8-4e07-a973-6b19cb8a6609 0
            powercfg -setdcvalueindex $ultimateGuid 7516b95f-f776-4464-8c53-06167f40cc99 3c0bc021-c8a8-4e07-a973-6b19cb8a6609 0
            
            # Apply changes
            powercfg -applysettingscheme $ultimateGuid
            
            Write-Host "Ultimate Performance plan configured successfully." -ForegroundColor Green
        } else {
            Write-Warning "Could not find or create Ultimate Performance plan. Using High Performance instead."
            powercfg -setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c
        }
    } catch {
        Write-Warning "Failed to configure power settings: $_"
    }
}

function Set-ProcessAffinity {
    Write-Host "Configuring CPU affinity for benchmarking..." -ForegroundColor Yellow
    
    try {
        # Get number of logical processors
        $processors = Get-WmiObject Win32_Processor
        $coreCount = $processors.NumberOfLogicalProcessors
        
        Write-Host "Detected $coreCount logical processors." -ForegroundColor Cyan
        
        # For benchmarking, we want to reserve 2 cores for system and use the rest for benchmarks
        if ($coreCount -gt 4) {
            $benchmarkCores = $coreCount - 2
            Write-Host "Reserving 2 cores for system, using $benchmarkCores cores for benchmarks." -ForegroundColor Cyan
            
            # Create affinity mask (using all but the last 2 cores)
            $affinityMask = [math]::Pow(2, $benchmarkCores) - 1
            
            Write-Host "CPU affinity mask: $affinityMask" -ForegroundColor Cyan
            Write-Host "To apply affinity manually, use: start /affinity $affinityMask your_program.exe" -ForegroundColor Yellow
        } else {
            Write-Host "System has $coreCount cores or fewer. Using all cores for benchmarks." -ForegroundColor Cyan
        }
    } catch {
        Write-Warning "Failed to configure CPU affinity: $_"
    }
}

function Optimize-BackgroundServices {
    Write-Host "Optimizing background services for benchmarking..." -ForegroundColor Yellow
    
    try {
        # Disable Windows Search
        Write-Host "Disabling Windows Search service..." -ForegroundColor Cyan
        $searchService = Get-Service -Name "WSearch" -ErrorAction SilentlyContinue
        if ($searchService) {
            Set-Service -Name "WSearch" -StartupType Disabled -ErrorAction SilentlyContinue
            Stop-Service -Name "WSearch" -Force -ErrorAction SilentlyContinue
            Write-Host "Windows Search disabled." -ForegroundColor Green
        }
        
        # Disable Superfetch/SysMain
        Write-Host "Disabling Superfetch/SysMain service..." -ForegroundColor Cyan
        $sysMainService = Get-Service -Name "SysMain" -ErrorAction SilentlyContinue
        if ($sysMainService) {
            Set-Service -Name "SysMain" -StartupType Disabled -ErrorAction SilentlyContinue
            Stop-Service -Name "SysMain" -Force -ErrorAction SilentlyContinue
            Write-Host "Superfetch/SysMain disabled." -ForegroundColor Green
        }
        
        # Disable Windows Update service temporarily
        Write-Host "Disabling Windows Update service temporarily..." -ForegroundColor Cyan
        $updateService = Get-Service -Name "wuauserv" -ErrorAction SilentlyContinue
        if ($updateService) {
            Set-Service -Name "wuauserv" -StartupType Manual -ErrorAction SilentlyContinue
            Stop-Service -Name "wuauserv" -Force -ErrorAction SilentlyContinue
            Write-Host "Windows Update disabled temporarily." -ForegroundColor Green
        }
        
        # Disable telemetry services
        Write-Host "Disabling telemetry services..." -ForegroundColor Cyan
        $telemetryServices = @("DiagTrack", "WpnService", "XblAuthManager", "XblGameSave")
        foreach ($serviceName in $telemetryServices) {
            $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
            if ($service) {
                Set-Service -Name $serviceName -StartupType Disabled -ErrorAction SilentlyContinue
                Stop-Service -Name $serviceName -Force -ErrorAction SilentlyContinue
                Write-Verbose-Output "Disabled $serviceName service."
            }
        }
        
        Write-Host "Background services optimized." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to optimize some background services: $_"
    }
}

function Set-DefenderExclusions {
    Write-Host "Configuring Windows Defender exclusions for benchmarking..." -ForegroundColor Yellow
    
    try {
        # Get current directory and parent directories for exclusions
        $currentPath = Get-Location
        $parentPath = Split-Path $currentPath -Parent
        $benchmarkDirs = @($currentPath, $parentPath, "C:\temp", "C:\benchmark-results")
        
        foreach ($dir in $benchmarkDirs) {
            if (Test-Path $dir) {
                Write-Host "Adding exclusion for directory: $dir" -ForegroundColor Cyan
                Add-MpPreference -ExclusionPath $dir -ErrorAction SilentlyContinue
                Write-Verbose-Output "Added exclusion for $dir"
            }
        }
        
        # Exclude benchmark processes
        $benchmarkProcesses = @("morsel-daemon.exe", "morsel-cli.exe", "morsel-tui.exe", "Ditto.exe")
        foreach ($process in $benchmarkProcesses) {
            Write-Host "Adding exclusion for process: $process" -ForegroundColor Cyan
            Add-MpPreference -ExclusionProcess $process -ErrorAction SilentlyContinue
            Write-Verbose-Output "Added exclusion for $process"
        }
        
        Write-Host "Windows Defender exclusions configured." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to configure Windows Defender exclusions: $_"
    }
}

function Disable-VisualEffects {
    Write-Host "Optimizing visual effects for benchmarking..." -ForegroundColor Yellow
    
    try {
        # Set visual effects for best performance
        $keyPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects"
        
        # Set visual effects to "Adjust for best performance"
        Set-ItemProperty -Path $keyPath -Name "VisualFXSetting" -Value 2 -Type DWord -ErrorAction SilentlyContinue
        
        # Disable specific visual effects
        $visualEffectsKey = "HKCU:\Control Panel\Desktop"
        Set-ItemProperty -Path $visualEffectsKey -Name "DragFullWindows" -Value "0" -ErrorAction SilentlyContinue
        Set-ItemProperty -Path $visualEffectsKey -Name "MenuShowDelay" -Value "0" -ErrorAction SilentlyContinue
        Set-ItemProperty -Path $visualEffectsKey -Name "UserPreferencesMask" -Value "9012038010000000" -ErrorAction SilentlyContinue
        
        # Disable animations
        $animationKey = "HKCU:\Control Panel\Desktop\WindowMetrics"
        Set-ItemProperty -Path $animationKey -Name "MinAnimate" -Value "0" -ErrorAction SilentlyContinue
        
        Write-Host "Visual effects optimized for performance." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to optimize visual effects: $_"
    }
}

function Set-DwmVSyncSettings {
    Write-Host "Configuring DWM VSync settings..." -ForegroundColor Yellow
    
    try {
        # Disable DWM VSync for more consistent frame timing
        $dwmKey = "HKCU:\Software\Microsoft\Windows\DWM"
        Set-ItemProperty -Path $dwmKey -Name "EnableAeroPeek" -Value 0 -Type DWord -ErrorAction SilentlyContinue
        Set-ItemProperty -Path $dwmKey -Name "AlwaysHibernateThumbnails" -Value 0 -Type DWord -ErrorAction SilentlyContinue
        
        Write-Host "DWM settings configured." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to configure DWM settings: $_"
    }
}

function Set-NetworkOptimization {
    Write-Host "Optimizing network settings for benchmarking..." -ForegroundColor Yellow
    
    try {
        # Disable Nagle's algorithm for lower latency
        $tcpKey = "HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"
        Set-ItemProperty -Path $tcpKey -Name "TcpAckFrequency" -Value 1 -Type DWord -ErrorAction SilentlyContinue
        Set-ItemProperty -Path $tcpKey -Name "TCPNoDelay" -Value 1 -Type DWord -ErrorAction SilentlyContinue
        
        Write-Host "Network settings optimized." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to optimize network settings: $_"
    }
}

function Set-SystemPriority {
    Write-Host "Configuring system priority settings..." -ForegroundColor Yellow
    
    try {
        # Set system foreground boost to disabled for more consistent scheduling
        $priorityKey = "HKLM:\SYSTEM\CurrentControlSet\Control\PriorityControl"
        Set-ItemProperty -Path $priorityKey -Name "Win32PrioritySeparation" -Value 0x28 -Type DWord -ErrorAction SilentlyContinue
        
        Write-Host "System priority configured." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to configure system priority: $_"
    }
}

function Show-EnvironmentSummary {
    Write-Host "`n========================================" -ForegroundColor Cyan
    Write-Host "Benchmark Environment Summary" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    
    # Show current power plan
    $currentPlan = powercfg -getactivescheme
    Write-Host "Power Plan: $currentPlan" -ForegroundColor Green
    
    # Show system info
    $osInfo = Get-WmiObject Win32_OperatingSystem
    Write-Host "OS: $($osInfo.Caption) Build $($osInfo.BuildNumber)" -ForegroundColor Green
    
    $cpuInfo = Get-WmiObject Win32_Processor
    Write-Host "CPU: $($cpuInfo.Name) - $($cpuInfo.NumberOfCores) cores, $($cpuInfo.NumberOfLogicalProcessors) logical processors" -ForegroundColor Green
    
    $memoryInfo = Get-WmiObject Win32_ComputerSystem
    $totalMemory = [math]::Round($memoryInfo.TotalPhysicalMemory / 1GB, 2)
    Write-Host "Memory: $totalMemory GB" -ForegroundColor Green
    
    # Show current state of services
    Write-Host "`nBackground Services Status:" -ForegroundColor Yellow
    $services = @("WSearch", "SysMain", "wuauserv")
    foreach ($serviceName in $services) {
        $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
        if ($service) {
            $status = if ($service.Status -eq "Stopped") { "Stopped (Good)" } else { "Running" }
            Write-Host "  $serviceName : $status" -ForegroundColor $(if ($service.Status -eq "Stopped") { "Green" } else { "Yellow" })
        }
    }
    
    Write-Host "========================================" -ForegroundColor Cyan
}

# Main execution
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Benchmark Environment Initialization" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Check for admin privileges
if (-not (Test-AdminPrivileges)) {
    Write-Error "This script requires administrator privileges. Please run as Administrator."
    exit 1
}

Write-Host "Administrator privileges confirmed." -ForegroundColor Green

# Execute optimization steps
if (-not $SkipPowerSettings) {
    Set-UltimatePerformancePowerPlan
}

Set-ProcessAffinity

if (-not $SkipServiceOptimization) {
    Optimize-BackgroundServices
}

if (-not $SkipDefenderExclusions) {
    Set-DefenderExclusions
}

if (-not $SkipVisualEffects) {
    Disable-VisualEffects
}

Set-DwmVSyncSettings
Set-NetworkOptimization
Set-SystemPriority

# Show summary
Show-EnvironmentSummary

Write-Host "`nBenchmark environment initialization completed!" -ForegroundColor Green
Write-Host "Note: Some changes may require a system restart to take full effect." -ForegroundColor Yellow
Write-Host "To restore original settings, run Restore-BenchmarkEnvironment.ps1" -ForegroundColor Yellow