# Retro Realm nativo para Android (experimental)

La aplicación Android original **sigue siendo la que distribuye el workflow
\`RetroRealm.yml\` en \`main\`**. La compilación de esta rama no es
una versión para publicar: falta completar el port de sprites, IA y persistencia.

## Preparar audio

Macroquad acepta audio Vorbis/OGG y WAV. Los MP3 originales se conservan.
Se convierten en .ogg sin agregar las conversiones al repositorio.

Windows PowerShell:

\`\`\`powershell
.\scripts\prepare-audio.ps1
cargo run -p retro-realm
\`\`\`

Linux/CI:

\`\`\`bash
bash scripts/prepare-audio.sh
cargo run -p retro-realm
\`\`\`

Se requiere \`ffmpeg\` instalado en PATH.

## APK nativo

El workflow \`.github/workflows/rust-android.yml\` intenta compilar un APK
experimental Rust + Macroquad, separado del APK WebView anterior.

Utiliza Rust 1.95 + Android NDK r28 y el fork actualizado de
[hoklims/cargo-quad-apk](https://github.com/hoklims/cargo-quad-apk), puesto que
la imagen Docker histórica del tutorial Macroquad trae Cargo 1.61 y no soporta
las opciones actuales del workspace.

**Nota:** la imagen y la herramienta Android de Macroquad pueden necesitar ajustes
según la versión de Cargo/Android NDK; no prometer un APK funcional hasta
que CI lo construya y un teléfono real lo ejecute.

El identificador Android se mantiene \`com.pixelquest.game\`, pero instalar
un APK de firma diferente sobre el original puede fallar o provocar pérdida
de ajustes: probar en un entorno separado, sin desinstalar el juego anterior.
