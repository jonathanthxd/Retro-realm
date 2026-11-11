# Android nativo experimental — Retro Realm

El juego anterior sigue siendo HTML/WebView en `main`. Esta rama crea un **APK Android real con Rust + Macroquad**, separado mediante el identificador `com.pixelquest.game.preview`.

## Construcción automática

GitHub Actions: [Rust / Macroquad — Android APK experimental](https://github.com/jonathanthxd/Retro-realm/actions/workflows/rust-android.yml).

El workflow usa Rust 1.95, Android SDK/NDK r28, `cargo-quad-apk` y FFmpeg. Desde una ejecución satisfactoria, descargar el artefacto **`RetroRealm-Rust-APK-PRELIMINAR`**. La extensión del archivo de descarga es .zip y dentro está el .apk. Los artefactos de prueba caducan a los siete días.

Los builds de `push` generan el APK de prueba. El smoke test en emulador, por ser lento y costoso, se ejecuta **solo** mediante `workflow_dispatch` activando `run_emulator`.

## Audio y PC

Los archivos MP3 originales permanecen en la raíz. Para crear OGG compatibles con el motor:

```powershell
.\scripts\prepare-audio.ps1
cargo run -p retro-realm
```

Requiere FFmpeg y Rust. Los archivos convertidos no se suben al control de versiones; CI los genera durante la compilación.

La música del primer bosque usa `bosque_nocturno.mp3`, recurso incluido en la raíz pero **no utilizado por el HTML original**. Los efectos generados en Rust son recreaciones provisionales del sintetizador del navegador, no grabaciones ni reproducciones 1:1.

## Comprobaciones móviles

- Instalar junto al WebView anterior (no reemplazar ni desinstalar el juego antiguo).
- Comprobar orientación horizontal, imagen no invertida, recorte en pantallas con notch.
- Escoger cada personaje, jugar, combatir, abrir cofres y atravesar ambos portales.
- Confirmar que el audio funciona con auriculares y al volver de segundo plano.
- Medir FPS, consumo de memoria y temperatura tras una partida larga.
- Revisar los artefactos de captura/logcat del smoke test en caso de fallos.

**Importante:** que Cargo cree el .apk no garantiza que se ejecute en todas las versiones Android ni que los controles sean equivalentes a los del HTML. El identificador de preview no puede leer por sí mismo `localStorage` privado de la otra aplicación.
