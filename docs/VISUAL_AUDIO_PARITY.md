# Retro Realm — fidelidad visual y auditiva

## Actualización Visual Clarity v7 — renderizado de la UI original

**Causa confirmada con el HTML:** el mundo se dibuja en Canvas de 180 px de alto (`#cv`), pero el HUD final (`#hd2`), inicio (`#home`), selección (`#hs`), pausa (`#pzp`) y controles táctiles (`#ctl`) se componen en CSS al tamaño de pantalla. En la primera APK de Rust, todo se rasterizaba a 180 px y después se ampliaba. Eso hacía que el HUD se viese innecesariamente pixelado incluso con arte correcto.

**Corrección implementada en `refactor/rust-macroquad-migration`:**

- `scripts/export-sprites.mjs` abre el HTML original en Chrome e **imprime los elementos DOM finales** en PNG transparentes a 960×540: `ui_hd2_*.png`, `ui_dom_*.png`, `ui_home_*.png`, `ui_select_*.png`, `ui_credits.png`, `ui_pause_panel.png`. También exporta las cajas CSS originales en `ui_*_layout.csv`.
- `assets.rs` carga esos assets por clase con filtrado de imagen adecuado. `ui_native.rs` los compone **después** del framebuffer retro (HUD, vida/maná dinámicos, nombre/NV/oro/diamantes, menú, selección, créditos, pausa y controles), con una cámara cuya orientación ya se corrigió.
- `main.rs` conserva el juego a 320–480×180, pasa la UI CSS al framebuffer de pantalla y utiliza rectángulos originales para pulsaciones en menús y botones básicos de pausa. La UI vieja se mantiene únicamente como fallback si faltasen assets.
- `render.rs` conserva sprites originales con filtrado `Nearest` y redondea píxeles del jugador/enemigos en coordenadas de cámara para evitar vibraciones.
- `Cargo.toml` versión Android de preview **`version_code=7`**; paquete aislado `com.pixelquest.game.preview`.

**Pruebas terminadas:** [Rust CI](https://github.com/jonathanthxd/Retro-realm/actions/runs/43136143942), [6 capturas nativas: menú, selección, créditos, nocturno, oscuro y pausa](https://github.com/jonathanthxd/Retro-realm/actions/runs/43136139122), [APK Android](https://github.com/jonathanthxd/Retro-realm/actions/runs/43136138953) concluyeron correctamente. Las capturas muestran una mejora muy grande en la fidelidad visual del HUD y menús, **no una certificación automática de paridad al 100%**.

**Aún falta:** probar la APK en móviles reales y distintas relaciones de aspecto/safe areas, ajustar diferencias residuales en composiciones y combate, persistencia de preferencias, y activar opciones avanzadas de pausa (joystick, mover/restablecer controles, escalas, estilos de música y volúmenes independientes). Actualmente esas opciones **se ven como el HTML, pero no todas ejecutan su acción**. La música del Bosque Nocturno sigue sin replicar el secuenciador WebAudio.

---


## Cambio de prioridad

La prueba del APK de migración ha confirmado una diferencia audiovisual muy grande respecto al HTML original. La arquitectura Rust/Macroquad es una base para conservar, pero **no se considera un resultado visual ni sonoro satisfactorio**. No se publicará como sustituto ni se fusionará a main por el mero hecho de compilar.

## Diferencias identificadas comparando código fuente

| Área | HTML original | Preview Rust | Siguiente trabajo |
|---|---|---|---|
| HUD | Retrato circular, marcos elaborados, nombre, barras, iconos y fuente RetroPx (HTML líneas ~1083–1113) | Panel rectangular con fuente genérica | Reconstruir composición exacta |
| Control táctil | Botones con iconos, degradados y brillos, joystick alternativo, escala y posiciones editables (HTML ~1–33, ~405–426) | Círculos y texto | Portar recursos, tamaños y editor |
| Bosque Nocturno | Paralaje + bruma + luz, varios pases (HTML ~867–877) | Fondo y capas simplificados | Igualar orden de pases, niebla, luz y color |
| Bosque Oscuro | Lluvia, salpicaduras y relámpagos (HTML ~1056–1079) | Lluvia aproximada | Portar partículas e iluminación temporal |
| Cámara | Seguimiento vertical amortiguado y desplazamientos (HTML ~1076–1079) | Vertical fija | Paridad de cámara y escala |
| Entidades | Animaciones, telegráficos, destellos, transformaciones (HTML ~969–992 y ~1116–1187) | PNG originales pero sin la mayoría de FX | Portar efectos por pose y tipo |
| Música | MP3 para menú/Bosque Oscuro/jefe **y música WebAudio procedimental en Bosque Nocturno** (HTML ~359–365, ~594–620, ~1188–1194) | OGG en bucle, incluso un archivo nocturno que el HTML no utiliza | Portar síntesis y secuencias originales, presets y transición |
| SFX | Síntesis distinta por evento, rampas de frecuencia y ganancias exponenciales (HTML ~365) | Tonos aproximados | Reproducir formas de onda y envolventes originales |
| Mezcla | Volumen separado de música/SFX y tres estilos musicales (HTML ~393–425) | Audio activado/desactivado | Mezclador y presets |

## Primer pase de reconstrucción — implementado en la rama

- `scenery.rs`: añade `zSky / zRidge / zForest / zCloud` exportados desde el HTML, bruma, haces de luz y luciérnagas. Los dos mapas todavía requieren ajustes de pases y color.
- `ui.rs`: menú inicial con protagonista, selector de tres personajes separado, HUD retrato/barras/monedas, controles metálicos con pictogramas originales. La fuente y medidas exactas siguen en revisión.
- `render.rs` y `main.rs`: cámara vertical amortiguada independiente del HUD, con seguimiento al jugador.
- `scripts/export-sprites.mjs`: extrae de HTML cielo, montañas, nubes, retratos e iconos sin redibujarlos.
- `rust-screenshots.yml`: genera cuatro capturas nativas para QA, además de las tres del HTML.

**Estado:** avance importante, pero no paridad completa. Falta validar capturas, corregir diferencias, completar FX/lluvia/menús de opciones, tipografía, joystick y audio.

## Criterios de calidad

1. No añadir un estilo nuevo ni reinterpretaciones arbitrarias: el HTML es la referencia.
2. Capturar menú y ambos bosques con el script scripts/capture-reference.mjs, guardar las capturas del workflow visual-reference.yml y cotejarlas con pantallas equivalentes del APK, con resolución y cámara comparables.
3. Corregir primero HUD, controles, cámara y fondo; después efectos, personajes y jefes. Cada cambio tendrá una escena visual de comparación.
4. Portar la música procedimental y los efectos desde sus fórmulas originales, no inventar pistas nuevas.
5. Comprobar audio con auriculares y rendimiento en Android real; las pruebas automatizadas de Cargo no certifican la calidad visual.
6. Mantener el PR #1 como borrador y la rama main intacta hasta la aceptación audiovisual del usuario.

El objetivo es **paridad del juego original**, no simplemente un juego funcional con la misma temática.
## Visual vNext — segunda pasada (11 noviembre 2026)

Cambios **implementados en el código** sobre la rama Rust:

1. **Animación original:** el exportador ahora extrae caminar, idle (4 poses), salto, caída, casteo y ataque por clase; OGRE sprites de golpe; anclas exactas Canvas guardadas en \`sprite_meta.csv\`. \`assets.rs\` y \`render.rs\` usan esos metadatos.
2. **FX a 60 Hz:** \`effects.rs\` gestiona chispas, números flotantes, daño, impactos, nivel, dash, escudo, anillos y rayos. Tiene RNG visual independiente y pool de partículas limitado a 240. Se verificó compilación y pruebas en CI; \`cargo test -p retro-realm --bin retro-realm\` comprueba aislamiento del gameplay y límites.
3. **Enemigos y proyectiles:** squish/stretch de slimes, fotogramas de ataque de ogros, casteos/avisos enemigos y estela de proyectiles.
4. **Jefes:** barra de 212 unidades con divisiones, coronación y calavera del original; señales de golpes y efectos del Rey.
5. **Atmósfera:** cámara horizontal suavizada además de la vertical; lluvia de 130 trazos, salpicaduras y relámpagos, viñeta posterior al paisaje y anterior al HUD.
6. **Inicio:** botón JUGAR con degradado de acero, héroe completo en su miniatura, versión original en la esquina y créditos del juego en una ventana funcional.
7. **QA:** workflow \`rust-screenshots.yml\` recompila el render al cambiar arte/código y permite compararlo con \`visual-reference.yml\`. APK con \`version_code=6\` y package preview aislado.

### Pendientes antes de declarar paridad

- Capturas comparables de combate real, jefe activo, daño, poderes, pausa, selección y créditos. En especial comprobar el orden visual y anclas en dispositivos Android reales.
- UI avanzada de opciones/controles y joystick configurable del HTML, precisión del HUD, efectos finos de entidades, transición de menús y lectura de textos.
- Música procedimental WebAudio y SFX originales **no se han portado** aún, por decisión de priorizar visual.
- Rendimiento/temperatura/latencia de controles en un teléfono real.

**No afirmar 1:1, no mezclar a main.** Una build o una captura de CI sólo prueban que se genera o funciona bajo las condiciones del runner.
