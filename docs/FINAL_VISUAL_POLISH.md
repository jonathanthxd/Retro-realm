# Retro Realm — Final Visual Polish v8

Esta es la última pasada de **presentación visual** en la rama `refactor/rust-macroquad-migration` antes de priorizar el trabajo técnico. El proyecto original `Retro Realm.html` permanece intacto en `main`.

## Correcciones implementadas

- **Nombre del jugador nítido:** `ui_font_atlas_hi.png` generado directamente desde RetroPx a 48 px, anchuras de cada carácter medidas por Chromium en `ui_font_advance.csv`, ajuste a píxeles del framebuffer, contorno legible y tamaño de etiqueta equivalente al original. Ya no se renderiza el nombre antiguo dentro del mundo a 180 px cuando está activa la UI nativa.
- **HUD, controles y menús:** siguen componiéndose a resolución CSS de pantalla a partir de las capturas exactas del DOM; valores dinámicos y etiquetas usan el atlas de alta resolución. Se mantienen las pantallas de inicio, selección, pausa y créditos.
- **Antorchas:** geometría y colores de madera, soporte metálico y llama reconstruidos a partir de `zProps()` original, flicker y chispas de trayectoria determinista, bloom del `createRadialGradient` original exportado a 3× y compuesto después del mundo y antes del HUD.
- **Portales:** arco y pilares de piedra con juntas, pedestal, gema, partículas y brillo cian del `zPortal()` original; textura de luz radial a resolución nativa.
- **Personajes:** posiciones del sprite redondeadas a píxeles, animaciones originales previamente extraídas, etiquetas de nombre separadas del render de 180 px.
- **Compatibilidad:** el escenario sigue a 180 px y la simulación `realm-core` no se modifica. Versión Android preview `version_code=8`, package `com.pixelquest.game.preview`.

## Evidencias CI — commit `33ffcadbc8aba31e7f493af784192b74819710a4`

- [Pruebas Rust](https://github.com/jonathanthxd/Retro-realm/actions/runs/43140989446): correcto.
- [Capturas de seis estados nativos](https://github.com/jonathanthxd/Retro-realm/actions/runs/43140989404): correcto.
- [APK Android experimental](https://github.com/jonathanthxd/Retro-realm/actions/runs/43140989497): correcto.

## Estado de aceptación

El código y las capturas de escritorio se han verificado. **Falta validar visualmente y táctilmente en un dispositivo Android real**: nitidez de letras/nombre, brillo de antorchas en ambas áreas, menú, pausa, control y portal al final de los niveles. No se afirma certificación 1:1 de cada estado, ni fidelidad auditiva; el siguiente bloque técnico abordará comportamiento/controles/audio después de recibir los resultados de esta prueba final. El PR debe mantenerse como borrador y nunca fusionar a `main` sin aceptación.
