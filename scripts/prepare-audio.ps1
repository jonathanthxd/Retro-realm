$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$destination = Join-Path $root "apps\retro-realm\assets"
New-Item -ItemType Directory -Force -Path $destination | Out-Null
if (-not (Get-Command ffmpeg -ErrorAction SilentlyContinue)) {
    throw "Se necesita ffmpeg instalado y disponible en PATH."
}
foreach ($name in @("menu", "bosque_nocturno", "bosque_oscuro", "jefe")) {
    $source = Join-Path $root "$name.mp3"
    if (-not (Test-Path $source)) { throw "No existe $source" }
    $out = Join-Path $destination "$name.ogg"
    & ffmpeg -hide_banner -loglevel error -y -i $source -vn -c:a libvorbis -q:a 5 $out
    if ($LASTEXITCODE -ne 0) { throw "Error al convertir $name" }
    Write-Host "Convertido $name.ogg"
}
