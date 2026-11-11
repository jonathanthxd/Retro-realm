# Migración Retro Realm a Rust + Macroquad — paridad y aceptación

## Objetivo

Reimplementar **el mismo juego** en Rust nativo, sin WebView ni motor externo, manteniendo dos mapas, tres clases, jefes, audio y arte original. La rama `main` **no debe reemplazarse** antes de superar las pruebas de paridad.

## Matriz de avance

| Área | Estado al 11 de noviembre de 2026 | Diferencias pendientes |
|---|---|---|
| Workspace `realm-core` + Macroquad | Implementado; compila en GitHub Actions | Revisar arquitectura a medida que se amplía |
| Física a tick fijo (60 Hz) | Implementada y con pruebas | Comparación cuadro a cuadro con navegador a 60/120 Hz |
| Maga Lunar, Cazadora y Mapache Cyber | Tres clases y tres habilidades portadas | Animaciones/hitboxes y FX específicos por clase |
| Movimiento, salto doble, dash y escudo | Portados | Afinar sensibilidad y joystick virtual |
| Bosque Nocturno y Bosque Oscuro | Mapas, cuotas 20/30, cofres y portales | Paridad de obstáculos, posicionamiento y ambiente |
| Enemigos | Variantes, estadísticas originales, segunda fase de slimes/grandes y efectos de daño | Pulir IA individual y recompensas exactas |
| Jefe Ogro Pantanoso | Aparición y segunda fase dividida | Especiales, colisiones y partículas |
| Rey de las Tumbas | Máquina de estados combo/salto/dash/furia | Afinar tiempos y animaciones frente al HTML |
| Sprites, escena y decoración | PNGs originales, cielo dithered, montañas, nubes, vegetación, bruma, haces de luz y bosque por capas exportados del Canvas | Pases finales, FX por enemigo, iluminación y comparación visual |
| Paralaje, cámara y Bosque Oscuro | Capas originales, dos bosques, lluvia y seguimiento vertical amortiguado | Ciclo visual exacto, relámpagos y salpicaduras, pruebas de encuadre |
| Audio | OGG desde MP3 propios, efectos sintetizados y mute | Comparación auditiva, volumen independiente de música/FX |
| UI y controles táctiles | Menú principal más fiel, selección separada, retrato HUD, barras y pictogramas originales, nuevos tamaños de controles | Tipografía RetroPx, escala y posiciones configurables, joystick, créditos, pausa completa |
| Perfil/ajustes y guardado | Nombre editable durante el menú por teclado PC, sin persistencia | Teclado virtual Android, mejor nivel, preferencias y controles persistentes |
| Android APK | Compilación nativa confirmada en CI | Arranque/emulador/controles/audio/FPS en dispositivos reales |

## Reglas de aprobación

1. No hacer merge del [PR #1](https://github.com/jonathanthxd/Retro-realm/pull/1) mientras no exista una comprobación de paridad.
2. Deben funcionar ambas rutas completas: 20 enemigos → ogro → segunda fase → portal; 30 enemigos → Rey → portal «Próximamente».
3. El juego debe permanecer a velocidad lógica constante en pantallas de 60/90/120 Hz.
4. Probar las tres clases con cada habilidad, doble salto, dash, escudo, todos los tipos de enemigo y cofres.
5. Reproducir las reglas de lentitud, quemadura, veneno, XP/monedas, gemas y jefes sin saltos de estado.
6. Probar en al menos un teléfono Android real de gama baja y uno de gama media, incluyendo música/SFX, suspensión, rotación e instalación.
7. No usar el identificador de la app original ni intentar importar datos privados del WebView sin una migración de datos comprobada y autorizada.

## Pruebas automáticas

```bash
cargo check --workspace --all-targets
cargo test -p realm-core
```

La compilación CI y un APK generado demuestran que el proyecto **construye**, no certifican su jugabilidad ni su fidelidad 1:1. El sistema sigue en **preview**.

El HTML original incluía datos `localStorage` (`pq_name`, `pq_lv`, `pq_cls`, `pq_mv`, `pq_sv`, `pq_ms`, `pq_ctl`, `rr_sz`). El port no debe afirmar que conserva esos datos, ni sustituir su almacenamiento antes de implementar y validar la importación.

Este fork no declara una licencia de distribución independiente en el upstream. Confirmar derechos y atribuciones antes de publicar APKs de forma externa al repositorio.

## Actualización visual vNext — primer pase, todavía no aprobado

Esta rama ahora incluye `scenery.rs` (capas finales de cielo/montañas/nubes), `ui.rs` (composición y botones), exportador de retratos e iconos, selector separado y cámara Y amortiguada. Las pruebas Rust **no certifican** calidad visual. Las capturas del HTML y del renderer nativo se producen en `visual-reference.yml` y `rust-screenshots.yml` para inspección lado a lado. La versión APK preview permanece experimental.
