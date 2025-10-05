# PohLang Installation Script for Windows
# Usage: Run this in PowerShell (as Administrator for system-wide install)
#        Or run as normal user for current user install

$VERSION = "v0.5.2"
$REPO = "AlhaqGH/PohLang"
$PLATFORM = "windows-x64"

Write-Host "🚀 Installing PohLang $VERSION for Windows..." -ForegroundColor Cyan
Write-Host ""

# Create temp directory
$TempDir = Join-Path $env:TEMP "pohlang-install"
New-Item -ItemType Directory -Force -Path $TempDir | Out-Null
Set-Location $TempDir

# Download
$DownloadUrl = "https://github.com/$REPO/releases/download/$VERSION/pohlang-$VERSION-$PLATFORM.zip"
$ZipPath = Join-Path $TempDir "pohlang.zip"

Write-Host "📥 Downloading from $DownloadUrl..." -ForegroundColor Yellow
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath -UseBasicParsing
} catch {
    Write-Host "❌ Download failed: $_" -ForegroundColor Red
    exit 1
}

# Extract
Write-Host "📦 Extracting..." -ForegroundColor Yellow
Expand-Archive -Path $ZipPath -DestinationPath $TempDir -Force

# Determine install location
$IsAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if ($IsAdmin) {
    # System-wide installation
    $InstallDir = "C:\Program Files\PohLang"
    Write-Host "📋 Installing to $InstallDir (system-wide)..." -ForegroundColor Yellow
} else {
    # User installation
    $InstallDir = Join-Path $env:LOCALAPPDATA "PohLang"
    Write-Host "📋 Installing to $InstallDir (current user)..." -ForegroundColor Yellow
}

# Create install directory and copy binary
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item -Path "pohlang.exe" -Destination $InstallDir -Force

# Add to PATH
$PathScope = if ($IsAdmin) { "Machine" } else { "User" }
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", $PathScope)

if ($CurrentPath -notlike "*$InstallDir*") {
    Write-Host "🔧 Adding to PATH..." -ForegroundColor Yellow
    $NewPath = "$CurrentPath;$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, $PathScope)
    $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
}

# Verify installation
$PohlangPath = Join-Path $InstallDir "pohlang.exe"
if (Test-Path $PohlangPath) {
    Write-Host ""
    Write-Host "✅ PohLang installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Try it out:" -ForegroundColor Cyan
    Write-Host "  pohlang --version"
    Write-Host ""
    Write-Host "Create a test program:" -ForegroundColor Cyan
    Write-Host "  echo 'Start Program' > hello.poh"
    Write-Host "  echo 'Write `"Hello from PohLang!`"' >> hello.poh"
    Write-Host "  echo 'End Program' >> hello.poh"
    Write-Host "  pohlang --run hello.poh"
    Write-Host ""
    Write-Host "📚 Documentation: https://github.com/$REPO" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "⚠️  Note: You may need to restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
} else {
    Write-Host "❌ Installation failed. Please install manually from:" -ForegroundColor Red
    Write-Host "   https://github.com/$REPO/releases"
    exit 1
}

# Cleanup
Set-Location $env:USERPROFILE
Remove-Item -Path $TempDir -Recurse -Force
