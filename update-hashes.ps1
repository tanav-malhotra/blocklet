#!/usr/bin/env pwsh
# Helper script to download v0.1.3 binaries and update all package manager hashes

$ErrorActionPreference = 'Stop'
$VERSION = "v0.1.3"

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "  Blocklet v0.1.3 Package Hash Updater" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# Create temp directory
$tempDir = Join-Path $env:TEMP "blocklet-release"
New-Item -ItemType Directory -Force -Path $tempDir | Out-Null
Push-Location $tempDir

try {
    # Download binaries
    Write-Host "[1/4] Downloading Windows binary..." -ForegroundColor Yellow
    $winUrl = "https://github.com/tanav-malhotra/blocklet/releases/download/$VERSION/blocklet-windows-amd64.exe"
    Invoke-WebRequest -Uri $winUrl -OutFile "blocklet-windows-amd64.exe" -ErrorAction Stop
    
    Write-Host "[2/4] Downloading source tarball for AUR..." -ForegroundColor Yellow
    $srcUrl = "https://github.com/tanav-malhotra/blocklet/archive/$VERSION.tar.gz"
    Invoke-WebRequest -Uri $srcUrl -OutFile "$VERSION.tar.gz" -ErrorAction Stop
    
    # Calculate hashes
    Write-Host "[3/4] Calculating SHA256 hashes..." -ForegroundColor Yellow
    $winHash = (Get-FileHash "blocklet-windows-amd64.exe" -Algorithm SHA256).Hash.ToLower()
    $srcHash = (Get-FileHash "$VERSION.tar.gz" -Algorithm SHA256).Hash.ToLower()
    
    # Display results
    Write-Host ""
    Write-Host "==================================================" -ForegroundColor Green
    Write-Host "  SHA256 Hashes Calculated Successfully!" -ForegroundColor Green
    Write-Host "==================================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Windows Binary Hash:" -ForegroundColor Cyan
    Write-Host "  $winHash" -ForegroundColor White
    Write-Host ""
    Write-Host "Source Tarball Hash (for AUR):" -ForegroundColor Cyan
    Write-Host "  $srcHash" -ForegroundColor White
    Write-Host ""
    
    # Update files
    Write-Host "[4/4] Updating package manager files..." -ForegroundColor Yellow
    
    Pop-Location
    
    # Update Chocolatey
    $chocoFile = "choco/tools/chocolateyinstall.ps1"
    $chocoContent = Get-Content $chocoFile -Raw
    $chocoContent = $chocoContent -replace "checksum64\s*=\s*'[^']*'", "checksum64    = '$winHash'"
    Set-Content $chocoFile -Value $chocoContent -NoNewline
    Write-Host "  ✓ Updated $chocoFile" -ForegroundColor Green
    
    # Update PKGBUILD
    $pkgbuildFile = "PKGBUILD"
    $pkgbuildContent = Get-Content $pkgbuildFile -Raw
    $pkgbuildContent = $pkgbuildContent -replace "sha256sums=\('SKIP'\)", "sha256sums=('$srcHash')"
    Set-Content $pkgbuildFile -Value $pkgbuildContent -NoNewline
    Write-Host "  ✓ Updated $pkgbuildFile" -ForegroundColor Green
    
    # Update WinGet installer
    $wingetFile = "winget/blocklet.installer.yaml"
    $wingetContent = Get-Content $wingetFile -Raw
    $wingetContent = $wingetContent -replace "InstallerSha256:\s*<TO_BE_FILLED_AFTER_RELEASE>", "InstallerSha256: $winHash"
    Set-Content $wingetFile -Value $wingetContent -NoNewline
    Write-Host "  ✓ Updated $wingetFile" -ForegroundColor Green
    
    Write-Host ""
    Write-Host "==================================================" -ForegroundColor Green
    Write-Host "  All files updated successfully!" -ForegroundColor Green
    Write-Host "==================================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "  1. Review changes: git diff" -ForegroundColor White
    Write-Host "  2. Commit: git add . && git commit -m 'Update v0.1.3 hashes'" -ForegroundColor White
    Write-Host "  3. Push: git push origin main" -ForegroundColor White
    Write-Host "  4. Publish to Chocolatey, AUR, WinGet" -ForegroundColor White
    Write-Host ""
    
} catch {
    Pop-Location
    Write-Host ""
    Write-Host "ERROR: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host ""
    Write-Host "This usually means the GitHub release isn't ready yet." -ForegroundColor Yellow
    Write-Host "Wait a few more minutes and try again!" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Check build status at:" -ForegroundColor Cyan
    Write-Host "  https://github.com/tanav-malhotra/blocklet/actions" -ForegroundColor White
    Write-Host ""
    exit 1
}

