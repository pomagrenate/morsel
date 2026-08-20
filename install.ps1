param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:USERPROFILE\.local\bin"
)

$ErrorActionPreference = "Stop"

Write-Host "Installing Morsel $Version..."

# Detect architecture
$Arch = $env:PROCESSOR_ARCHITECTURE
switch ($Arch) {
    "AMD64" { $ArchSuffix = "x86_64" }
    "ARM64" { $ArchSuffix = "arm64" }
    default {
        Write-Host "Unsupported architecture: $Arch"
        exit 1
    }
}

$AssetName = "morsel-windows-$ArchSuffix"
$DownloadUrl = "https://github.com/yourusername/morsel/releases/download/$Version/$AssetName.zip"

Write-Host "Downloading from $DownloadUrl..."

# Create temp directory
$TmpDir = Join-Path $env:TEMP "morsel-install"
New-Item -ItemType Directory -Path $TmpDir -Force | Out-Null
Set-Location $TmpDir

# Download
Invoke-WebRequest -Uri $DownloadUrl -OutFile "morsel.zip"

# Extract
Expand-Archive -Path "morsel.zip" -Force

# Install
New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
Move-Item -Path "morsel.exe" -Destination $InstallDir -Force

# Cleanup
Set-Location -
Remove-Item -Path $TmpDir -Recurse -Force

Write-Host "Morsel installed to $InstallDir\morsel.exe"
Write-Host "Add $InstallDir to your PATH if not already added"
