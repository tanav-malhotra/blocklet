$ErrorActionPreference = 'Stop'
$toolsDir   = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64      = 'https://github.com/tanav-malhotra/blocklet/releases/download/v0.1.2/blocklet-windows-amd64.exe'

$packageArgs = @{
  packageName   = $env:ChocolateyPackageName
  unzipLocation = $toolsDir
  url64bit      = $url64
  checksum64    = '<TO_BE_FILLED_AFTER_RELEASE>'
  checksumType64= 'sha256'
}

Install-ChocolateyBinFile -Name 'blocklet' -Path "$toolsDir\blocklet.exe"
Get-ChocolateyWebFile @packageArgs

