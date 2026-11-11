# Retro Realm

Retro Realm es un juego de acción y plataformas 2D con tres personajes, enemigos, dos bosques y jefes finales.

## Edición original y migración nativa

**La rama `main` conserva la versión original HTML/WebView.** La migración experimental a Rust + Macroquad se desarrolla en [`refactor/rust-macroquad-migration`](https://github.com/jonathanthxd/Retro-realm/tree/refactor/rust-macroquad-migration), con un [pull request en borrador](https://github.com/jonathanthxd/Retro-realm/pull/1).

| Carpeta | Función |
|---|---|
| `Retro Realm.html` | Juego original, referencia funcional y visual |
| `crates/realm-core` | Simulación determinista en Rust, sin dependencias gráficas |
| `apps/retro-realm` | Aplicación nativa con Macroquad, teclado y controles táctiles |
| `apps/retro-realm/assets/sprites` | Sprites PNG y decoración **extraídos del Canvas original** |
| `docs/MIGRATION.md` | Comparación de sistemas, límites y criterios de paridad |
| `docs/ANDROID.md` | Build APK y guía de prueba en Android |
| `docs/VISUAL_AUDIO_PARITY.md` | Auditoría audiovisual, primera actualización visual y pendientes |
| `apps/retro-realm/src/scenery.rs` y `ui.rs` | Capas de paisaje originales y recuperación de interfaz |

### Ejecutar en Windows

Instalar [Rust](https://rustup.rs/) y [FFmpeg](https://ffmpeg.org/). Desde la raíz del repositorio:

```powershell
.\scripts\prepare-audio.ps1
cargo run -p retro-realm
```

Para pruebas de simulación:

```powershell
cargo test -p realm-core
cargo check --workspace --all-targets
```

Los controles de PC incluyen A/D o flechas para moverte, espacio para saltar, F o X para golpear, 1/2/3 para habilidades, C/Shift para dash, Q/E para escudo y Enter para interactuar. ESC abre pausa, H regresa al menú y M silencia/reactiva audio.

### APK Android nativo

El workflow [Rust / Macroquad — Android APK experimental](https://github.com/jonathanthxd/Retro-realm/actions/workflows/rust-android.yml) compila APKs de prueba. Abre una ejecución exitosa y descarga el artefacto **`RetroRealm-Rust-APK-PRELIMINAR`**.

La primera actualización visual cambia el menú principal, el selector, el HUD, los controles y el escenario; añade cámara vertical amortiguada. Las [capturas de referencia HTML](https://github.com/jonathanthxd/Retro-realm/actions/workflows/visual-reference.yml) pueden compararse con las [capturas nativas Rust](https://github.com/jonathanthxd/Retro-realm/actions/workflows/rust-screenshots.yml). **El aspecto todavía no ha sido validado como equivalente al original.**

Este APK usa el identificador `com.pixelquest.game.preview` para poder instalarse junto al juego original. **No es una versión final ni ha sustituido el WebView original.** Requiere pruebas en móviles reales y comparación funcional antes de cualquier reemplazo.

### Derechos de autor

Este fork no incluye una licencia de reutilización explícita declarada por el upstream. La migración no supone que se hayan concedido derechos de redistribución independiente del código, imágenes o música originales.
