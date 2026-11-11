# Retro Realm

Retro Realm es un juego de acción y plataformas 2D para móvil, con tres personajes y dos bosques jugables.

## Migración a Rust y Macroquad

La reconstrucción **está en desarrollo** en la rama
[\`refactor/rust-macroquad-migration\`](https://github.com/jonathanthxd/Retro-realm/tree/refactor/rust-macroquad-migration).
El objetivo es preservar el juego original sin convertirlo en otro proyecto.

- \`Retro Realm.html\`: implementación original intacta, ejecutable en navegador y empaquetable con el workflow Android heredado.
- \`crates/realm-core\`: simulación del juego en Rust sin dependencias gráficas; pruebas de las reglas de combate y progresión.
- \`apps/retro-realm\`: aplicación nativa Rust + Macroquad, aún en validación.
- \`docs/MIGRATION.md\`: contrato de paridad y limitaciones actuales.

### Ejecutar la versión Rust en PC

Instala [Rust](https://rustup.rs/) y ejecuta desde la raíz:

\`\`\`powershell
cargo run -p retro-realm
cargo test -p realm-core
\`\`\`

La versión Android nativa se empaquetará con \`cargo-quad-apk\` una vez probada la compatibilidad del renderizador y del audio. La versión WebView original **no ha sido sustituida**.

> Licencia: el repositorio original no declara una licencia de redistribución. No se concede ninguna licencia nueva sobre el código ni sobre los recursos del autor original mediante esta migración.
