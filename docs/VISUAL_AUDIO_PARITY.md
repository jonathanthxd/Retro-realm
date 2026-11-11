# Retro Realm — fidelidad visual y auditiva

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

## Criterios de calidad

1. No añadir un estilo nuevo ni reinterpretaciones arbitrarias: el HTML es la referencia.
2. Capturar menú y ambos bosques con el script scripts/capture-reference.mjs, guardar las capturas del workflow visual-reference.yml y cotejarlas con pantallas equivalentes del APK, con resolución y cámara comparables.
3. Corregir primero HUD, controles, cámara y fondo; después efectos, personajes y jefes. Cada cambio tendrá una escena visual de comparación.
4. Portar la música procedimental y los efectos desde sus fórmulas originales, no inventar pistas nuevas.
5. Comprobar audio con auriculares y rendimiento en Android real; las pruebas automatizadas de Cargo no certifican la calidad visual.
6. Mantener el PR #1 como borrador y la rama main intacta hasta la aceptación audiovisual del usuario.

El objetivo es **paridad del juego original**, no simplemente un juego funcional con la misma temática.