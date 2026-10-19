# HellerTimer

[![Made with Slint](https://img.shields.io/badge/Made_with-Slint-007bc7)](https://slint.dev)
[![Checks](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml/badge.svg)](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) · [Русский](README.ru.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [中文](README.zh.md) · [日本語](README.ja.md)

Un temporizador Pomodoro nativo escrito en **Rust + Slint**, inspirado en [Pomatez](https://github.com/zidoro/pomatez). Lleva el nombre de Joseph Heller, autor de *Catch-22*.

El temporizador y la interfaz de escritorio funcionan; la versión web es una demo, la de escritorio es la aplicación principal.

[Descargar versiones](https://github.com/hlophlopgaming/HellerTimer/releases) · [Sitio y demo web](https://hlophlopgaming.github.io/HellerTimer/) · [Compilaciones](https://github.com/hlophlopgaming/HellerTimer/actions)

<p align="center"><img src="site/screenshot.png" width="360" alt="HellerTimer en Linux"></p>
<p align="center"><img src="site/mini.png" width="260" alt="Ventana mini compacta con temporizador, pausa y expandir"></p>

## Funciones

- Enfoque, descansos cortos y largos; rondas configurables; cuatro descansos especiales programados.
- Modo estricto, inicio automático del enfoque, descansos opcionales a pantalla completa y notificaciones anticipadas extra.
- Plazos absolutos en `SystemTime`: las transiciones perdidas se reproducen tras dormir, anunciando solo la última.
- Listas de tareas con títulos, descripciones, completado, reordenación y movimiento entre listas. Intro guarda los cambios; las flechas reordenan y → mueve a la siguiente lista. Solo se pueden eliminar listas vacías.
- Interfaz en inglés y ruso (y siete idiomas más), Noto Sans con cirílico incluido, cinco temas (Light, Dark, Paper, Forest, Midnight) y los sonidos/grabaciones de voz originales de Pomatez.
- Un icono de reloj de arena original inspirado en *Catch-22*.
- Bandeja con progreso, ventana mini compacta, siempre visible, barra de título nativa/propia, cerrar/minimizar en la bandeja, instancia única y autoinicio.
- Verificación opcional de actualizaciones 30 segundos tras el inicio, con un límite de 5 segundos. No se necesita red para iniciar ni usar el temporizador.
- Almacenamiento JSON atómico y versionado con guardado diferido de 500 ms. Un archivo dañado se respalda como `data.json.bak`.
- Renderizado por software por defecto; FemtoVG opcional para comparaciones. Un único temporizador single-shot programa los despertares del núcleo; sin hilo de temporizador ni runtime asíncrono.

## Estado por plataforma

| Función | Linux | Windows | macOS | Web | Android |
|---|---|---|---|---|---|
| Temporizador, tareas, configuración, traducciones | verificado localmente | build CI | build CI | WASM verificado | SDK/dispositivo pendiente |
| Bandeja y progreso | requiere host StatusNotifier | implementado | implementado | — | notificación de cuenta atrás |
| Ventana mini / fijar | depende del compositor | implementado | implementado | en el lienzo | en el diseño |
| Atajos globales | X11 | implementado | implementado | — | — |
| Sonido / voz en inglés | `paplay` | WinMM nativo | `afplay` | campana tras interacción | pendiente |
| Notificaciones del sistema | `notify-send` | pendiente; oculto | `osascript` | permiso del navegador | puente Java |
| Autoinicio | implementado | implementado | implementado | — | — |

Las opciones no soportadas se ocultan en Ajustes. En Linux, el sonido y las notificaciones solo se ofrecen si las utilidades están instaladas; la bandeja requiere un watcher StatusNotifier activo. GNOME normalmente necesita la extensión AppIndicator. Los atajos globales no se registran bajo Wayland. «Siempre visible» está oculto allí.

Atajos de escritorio: **Ctrl+Alt+P** iniciar/pausar, **Ctrl+Alt+N** omitir, **Ctrl+Alt+M** ventana mini. Los atajos en conflicto o no soportados se registran y la app sigue funcionando.

## Instalación / ejecución

Descarga un paquete de [Releases](https://github.com/hlophlopgaming/HellerTimer/releases). La atribución «Made with Slint» y los avisos completos acompañan a los paquetes.

```
cargo run -p heller-desktop
cargo run -p heller-desktop -- --start
cargo run -p heller-desktop -- --mini
cargo build --release -p heller-desktop --features femtovg
```

Requiere Rust estable 1.92+ y las bibliotecas de plataforma para Slint/winit. Ubuntu:

```
sudo apt install libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libfontconfig1-dev libx11-xcb-dev
```

Utilidades opcionales de Linux: `pulseaudio-utils` (`paplay`, funciona también con el servidor de compatibilidad PulseAudio de PipeWire), `libnotify-bin`, `curl` y `busctl` de systemd. No hay dependencia de GTK.

CLI: `--start`, `--hidden`, `--mini`, `--renderer software|femtovg`, `--animation`, `--no-animation`, `--bench-first-frame`, `--help`. FemtoVG requiere la característica de compilación anterior. `--hidden` vuelve a mostrar una ventana si no hay bandeja. La instrumentación del primer fotograma solo informa si el renderer soporta el notificador de renderizado de Slint.

Un archivo `portable` junto al ejecutable selecciona un directorio `data/` vecino; si no, se usan los directorios de configuración de la plataforma. Los archivos portables de Windows incluyen este marcador. No hay importación desde Pomatez.

Los paquetes macOS sin firmar pueden requerir **Open Anyway** en Ajustes del sistema → Privacidad y seguridad. No se incluyen credenciales de firma de Apple. Los APK están firmados en debug hasta configurar un keystore de release.

## Web y Android

```
rustup target add wasm32-unknown-unknown
cargo check -p heller-web --target wasm32-unknown-unknown
cargo install trunk --locked
cd crates/heller-web
trunk serve
```

Las notificaciones del navegador requieren permiso y un origen seguro (HTTPS o localhost); el sonido requiere interacción del usuario. Las pestañas ocultas pueden retrasar notificaciones por el throttling del navegador; el tiempo restante se recalcula a partir de los plazos al volver. El tema se guarda localmente. Los ajustes oscuros/claros de la versión 1 migran a la versión 2 sin cambiar tareas ni configuración. El almacenamiento puede no estar disponible en sesiones privadas; los errores se registran. El manifiesto PWA permite instalación independiente; no hay caché sin conexión.

Android: se necesitan Java 17, Android SDK platform 35, NDK y Gradle 8.11.1. Instala `cargo-ndk`, añade los tres targets Rust de Android y sigue `.github/workflows/release.yml` para generar las bibliotecas JNI y ejecutar `gradle assembleDebug` en `crates/heller-android/android`. La denegación de alarma exacta cae a una inexacta; el permiso de notificaciones se pide en Android 13+. El receptor Java puede anunciar el próximo plazo sin el proceso de UI. **Continuar varias fases tras la muerte del proceso no está implementado:** reabre la app. Las builds de Android y el IME requieren prueba en dispositivo real.

## Verificación

```
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test -p heller-core
cargo check -p heller-web --target wasm32-unknown-unknown
cargo check -p heller-android --target aarch64-linux-android
python bench/bench.py --help
python -m unittest discover -s bench
```

Para ejercitar controles Slint reales y tamaños de ventana en una sesión gráfica:

```
cargo run -p heller-desktop --example ui-smoke
cargo run -p heller-desktop --example ui-smoke -- --start-mini
cargo run -p heller-desktop --example ui-smoke -- --frameless
```

Estos escenarios envían eventos de puntero, prueban cada entrada de la ventana mini y su restauración (incluidas ventanas ocultas), el traspaso del arrastre nativo y la liberación del puntero, controles del temporizador, tareas, configuración, todos los temas, ruso y persistencia. Usan una plataforma en memoria y nunca modifican tus datos. Las capturas RGBA y las dimensiones van a `$HELLER_SCREENSHOT_DIR` o a un directorio temporal `hellertimer-ui-smoke`. CI ejecuta los mismos escenarios bajo Xvfb.

El último comando de Android requiere SDK/JDK incluso para `cargo check`. Los trabajos largos de empaquetado y release se ejecutan en CI. Un push de tag `v*` compila los paquetes de release; el workflow publica de inmediato un release regular marcado como Latest y luego adjunta los paquetes. GitHub Pages se despliega desde `main` con Actions. No se requiere PR.

## Rendimiento

Consulta la [guía de benchmark](bench/README.md). Objetivos: ≤30 MiB de PSS visible, ≤20 MiB de PSS en bandeja, CPU oculta casi nula, ≤250 ms de inicio. Ambos renderizadores se comparan contra el árbol de procesos completo de Pomatez en la misma máquina.

<!-- bench:start -->
Medido en un Linux de escritorio frente a Pomatez 1.11.0 (árbol de procesos completo, PSS). Consulta la [guía de benchmark](bench/README.md). Gráfico: [SVG](site/bench/benchmark.svg).

| Application | Scenario | Private MiB | CPU % |
|---|---|---:|---:|
| HellerTimer/software | idle-visible | 16.9 | 0.00 |
| HellerTimer/femtovg | idle-visible | 51.9 | 0.00 |
| HellerTimer/software | running-static | 17.3 | 0.07 |
| HellerTimer/femtovg | running-static | 49.6 | 0.10 |
| HellerTimer/software | mini | 15.9 | 0.00 |
| Pomatez | idle-visible | 301.5 | 1.07 |
<!-- bench:end -->

La bandeja integrada de Slint usa ksni en Linux; sus dependencias internas están fuera del núcleo puro del temporizador. Los helpers de audio nativos solo se ejecutan durante efectos y liberan el dispositivo. El modo mini reutiliza la ventana principal. La interpolación visible del progreso es opcional y está desactivada por defecto.

## Trabajo de paridad restante

Esta implementación aún no satisface la definición completa de hecho de PLAN.md: paridad visual exacta, drag-and-drop de tareas y menús de movimiento arbitrario, seguimiento del tema del sistema, un diseño compacto separado, notificaciones del sistema de Windows, voz en navegador/Android, sonido de Android y continuación de fases tras la muerte del proceso, automatización de benchmark de arranque en frío/tamaño, macOS universal/firma/notarización y verificación en dispositivos. Los desfases horarios de los descansos especiales se resuelven al inicio; reinicia tras un cambio de zona horaria/DST. Las sesiones del temporizador no sobreviven al reinicio del proceso.

## Licencia y atribución

Código fuente: [MIT](LICENSE). Slint se usa bajo la [Slint Royalty-free License 2.0](assets/licenses/Slint-Royalty-free-2.0.md), con el widget AboutSlint en Ajustes y atribución junto a las descargas. Los [avisos de terceros](THIRD_PARTY.md) cubren los assets MIT de Pomatez, Noto Sans (OFL) y las dependencias directas. Gracias a los autores de Pomatez y Slint.
