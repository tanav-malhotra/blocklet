#!/usr/bin/env pwsh
# Build and prepare release for v0.1.3

$ErrorActionPreference = 'Stop'
$VERSION = "v0.1.3"

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "  Building Blocklet $VERSION for Release" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# Build for current platform
Write-Host "[1/3] Building release binary..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Create release directory
$releaseDir = "release-$VERSION"
New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null

# Copy binary
Write-Host "[2/3] Copying binary..." -ForegroundColor Yellow
if ($IsWindows -or $env:OS -eq "Windows_NT") {
    Copy-Item "target/release/blocklet.exe" "$releaseDir/blocklet-windows-amd64.exe"
    $binaryPath = "$releaseDir/blocklet-windows-amd64.exe"
} elseif ($IsMacOS) {
    Copy-Item "target/release/blocklet" "$releaseDir/blocklet-macos-amd64"
    $binaryPath = "$releaseDir/blocklet-macos-amd64"
} else {
    Copy-Item "target/release/blocklet" "$releaseDir/blocklet-linux-amd64"
    $binaryPath = "$releaseDir/blocklet-linux-amd64"
}

# Calculate hash
Write-Host "[3/3] Calculating SHA256..." -ForegroundColor Yellow
$hash = (Get-FileHash $binaryPath -Algorithm SHA256).Hash.ToLower()

Write-Host ""
Write-Host "==================================================" -ForegroundColor Green
Write-Host "  Build Complete!" -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Green
Write-Host ""
Write-Host "Binary location:" -ForegroundColor Cyan
Write-Host "  $binaryPath" -ForegroundColor White
Write-Host ""
Write-Host "SHA256 Hash:" -ForegroundColor Cyan
Write-Host "  $hash" -ForegroundColor White
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "  1. Go to: https://github.com/tanav-malhotra/blocklet/releases/new" -ForegroundColor White
Write-Host "  2. Choose tag: $VERSION" -ForegroundColor White
Write-Host "  3. Title: $VERSION - Fix spacing after certain letters" -ForegroundColor White
Write-Host "  4. Description: Copy from CHANGELOG.md" -ForegroundColor White
Write-Host "  5. Upload binary from: $releaseDir" -ForegroundColor White
Write-Host "  6. Publish release" -ForegroundColor White
Write-Host ""


