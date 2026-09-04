param(
  [Parameter(Mandatory=$true)][string]$ExePath,
  [Parameter(Mandatory=$true)][string]$CargoProjectPath,
  [string]$Version = "1.0.0",
  [string]$OutDir = ".\release_build"
)
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$exe = (Resolve-Path $ExePath).Path
$project = (Resolve-Path $CargoProjectPath).Path
$cargoToml = Join-Path $project "Cargo.toml"
$cargoLock = Join-Path $project "Cargo.lock"
if (!(Test-Path $cargoToml)) { throw "Cargo.toml not found: $cargoToml" }
if (!(Test-Path $cargoLock)) { throw "Cargo.lock not found. Run the verified V1 build first: $cargoLock" }

$dest = Join-Path (Resolve-Path (New-Item -ItemType Directory -Force $OutDir)).Path "Yekaterina_v${Version}_windows-x64"
if (Test-Path $dest) { Remove-Item -Recurse -Force $dest }
New-Item -ItemType Directory -Force $dest | Out-Null
New-Item -ItemType Directory -Force (Join-Path $dest "third_party_licenses") | Out-Null
Copy-Item $exe (Join-Path $dest "yekaterina.exe")
Copy-Item (Join-Path $root "LICENSE.txt") $dest
Copy-Item (Join-Path $root "PRIVACY.md") $dest
Copy-Item (Join-Path $root "releases\v1.0.0\RELEASE_NOTES.md") $dest
Copy-Item (Join-Path $root "SMOKE_TEST_WINDOWS.bat") $dest
Copy-Item -Recurse (Join-Path $root "tools") $dest

$metadataJson = & cargo metadata --locked --format-version 1 --manifest-path $cargoToml
if ($LASTEXITCODE -ne 0) { throw "cargo metadata failed" }
$meta = $metadataJson | ConvertFrom-Json
$rootId = $meta.resolve.root
$packages = @($meta.packages | Where-Object { $_.id -ne $rootId } | Sort-Object name, version)
$lines = @(
  "# Third-party components",
  "",
  "Generated from the locked dependency graph used to build Yekaterina v$Version.",
  "",
  "| Component | Version | Declared license | Repository |",
  "|---|---:|---|---|"
)
foreach ($pkg in $packages) {
  $license = if ($pkg.license) { $pkg.license } else { "NOT DECLARED" }
  $repo = if ($pkg.repository) { $pkg.repository } else { "" }
  $lines += "| $($pkg.name) | $($pkg.version) | $license | $repo |"
  $pkgDir = Split-Path -Parent $pkg.manifest_path
  $licDest = Join-Path (Join-Path $dest "third_party_licenses") ("{0}-{1}" -f $pkg.name,$pkg.version)
  $candidates = @(Get-ChildItem -LiteralPath $pkgDir -File -ErrorAction SilentlyContinue | Where-Object { $_.Name -match '^(LICENSE|LICENCE|COPYING|NOTICE|UNLICENSE)' })
  if ($candidates.Count -gt 0) {
    New-Item -ItemType Directory -Force $licDest | Out-Null
    foreach ($f in $candidates) { Copy-Item -LiteralPath $f.FullName -Destination $licDest }
  }
}
$lines | Set-Content -Encoding UTF8 (Join-Path $dest "THIRD_PARTY_COMPONENTS.md")

$hash = (Get-FileHash -Algorithm SHA256 (Join-Path $dest "yekaterina.exe")).Hash.ToLowerInvariant()
"$hash  yekaterina.exe" | Set-Content -Encoding ASCII (Join-Path $dest "SHA256SUMS.txt")

$zip = Join-Path (Split-Path -Parent $dest) ("Yekaterina_v{0}_windows-x64.zip" -f $Version)
if (Test-Path $zip) { Remove-Item -Force $zip }
Compress-Archive -Path (Join-Path $dest '*') -DestinationPath $zip -CompressionLevel Optimal
$zipHash=(Get-FileHash -Algorithm SHA256 $zip).Hash.ToLowerInvariant()
"$zipHash  $(Split-Path -Leaf $zip)" | Set-Content -Encoding ASCII ($zip + ".sha256.txt")
Write-Host "PASS: release asset created"
Write-Host "ZIP:  $zip"
Write-Host "SHA:  $($zip).sha256.txt"
Write-Host "NOTE: review THIRD_PARTY_COMPONENTS.md and third_party_licenses before publishing."
