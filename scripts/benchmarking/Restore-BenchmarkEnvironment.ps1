# Restore-BenchmarkEnvironment.ps1
# Restores system settings after benchmarking
# Reverses changes made by Initialize-BenchmarkEnvironment.ps1

param(
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

function Restore-PowerSettings {
    Write-Host "Restoring power settings..." -ForegroundColor Yellow
    
    try {
        # Restore Balanced power plan
        $balancedGuid = "381b4222-f694-41f0-9685-ff5bb260df2e"
        powercfg -setactive $balancedGuid
        
        Write-Host "Power settings restored to Balanced plan." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to restore power settings: $_"
    }
}

function Restore-BackgroundServices {
    Write-Host "Restoring background services..." -ForegroundColor Yellow
    
    try {
        # Re-enable Windows Search
        Write-Host "Re-enabling Windows Search service..." -ForegroundColor Cyan
        $searchService = Get-Service -Name "WSearch" -ErrorAction SilentlyContinue
        if ($searchService) {
            Set-Service -Name "WSearch" -StartupType Automatic -ErrorAction SilentlyContinue
            Start-Service -Name "WSearch" -ErrorAction SilentlyContinue
            Write-Host "Windows Search re-enabled." -ForegroundColor Green
        }
        
        # Re-enable Superfetch/SysMain
        Write-Host "Re-enabling Superfetch/SysMain service..." -ForegroundColor Cyan
        $sysMainService = Get-Service -Name "SysMain" -ErrorAction SilentlyContinue
        if ($sysMainService) {
            Set-Service -Name "SysMain" -StartupType Automatic -ErrorAction SilentlyContinue
            Start-Service -Name "SysMain" -ErrorAction SilentlyContinue
            Write-Host "Superfetch/SysMain re-enabled." -ForegroundColor Green
        }
        
        # Re-enable Windows Update service
        Write-Host "Re-enabling Windows Update service..." -ForegroundColor Cyan
        $updateService = Get-Service -Name "wuauserv" -ErrorAction SilentlyContinue
        if ($updateService) {
            Set-Service -Name "wuauserv" -StartupType Automatic -ErrorAction SilentlyContinue
            Write-Host "Windows Update re-enabled." -ForegroundColor Green
        }
        
        # Re-enable telemetry services
        Write-Host "Re-enabling telemetry services..." -ForegroundColor Cyan
        $telemetryServices = @("DiagTrack", "WpnService")
        foreach ($serviceName in $telemetryServices) {
            $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
            if ($service) {
                Set-Service -Name $serviceName -StartupType Automatic -ErrorAction SilentlyContinue
                Start-Service -Name $serviceName -ErrorAction SilentlyContinue
                Write-Verbose-Output "Re-enabled $serviceName service."
            }
        }
        
        Write-Host "Background services restored." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to restore some background services: $_"
    }
}

function Remove-DefenderExclusions {
    Write-Host "Removing Windows Defender exclusions..." -ForegroundColor Yellow
    
    try {
        # Remove directory exclusions
        $currentPath = Get-Location
        $parentPath = Split-Path $currentPath -Parent
        $benchmarkDirs = @($currentPath, $parentPath, "C:\temp", "C:\benchmark-results")
        
        foreach ($dir in $benchmarkDirs) {
            if (Test-Path $dir) {
                Write-Host "Removing exclusion for directory: $dir" -ForegroundColor Cyan
                Remove-MpPreference -ExclusionPath $dir -ErrorAction SilentlyContinue
                Write-Verbose-Output "Removed exclusion for $dir"
            }
        }
        
        # Remove process exclusions
        $benchmarkProcesses = @("morsel-daemon.exe", "morsel-cli.exe", "morsel-tui.exe", "Ditto.exe")
        foreach ($process in $benchmarkProcesses) {
            Write-Host "Removing exclusion for process: $process" -ForegroundColor Cyan
            Remove-MpPreference -ExclusionProcess $process -ErrorAction SilentlyContinue
            Write-Verbose-Output "Removed exclusion for $process"
        }
        
        Write-Host "Windows Defender exclusions removed." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to remove Windows Defender exclusions: $_"
    }
}

function Restore-VisualEffects {
    Write-Host "Restoring visual effects..." -ForegroundColor Yellow
    
    try {
        # Restore default visual effects
        $keyPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects"
        Set-ItemProperty -Path $keyPath -Name "VisualFXSetting" -Value 0 -Type DWord -ErrorAction SilentlyContinue
        
        # Restore default desktop settings
        $visualEffectsKey = "HKCU:\Control Panel\Desktop"
        Set-ItemProperty -Path $visualEffectsKey -Name "DragFullWindows" -Value "1" -ErrorAction SilentlyContinue
        Set-ItemProperty -Path $visualEffectsKey -Name "MenuShowDelay" -Value "400" -ErrorAction SilentlyContinue
        Set-ItemProperty -Path $visualEffectsKey -Name "UserPreferencesMask" -Value "9E0E038012000000" -ErrorAction SilentlyContinue
        
        # Re-enable animations
        $animationKey = "HKCU:\Control Panel\Desktop\WindowMetrics"
        Set-ItemProperty -Path $animationKey -Name "MinAnimate" -Value "1" -ErrorAction SilentlyContinue
        
        Write-Host "Visual effects restored to default." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to restore visual effects: $_"
    }
}

function Restore-DwmSettings {
    Write-Host "Restoring DWM settings..." -ForegroundColor Yellow
    
    try {
        # Restore default DWM settings
        $dwmKey = "HKCU:\Software\Microsoft\Windows\DWM"
        Set-ItemProperty -Path $dwmKey -Name "EnableAeroPeek" -Value 1 -Type DWord -ErrorAction SilentlyContinue
        Set-ItemProperty -Path $dwmKey -Name "AlwaysHibernateThumbnails" -Value 1 -Type DWord -ErrorAction SilentlyContinue
        
        Write-Host "DWM settings restored." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to restore DWM settings: $_"
    }
}

function Restore-NetworkSettings {
    Write-Host "Restoring network settings..." -ForegroundColor Yellow
    
    try {
        # Restore default TCP settings
        $tcpKey = "HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"
        Remove-ItemProperty -Path $tcpKey -Name "TcpAckFrequency" -ErrorAction SilentlyContinue
        Remove-ItemProperty -Path $tcpKey -Name "TCPNoDelay" -ErrorAction SilentlyContinue
        
        Write-Host "Network settings restored." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to restore network settings: $_"
    }
}

function Restore-SystemPriority {
    Write-Host "Restoring system priority settings..." -ForegroundColor Yellow
    
    try {
        # Restore default system priority
        $priorityKey = "HKLM:\SYSTEM\CurrentControlSet\Control\PriorityControl"
        Set-ItemProperty -Path $priorityKey -Name "Win32PrioritySeparation" -Value 0x18 -Type DWord -ErrorAction SilentlyContinue
        
        Write-Host "System priority restored." -ForegroundColor Green
    } catch {
        Write-Warning "Failed to restore system priority: $_"
    }
}

function Show-RestoreSummary {
    Write-Host "`n========================================" -ForegroundColor Cyan
    Write-Host "Environment Restore Summary" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    
    # Show current power plan
    $currentPlan = powercfg -getactivescheme
    Write-Host "Power Plan: $currentPlan" -ForegroundColor Green
    
    # Show service status
    Write-Host "`nBackground Services Status:" -ForegroundColor Yellow
    $services = @("WSearch", "SysMain", "wuauserv")
    foreach ($serviceName in $services) {
        $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
        if ($service) {
            $status = if ($service.Status -eq "Running") { "Running (Restored)" } else { "Stopped" }
            Write-Host "  $serviceName : $status" -ForegroundColor $(if ($service.Status -eq "Running") { "Green" } else { "Yellow" })
        }
    }
    
    Write-Host "========================================" -ForegroundColor Cyan
}

# Main execution
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Benchmark Environment Restoration" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Check for admin privileges
if (-not (Test-AdminPrivileges)) {
    Write-Error "This script requires administrator privileges. Please run as Administrator."
    exit 1
}

Write-Host "Administrator privileges confirmed." -ForegroundColor Green

# Execute restoration steps
Restore-PowerSettings
Restore-BackgroundServices
Remove-DefenderExclusions
Restore-VisualEffects
Restore-DwmSettings
Restore-NetworkSettings
Restore-SystemPriority

# Show summary
Show-RestoreSummary

Write-Host "`nBenchmark environment restoration completed!" -ForegroundColor Green
Write-Host "Some changes may require a system restart to take full effect." -ForegroundColor Yellow