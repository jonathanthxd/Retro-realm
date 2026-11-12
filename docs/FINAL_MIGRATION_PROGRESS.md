# Final Migration Update — avance v9 (implementación en curso)

**Estado:** primer bloque integrado en Rust, pendiente de pruebas en un teléfono real. No significa que el port esté terminado.

## Implementado

- `layout.rs`: geometría UI compartida de 540 unidades lógicas de alto, ancho proporcional al viewport, escala de capturas de menú sin deformación, coordenadas táctiles unificadas y rectángulos de botón adaptativos. El framebuffer del mundo usa el mismo aspecto, eliminando el clamp antiguo 320–480.
- `controls.rs`: entrada multitáctil usando los rectángulos de la misma capa de dibujo, botones escalados y joystick alternativo con eje horizontal y gesto vertical de salto.
- `ui_native.rs`: escala adaptable de menús y pausa, control de volumen visual interactivo, opción seleccionada de estilos y modos, overlay para editar controles, y pantalla de derrota en resolución nativa con nivel/oro y acción de reinicio.
- `settings.rs`: archivo JSON versión 1, validación y guardado atómico de nombre, récord de nivel, clase, volumen de música/SFX, estilo, modo de controles, tamaño y desplazamiento de cada botón.
- `main.rs`: conecta el estado de los ajustes con audio e input, solicita teclado Android al pulsar el nombre, y persiste el perfil sin modificar `main` ni el HTML. El volumen música/SFX es independiente; los tres estilos cambian de pista de prueba, pero NO reproducen todavía el WebAudio exacto.
- `sfx.rs`: aplica el nivel de volumen a efectos, separado del audio musical.
- `rust-screenshots.yml`: genera capturas en 960×540, 1024×768 (4:3) y 1200×540 (20:9) para menú, juego y pausa.
- `Cargo.toml`: preview Android `version_code=9`; paquete de prueba `com.pixelquest.game.preview`, separado del original.

## Verificaciones

- [Pruebas Rust exitosas](https://github.com/jonathanthxd/Retro-realm/actions/runs/43158067816).
- [Capturas responsive CI exitosas](https://github.com/jonathanthxd/Retro-realm/actions/runs/43158359244), 9 escenas entregadas, aspecto 4:3/16:9/20:9.
- [APK Android compilada](https://github.com/jonathanthxd/Retro-realm/actions/runs/43158359284); artefacto `RetroRealm-Rust-APK-PRELIMINAR`, versionCode 9.
- SHA-256 del APK descomprimido: `ced037395d7146cafbb8aaebee767b6e614731ce855734cf55a8b9145357bbc3`.

## No completado / no afirmar

1. Safe areas de notch, navegación por gestos y cutouts **medidas con los insets reales de Android**: sigue pendiente. Los márgenes actuales son conservadores, no una lectura de Android WindowInsets.
2. Teclado Android: `miniquad::window::show_keyboard` se solicita y el campo lee caracteres, pero faltan pruebas de IME/composición/acentos/pegar en un móvil real.
3. Pistas nocturnas, 3 estilos, envolventes y SFX **idénticos al WebAudio** original: no terminados. La selección de estilo v9 modifica provisionalmente las pistas existentes.
4. Persistencia Android en perfiles multusuario y actualización a package definitivo: por ahora se guarda en ruta privada del identificador **preview**; para cambiar identificador hay que adaptar a `NativeActivity.getFilesDir()`/puente nativo.
5. Ajustes más avanzados de pantalla/HUD y acabado táctil del editor en teléfonos de ratios extremos: pendiente de QA en móvil.
6. Ciclo de vida Android (background/foreground, botón Atrás, llamada, foco de audio), 60/90/120 Hz y FPS en hardware real: sin validar.
7. Comparación de partidas completas de las tres clases, Ogro y Rey contra HTML: sin certificación integral.
8. APK final firmada y reemplazo de la original: NO realizado; PR #1 debe permanecer en borrador.

**Próximo bloque técnico:** audio WebAudio fiel, IME Android e insets reales, pruebas de jugabilidad Android y validación de los ajustes físicos.

**Regla:** no fusionar a main sin aceptación explícita.
