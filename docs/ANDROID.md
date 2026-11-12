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

## Nueva APK de pruebas visuales

La actualización nativa `version_code=6` incluye reconstrucción del menú, selector, créditos originales, HUD y barra segmentada de jefe, iconos de control, paisaje Canvas, cámara horizontal/vertical amortiguada, poses originales, animaciones de enemigos, chispas, números de daño, rayos, lluvia y viñeta.

Las capturas nativas de escritorio de este código se publican en el workflow [Rust / Macroquad — capturas visuales nativas](https://github.com/jonathanthxd/Retro-realm/actions/workflows/rust-screenshots.yml). Son una herramienta de comparación, **no sustituyen** la revisión en Android.

## Comprobaciones móviles

- Instalar junto al WebView anterior (no reemplazar ni desinstalar el juego antiguo).
- Comprobar orientación horizontal, imagen no invertida, recorte en pantallas con notch.
- Escoger cada personaje, jugar, combatir, abrir cofres y atravesar ambos portales.
- Confirmar que el audio funciona con auriculares y al volver de segundo plano.
- Medir FPS, consumo de memoria y temperatura tras una partida larga.
- Revisar los artefactos de captura/logcat del smoke test en caso de fallos.

**Importante:** que Cargo cree el .apk no garantiza que se ejecute en todas las versiones Android ni que los controles sean equivalentes a los del HTML. El identificador de preview no puede leer por sí mismo `localStorage` privado de la otra aplicación.

## Qué verificar visualmente en la APK v6

Abrir inicio y créditos; elegir los tres héroes; recorrer ambos bosques y observar suelo, fondo, nubes, lluvia y cámara; probar salto/caída/dash/escudo; golpear y derrotar enemigos (números y partículas); revisar la barra de jefe; comprobar controles táctiles en pantalla pequeña. El audio sigue siendo provisional respecto al HTML.
