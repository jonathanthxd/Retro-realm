# Retro Realm — Final Migration Update
**Especificación de cierre Rust + Macroquad**  
**Estado:** definición aprobable, NO implementación.  
**Base auditada:** rama `refactor/rust-macroquad-migration`, después de Final Visual Polish v8.  
**Fuente funcional de verdad:** `Retro Realm.html` en `main`.  
**Regla:** no fusionar PR #1 ni reemplazar el APK original hasta pasar todos los criterios P0 y la aceptación en Android real.

## Resumen

Esta fase no es un cuarto rediseño visual. Resuelve bloqueantes de la *experiencia nativa*: adaptación a proporciones y DPI, controles de tamaño adecuado y realmente editables, ajustes funcionales, nombre editable con teclado Android, pantalla de derrota nítida, persistencia, audio original y validación completa de jugabilidad y ciclo de vida. La versión visual v8 será la base estética.

### Diagnóstico confirmado en el código actual

1. `ui_native::css_width()` supone una altura lógica de 540 y mínimo de 960; el menú usa capturas DOM **fijas 960×540** y las centra. El HUD y los controles usan coordenadas exportadas de esa pantalla con anclajes puntuales. Esto no equivale a un layout responsive para 18:9, 20:9, recortes y ajustes del usuario. La impresión de HUD «4:3» en un teléfono debe medirse; no afirmar un formato intrínseco 4:3 sin una captura con geometría.
2. `main.rs` crea un mundo `virtual_width=round(180 * screen_width/screen_height)` limitado a 320..480 y luego estira la textura a todo el framebuffer. Fuera del rango, o con recortes, puede deformar el mundo. `controls.rs` usa círculos táctiles de coordenadas codificadas y `point * 3` en algunos flujos; el layout mostrado y el hitbox no comparten una única fuente de verdad.
3. `ui_native.rs` usa `ui_pause_panel.png` como pantalla visual, pero al dibujarla termina temprano; no es una UI interactiva. `main.rs` conecta Música y Sonidos al mismo `audio_enabled`; no hay mezclador, selección de estilos, arrastre y restablecimiento de controles.
4. El nombre se puede editar con `get_char_pressed()` en PC, pero **no invoca el IME Android**. En el HTML `#nm` es un `input maxlength="14"` que guarda `pq_name` y usa «Jugador» si queda vacío.
5. `render.rs` dibuja `HAS CAIDO` dentro del framebuffer de **180 px** y lo amplía a pantalla. El HTML original dice «HAS CAÍDO», muestra «Nivel n · Oro n» y «Toca para reiniciar», además de un overlay oscuro.
6. El HTML guarda `pq_name`, `pq_lv` (**récord de nivel más alto**, NO nivel activo de sesión), `pq_cls`, `pq_mv`, `pq_sv`, `pq_ms`, `pq_ctl` y `rr_sz`. Rust todavía no los persiste. El código original ofrece escalas individuales **40–220 %** para controles y botones/joystick alternativos.
7. Rust reproduce pistas OGG y SFX aproximados. En el HTML hay efectos WebAudio con sobres y música nocturna secuenciada, tres estilos musicales (Épica, Folk, Nocturna), volumen separado de música/SFX. No está certificada paridad auditiva.
8. Cargo CI y la generación del APK prueban construcción, no jugabilidad, alcance táctil, FPS ni ciclo de vida Android en un teléfono físico.

## 1. Pantalla, densidad, proporciones y safe areas — P0

**Nuevo módulo `DisplayMetrics + LayoutEngine + WorldTransform`**, utilizado por renderizador, UI y entrada táctil.

* Entrada: píxeles reales del framebuffer; ancho/alto lógico reportados por Macroquad, DPI/densidad Android, orientación, área visible, recortes, insets de barras/gestos/teclado, preferencia de escala UI y control.
* Macroquad expone `screen_width`, `screen_height` y `screen_dpi_scale`, pero para áreas seguras y teclado se debe añadir la información Android nativa cuando Macroquad no la provea.
* Separar espacios: `PhysicalPixels`, `LogicalDp`, `SafeViewport`, `WorldViewport`, `UiRect`, `TouchHitRect`. Un solo transform compartido para dibujar y tocar; eliminar conversiones independientes `*3`, posiciones absolutos `960`, anclas ad hoc.
* Mundo: conservar el estilo pixel-art y el framebuffer lógico de altura 180 cuando sea posible. Ajustar **ancho visible** al aspecto de la zona de juego, sin estirar X/Y con escalas distintas. Para casos extremos decidir de forma explícita entre vista ampliada y barras discretas; jamás deformar el sprite.
* HUD: reconstruir la **disposición** desde recursos originales en elementos separados y anclados a la zona segura, no usar una sola captura fija estirada. Grupo superior izquierdo (retrato/HP/MP/nombre), indicadores de progreso y monedas en su zona correspondiente, pausa en zona segura, controles abajo a izquierda/derecha y menús centrados dentro de área visible.
* En pantalla estrecha: layout compacto o reagrupación conservando identidad; no superponer HUD, recortes, textos o botones. En formato ultrapanorámico: aprovechar ancho adicional sin estirar arte. La UI no se debe deformar en tablets.
* Márgenes Android: consultar insets de barra, navegación por gestos y notch/cámara; fondo puede ir hasta el borde, controles e información crítica deben quedar en área segura. Recalcular con rotación, tamaño y aparición de teclado.
* Pixel crisp: pixel art nearest con coordenadas enteras; fuente RetroPx high-res nítida en capa nativa; sin blur ni alias involuntario al cambiar DPI.
* La escala del HUD y la escala de botones deben tener ajustes separados; aplicar perfil de tamaño automático inicial basado en zona útil/densidad, no solamente en resolución en px.

**Pruebas:** 4:3, 16:9, 18:9, 19.5:9, 20:9, 21:9, diferentes densidades, tablets y landscape con notch izquierdo/derecho. No imagen cortada, deformada, tapada ni pixelada de forma no intencional. En cada viewport, comparar posiciones de visual y hitbox.

## 2. Controles táctiles auténticos y personalizables — P0

* Dos modos plenamente funcionales: **Botones** izquierda/derecha y **Joystick** analógico/digital según el original. Reproducir joystick con captura de dedo, eje horizontal y gesto vertical para saltar; soltar al cancelar/interrumpir toque.
* Acciones: Izquierda, Derecha, Salto, Ataque, Escudo, Habilidades 1/2/3, Dash, Interactuar, Pausa. Mantener teclas PC. Soporte multitáctil: moverse + saltar + atacar/castear simultáneamente, sin perder el primer contacto.
* Objetivo táctil mínimo **48 dp**, aunque el icono visible sea más pequeño; controles principales de inicio propuestos **64–84 dp** y secundarios **56–72 dp**, ajustables y sujetos a pruebas en teléfonos reales. Nunca reducirlos en función de más píxeles físicos. Mantener escala por botón entre **40 % y 220 %** como el HTML, sin permitir que el área táctil caiga por debajo del mínimo seguro.
* Editor de controles: modo «Mover controles» con bordes de selección, arrastre en tiempo real, tamaño individual, restablecimiento por botón y global, «Listo» para guardar; impedir arrastre fuera de la safe area y solapamientos que vuelvan inaccesibles las acciones. Incluir opacidad opcional para legibilidad sin cambiar hitbox.
* Guardar coordenadas relativas a los bordes seguros y/o posiciones normalizadas, **nunca solo offsets de una resolución**. Reposicionar inteligentemente cuando cambien proporción o DPI, conservando preferencias cuando se pueda.
* Separar visual/entrada del diseño original: ambos deben consultar los mismos rectángulos transformados, tamaño y z-order. Modal y pantalla de derrota capturan pulsaciones sin filtrar acciones a la partida.
* Opciones por defecto «Automático», «Botones» y «Joystick» con vista previa; restauración sencilla si una personalización impide alcanzar un botón.

**Pruebas:** pulsar simultáneamente 2, 3 y 4 acciones; arrastrar, guardar, forzar cierre y reiniciar; cambiar modo; volver a valores iniciales; cambiar DPI/ratio; comprobar cada botón en teléfono real. 0 pulsaciones fantasma al abrir ajustes/pausa.

## 3. Ajustes completos, interactivos y persistentes — P0

No se aceptan pantallas PNG que parezcan botones pero no respondan. Usar sprites/skin del original como decoraciones + widgets nativos interactivos transparentes o renderizados dinámicamente.

* Deslizadores **Música 0–100** y **Sonidos 0–100** completamente independientes y efectivos durante la reproducción, con lectura visible de valores. Botón maestro mute separado, si se conserva.
* Tres estilos **Épica, Folk y Nocturna**, con estado visual seleccionado y cambio audible real sin duplicar pistas o quedarse sonando al salir.
* **Botones / Joystick**, **Mover controles**, **Restablecer**, y selector de escala individual de botones con confirmación/cancelación.
* **Cambiar personaje**, **Menú principal**, **Continuar**: flujos reales, coherentes con el HTML; decidir explícitamente si abandonar partida descarta el intento. Android Atrás abre/cierra pausa con comportamiento predecible.
* Subpantalla de **Pantalla e interfaz** (mejora razonable al original): Escala HUD (Automática/Pequeña/Media/Grande), Escala de texto (Auto y ajuste), Mostrar controles (según modo), Ajustar áreas seguras (solo diagnóstico si Android no reporta bien insets). No convertir cada campo en una nueva dependencia externa.
* Una configuración central de estado, enlazada a audio, render, input y persistencia: si un control está dibujado, tiene callback real; sus valores actuales permanecen visibles. Soporte de scroll en pantallas pequeñas con modal accesible.

**Pruebas:** cambiar cada opción durante partida y verificar el efecto inmediato, guardar, salir, relanzar y confirmar persistencia. Ninguna opción decorativa.

## 4. Nombre, teclado Android y perfil — P0

* Al tocar el campo «Jugador» en inicio, abrir editor verdadero y **teclado del sistema Android (IME)**. Campo nativo o puente fiable desde Rust, sincronizado con la textura/RetroPx.
* Límite de **14 caracteres** del HTML, saneamiento de caracteres de control, soporte de tildes/ñ/Unicode introducido mediante IME, cursor, selección, borrar, pegar, «Aceptar», «Cancelar» y botón Atrás para cerrar teclado sin iniciar el juego.
* Recortar espacios extremos y, si queda vacío, restaurar **«Jugador»**. Actualizar inmediatamente menú, nombre sobre jugador y HUD. Persistir entre reinicios.
* La pulsación del campo debe usar el mismo layout que el dibujo (incluidas notch/safe area). El teclado no tapa texto activo ni «Aceptar».
* Guardar **nombre, clase elegida y nivel máximo** como perfil, separado del nivel actual de sesión. El nivel máximo debe actualizarse solo si se supera. La partida original usa nivel 1 en cada inicio; no cambiar economía/dificultad por error al cargar el récord.

**Prueba:** editar con teclado físico y Android; probar «Ñandú», nombres largos, borrar, confirmar/cancelar, reinstalar como upgrade, cerrar app y reabrir.

## 5. Overlay de derrota, mensajes y UI contextual — P0

* Sacar de `render.rs` todas las interfaces que deban leerse nítidas tras el escalado retro: **derrota**, mensajes de jefe, avisos, indicaciones de cofres/portal, contador importante y cualquier texto que hoy se amplíe desde 180 px sin intención artística.
* La derrota debe conservar el contenido del HTML: «**HAS CAÍDO**», «**Nivel X · Oro Y**», «**Toca para reiniciar**» con fondo oscurecido; versión nativa a la misma resolución de la UI, con sombras y contornos legibles, tamaño adaptado a DPI/ratio.
* Reiniciar con toque (superficie explícita), sin iniciar ataques por debajo del overlay. Mantener tecla R como atajo PC.
* Mensajes de estado: nivel, jefe derrotado, daño, obtención de oro/gemas, «Próximamente» y condiciones de apertura de portales. Animaciones suaves, coherentes con estilo RetroPx y sin confundir mensajes con botones.
* Pausa y créditos también deben adaptar textos y scroll a safe areas. Evitar doble dibujo de HUD o texto entre capas.

**Pruebas:** muerte en ambos mapas, toque reiniciar, salir a menú, nombres largos y layout compacto/ultrawide. Verificar screenshot dorado para todos los overlays.

## 6. Guardado y compatibilidad de datos — P0

* Modelo versionado de preferencias/usuario (`schema_version`) en almacenamiento privado Android. Guardar atómicamente (archivo temporal + reemplazo seguro), validar valores, recuperarse de archivo malformado, defaults sensatos; no guardar ni exponer datos privados fuera del sandbox.
* Campos: `player_name`, `best_level`, `selected_class`, `music_volume`, `sfx_volume`, `music_style`, `control_mode`, `control_size` por ID, `control_position` relativa a safe area, `hud_scale`, `text_scale` y opciones básicas de pantalla.
* Equivalencia con claves HTML: `pq_name`, `pq_lv`, `pq_cls`, `pq_mv`, `pq_sv`, `pq_ms`, `pq_ctl`, `rr_sz`. **No suponer** acceso directo a `localStorage` de otra aplicación/UID. Una migración explícita/importación solo si es técnica y legalmente viable; jamás sobrescribir la base original automáticamente.
* Guardar en confirmación y al abandonar pantallas; debouncing para arrastres, flush seguro al entrar en segundo plano; verificar que la partida en curso no se modifica cuando solo se guardan preferencias.
* Asegurar compatibilidad de versiones nuevas del JSON. Una actualización APK con la misma identidad y firma debe mantener preferencias; una reinstalación limpia no está obligada a recuperarlas.

**Pruebas:** relanzar, upgrade, valores límite, JSON corrupto, cambios de clase y nivel récord; resultado consistente sin crash.

## 7. Audio de paridad + mezclador — P0

* Comparar pista por pista el HTML y Rust. Música MP3 propia puede conservarse mediante OGG equivalente donde proceda; no asumir que el archivo `bosque_nocturno.ogg` replica el **secuenciador WebAudio** original.
* Portar música nocturna/estilos musicales con sus patrones, formas de onda, notas, tiempos y envolventes originales, o documentar equivalencia auditiva demostrada. La selección Épica/Folk/Nocturna ha de alterar el audio realmente.
* SFX del HTML específicos por evento (salto, slash, golpe, habilidades por clase, dash, escudo, daño, muerte, nivel, cofre, portal, jefe) en vez de reutilizar tonos aproximados genéricos.
* Control independiente de volumen de música y SFX y mute general; evitar sonido simultáneo viejo/nuevo durante transiciones. Audio focus del sistema: pausa/duck al recibir llamada, app background, volver; no reproducir en segundo plano involuntariamente.
* Probar ausencia de clipping, latencia y transiciones (menú → bosque → jefe → segundo bosque → pausa → menú). Comparación con auriculares.

**Pruebas:** cada uno de los tres estilos, volúmenes 0/50/100, mute, background/resume, 11+ cues y jefes sin dobles sonidos.

## 8. Paridad jugable de inicio a fin — P0

* Automáticas: semillas/control de RNG, huellas de inputs a 60 Hz, estado por tick (posición, velocidad, vida/maná, cooldowns, daño, drops, enemigos, bosses, portal) y comparaciones con HTML donde se puedan capturar de forma reproducible. Distinguir variación RNG esperable de diferencias en reglas.
* Tres clases y todas las habilidades; movimiento, salto/doble salto, dash, escudo, combos, tiempo de invulnerabilidad, colisiones, daño a enemigos/jugador, estados quemadura/veneno/lentitud.
* Progresión completa **20 enemigos → Ogro Pantanoso y segunda fase → portal** y **30 enemigos → Rey de las Tumbas → portal de «Próximamente»**, con cofres, economía, niveles, recompensa de jefes y ida/vuelta si está permitida.
* Derrota en ambas áreas, reinicio sin duplicar premios o audio, pausa que detenga lógica y temporizadores, cambio de personaje sin mezcla de estado.
* Añadir pruebas automatizadas de lógica más guiones manuales con resultados esperados. Los tests de CI existentes son punto de partida, no aceptación integral.

**Pruebas:** mínimo una partida completa por cada una de las tres clases; secuencias de combate con jefes y camino de portales.

## 9. Ciclo de vida Android, rendimiento y accesibilidad — P0

* Arranque frío, volver desde Home/Recientes, bloquear/desbloquear, Android Atrás, rotación no deseada, cambio de enfoque, interrupción de audio y destrucción del proceso sin comportamientos corruptos. La orientación sigue **landscape**.
* Evitar pérdida de eventos táctiles por `TouchPhase::Cancelled`, reinicializar estados al volver para que ningún botón quede presionado. Pausa al pasar a background; reanudar solo en acción esperada.
* Mantener tick fijo 60 Hz independientemente de 30/60/90/120 Hz de pantalla. Instrumentar FPS, p95/p99 de frame time, carga y memoria sin usar estos datos para modificar gameplay.
* Metas iniciales de aceptación (a medir, no promesas): **60 FPS objetivo** en gama media, **30 FPS sostenidos mínimo** en perfil de gama baja definido, sin cierres por memoria, congelamientos repetidos ni audio defectuoso. Si la GPU no alcanza, permitir reducir solo resolución/efectos del mundo manteniendo UI y tamaño táctil nativos.
* Considerar fuentes de hasta 48 dp como objetivo táctil y contraste de texto; colores legibles y alta precisión de controles. No imponer una sensibilidad fija para todos los dispositivos.
* Arte/partículas y atlas: límites de memoria, evitar crear recursos nuevos cada fotograma, no duplicar texturas gigantes ni halos que tapen sprites; medir sobre dispositivo real.

**Pruebas:** al menos un Android gama baja y otro gama media cuando se pueda; tamaños/densidades/resoluciones variados. Capturas de pantalla + logs + FPS/frametime.

## 10. Packaging, firma y cierre del PR — P0

* Mantener `com.pixelquest.game.preview` mientras se prueba. La elección del package final depende de si se desea actualización sobre la app original o una instalación separada.
* Para upgrade in-place hace falta **mismo applicationId y clave de firma compatible**; si no, ofrecer una instalación paralela con mensaje claro (sin prometer conservación automática de datos del WebView).
* Comprobar nombre/ícono definitivos, permisos estrictamente necesarios, Android manifest (landscape, fullscreen, insets/cutouts), compatibilidad ABI, versión y código de versión incrementales, firma de producción, APK reproducible por CI.
* Revisar derechos/atribuciones de código, gráficos y música originales antes de redistribución pública, sin presumir licencia inexistente.
* Entregar **Release Candidate APK** firmada + SHA-256 + changelog + matriz QA + lista de limitaciones remanentes. Una vez todos los gates P0 estén aprobados por pruebas Android, pasar PR #1 de borrador a listo para revisión; merge **solo con permiso explícito** del usuario.

## Secuencia obligatoria de implementación

| Bloque | Dependencias | Entrega y gate |
|---|---|---|
| A. Transformaciones de pantalla | Ninguna | Layout responsive, aspect ratio, DPI, insets y mapeo visual/táctil compartido |
| B. Control editor + joystick | A | Controles grandes, multitouch, editor 40–220 %, guardado y restablecer |
| C. UI interactiva + nombre + derrota | A, B | Pausa completa, teclado Android 14 caracteres, overlay de derrota nítido |
| D. Persistencia + audio original | B, C | Preferencias y nivel récord; volúmenes separados, 3 estilos, SFX y ciclo de vida audio |
| E. QA jugabilidad + Android | A–D | Partidas completas, snapshots responsivos, FPS/touch/lifecycle y paridad verificada |
| F. Release Candidate | E | APK final de prueba, firma/identidad resueltas, documentación y PR en borrador hasta aprobación |

## Matriz de aceptación final

Todos los ítems son **bloqueantes P0**:

- [ ] Layout responsive real en 4:3, 16:9, 18:9, 19.5:9, 20:9, 21:9, tablets y notch en ambos laterales sin estiramiento deformante.
- [ ] HUD, menú, selección, pausa, créditos, derrota y overlays completamente nítidos y compatibles con insets/DPI.
- [ ] HUD y mundo alineados bajo un solo transform; los rectángulos táctiles coinciden con los elementos visuales.
- [ ] Botones cómodos de serie (hit targets ≥48 dp), multitáctil correcto, joystick real, editor completo y restauración por botón/global.
- [ ] Todos los ajustes visibles funcionales, con feedback inmediato y persistencia.
- [ ] Editor de nombre táctil abre Android IME y actualiza HUD/perfil, soporta 14 caracteres y nombre por defecto.
- [ ] Pantalla «HAS CAÍDO» con nivel/oro, sin blur, y reinicio táctil limpio.
- [ ] Nombre, mejor nivel, clase y preferencias persistentes después de relanzar/upgrade con datos válidos.
- [ ] Música/SFX originales, tres estilos y volúmenes separados, sin audio fantasma ni clipping.
- [ ] Dos bosques, tres clases, dos jefes y todos los portales/recompensas completados y comparados con HTML.
- [ ] Android Atrás, suspensión, cambio de foco, audio focus, safe areas, teclado y retorno sin bloqueos.
- [ ] Objetivos de rendimiento verificados y pruebas manuales sobre dispositivos reales, además de CI verde.
- [ ] APK/identidad/firma y derechos aclarados; PR permanece borrador hasta aprobación.

**No es un objetivo de esta fase:** añadir nuevos mapas, habilidades, tiendas, multijugador, cuentas online, telemetría comercial ni rediseñar la identidad visual. Las mejoras al original serán únicamente usabilidad, lectura, accesibilidad y estabilidad compatibles con la jugabilidad actual.

## Decisiones de producto ya adoptadas por defecto

1. Se conserva identidad visual de Retro Realm. UI moderna significa **adaptativa y nítida**, no temática nueva.
2. Paisaje pixel art a resolución lógica retro y HUD/overlays/touch a resolución de pantalla.
3. Los controles parten de perfil automático según densidad/safe area, siempre editables.
4. Se conserva el comportamiento de nombre por defecto «Jugador», máximo 14 caracteres y nivel récord.
5. La actualización no sustituye `main` ni borra la instalación original durante el QA.

## Decisiones finales pendientes antes de publicar

- Elegir si la aplicación nativa **reemplaza** a la original o convive con ella; comprobar acceso a firma y package de la original.
- Decidir cómo se tratarán **datos existentes de la app HTML** (ninguna importación / importación opt-in técnicamente viable), sin prometer acceso entre sandboxes.
- Validar físicamente la experiencia (incluido tamaño óptimo de botones en el teléfono de prueba) y autorizar el merge.

**Cierre:** solo puede llamarse «migración Rust terminada» tras pasar toda la matriz de aceptación, no por el número de versión del APK ni por capturas CI.
