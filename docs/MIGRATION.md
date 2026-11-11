# Retro Realm — contrato de migración 1:1

## Alcance inmutable

- Clases: Maga Lunar, Cazadora, Mapache Cyber.
- Bosques: Bosque Nocturno (20 enemigos) y Bosque Oscuro (30 enemigos).
- Jefes finales: Ogro Pantanoso y Rey de las Tumbas, respectivamente.
- Progresión: experiencia, niveles, HP/MP, monedas, gemas, cofres, portales.
- Mecánicas: movimiento, salto/doble salto, melee, tres habilidades, dash y escudo.
- Estilo: pixel art, HUD táctil, atmósfera de cada bosque y las pistas musicales originales.
- El tercer mapa permanece «Próximamente». No se añade contenido inventado.

## Estado verificable

| Sistema | Estado | Advertencias |
|---|---|---|
| HTML original y compilación WebView | Conservados | No se reemplazan durante la migración |
| Núcleo Rust sin gráficos | Implementación inicial | Comparar parámetros contra comportamiento JS completo |
| Tres clases y habilidades | Funcionales en simulación | Pueden faltar efectos/animaciones específicos |
| Dos mapas, cuotas y portales | Funcionales en simulación | Falta la paridad visual |
| IA y jefes | Parcial | Falta portar todos los patrones del Rey de las Tumbas |
| Cofres y recompensas | Parcial | Falta comparación exacta de probabilidad/tiempos |
| Pixel art y sprites originales | Pendiente | Render nativo actual es provisional |
| Música MP3 original | Preservada en raíz | Backend Macroquad usa WAV/OGG; requiere conversión y prueba |
| Guardado antiguo localStorage | Pendiente | Requiere migración segura desde WebView |
| APK Android nativo | Pendiente de pruebas | No afirmar compatibilidad con Android 5.0 sin prueba real |

## Reglas de ingeniería

1. No hacer cambios en \`main\` ni borrar el archivo HTML.
2. Probar las fórmulas del núcleo con tests deterministas.
3. Usar tick fijo de 1/60 segundo, limitado por un acumulador para evitar que el juego acelere en pantallas de 120Hz.
4. Mantener rendering, entrada y audio fuera de \`realm-core\`.
5. No publicar/reemplazar el APK heredado hasta paridad 1:1 validada en móvil.
6. No asumir que una compilación CI equivale a una prueba in-game.

## Lista de diferencias conocidas

La versión Rust actual **no es aún una migración completa**: la lógica de ataque específico de varios enemigos, la lógica avanzada del Rey, todos los sprites exactos, los efectos del mapa, la pantalla de configuración y la persistencia requieren portado individual y pruebas comparativas.

La rama se mantiene separada para evitar que esta etapa de desarrollo afecte a la versión original.
