# HellerTimer

[![Made with Slint](https://img.shields.io/badge/Made_with-Slint-007bc7)](https://slint.dev)
[![Checks](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml/badge.svg)](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) · [Русский](README.ru.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [中文](README.zh.md) · [日本語](README.ja.md)

Ein nativer Pomodoro-Timer in **Rust + Slint**, inspiriert von [Pomatez](https://github.com/zidoro/pomatez). Benannt nach Joseph Heller, dem Autor von *Catch-22*.

Timer und Desktop-Oberfläche funktionieren; die Web-Version ist eine Demo, die Desktop-App ist die Hauptanwendung.

[Releases herunterladen](https://github.com/hlophlopgaming/HellerTimer/releases) · [Website & Web-Demo](https://hlophlopgaming.github.io/HellerTimer/) · [Builds](https://github.com/hlophlopgaming/HellerTimer/actions)

<p align="center"><img src="site/screenshot.png" width="360" alt="HellerTimer unter Linux"></p>
<p align="center"><img src="site/mini.png" width="260" alt="Kompaktes Mini-Fenster mit Timer, Pause und Erweitern"></p>

## Funktionen

- Fokus, kurze und lange Pausen; konfigurierbare Runden; vier geplante Sonderpausen.
- Strikter Modus, automatischer Fokusstart, optionale Vollbild-Pausen und zusätzliche Vorab-Benachrichtigungen.
- Absolute `SystemTime`-Deadlines: verpasste Übergänge werden nach dem Aufwachen nachgeholt, nur der letzte Übergang wird angekündigt.
- Aufgabenlisten mit Titeln, Beschreibungen, Erledigt-Markierung, Neusortierung und Verschieben zwischen Listen. Enter speichert Änderungen; Pfeile sortieren um, → verschiebt in die nächste Liste. Nur leere Listen können gelöscht werden.
- Englische und russische Oberfläche (plus sieben weitere Sprachen), gebündelter Noto Sans mit Kyrillisch, fünf Themes (Light, Dark, Paper, Forest, Midnight) und die originalen Pomatez-Sounds/Sprachaufnahmen.
- Ein eigenes Sanduhr-Icon, inspiriert von Joseph Hellers *Catch-22*.
- Desktop-Tray mit Fortschritt, kompaktes Mini-Fenster, immer im Vordergrund, native/eigene Titelleiste, Schließen/Minimieren in den Tray, Einzelinstanz und Autostart.
- Optionale Update-Prüfung 30 Sekunden nach dem Start mit 5-Sekunden-Timeout. Für Start und Timer ist kein Netzwerk nötig.
- Atomarer, versionierter JSON-Speicher mit 500-ms-Speicherverzögerung. Eine beschädigte Desktop-Datei wird als `data.json.bak` gesichert.
- Standardmäßig Software-Rendering; optional FemtoVG für Vergleiche. Ein einziger Single-Shot-Timer plant die Core-Aufweckpunkte; kein Timer-Worker-Thread, kein Async-Runtime.

## Plattformstatus

| Funktion | Linux | Windows | macOS | Web | Android |
|---|---|---|---|---|---|
| Timer, Aufgaben, Konfiguration, Übersetzungen | lokal geprüft | CI-Build | CI-Build | WASM geprüft | SDK/Gerät offen |
| Tray und Fortschritt | StatusNotifier-Host nötig | implementiert | implementiert | — | Countdown-Benachrichtigung |
| Mini-Fenster / Anheften | compositorabhängig | implementiert | implementiert | im Canvas | im Layout |
| Globale Tastenkürzel | X11 | implementiert | implementiert | — | — |
| Sound / englische Stimme | `paplay` | natives WinMM | `afplay` | Glocke nach Interaktion | offen |
| Systembenachrichtigungen | `notify-send` | offen; versteckt | `osascript` | Browser-Berechtigung | Java-Brücke |
| Autostart | implementiert | implementiert | implementiert | — | — |

Nicht unterstützte Optionen werden in den Einstellungen ausgeblendet. Unter Linux werden Sound und Benachrichtigungen nur angeboten, wenn die nötigen Tools installiert sind; Tray-Steuerung braucht einen aktiven StatusNotifier-Watcher. GNOME benötigt normalerweise die AppIndicator-Erweiterung. Globale Kürzel werden unter Wayland nicht registriert. „Immer im Vordergrund“ ist dort ausgeblendet.

Desktop-Kürzel: **Strg+Alt+P** Start/Pause, **Strg+Alt+N** Überspringen, **Strg+Alt+M** Mini-Fenster. Konflikte oder nicht unterstützte Kürzel werden geloggt, die App bleibt nutzbar.

## Installation / Start

Lade ein Paket von [Releases](https://github.com/hlophlopgaming/HellerTimer/releases) herunter. Die „Made with Slint“-Attribution und vollständige Hinweise liegen den Paketen bei.

```
cargo run -p heller-desktop
cargo run -p heller-desktop -- --start
cargo run -p heller-desktop -- --mini
cargo build --release -p heller-desktop --features femtovg
```

Benötigt stabiles Rust 1.92+ und die Plattform-Bibliotheken für Slint/winit. Ubuntu:

```
sudo apt install libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libfontconfig1-dev libx11-xcb-dev
```

Optionale Linux-Utilities: `pulseaudio-utils` (`paplay`, funktioniert auch mit PipeWires PulseAudio-Kompatibilitätsserver), `libnotify-bin`, `curl` und `busctl` aus systemd. Keine GTK-Abhängigkeit.

CLI: `--start`, `--hidden`, `--mini`, `--renderer software|femtovg`, `--animation`, `--no-animation`, `--bench-first-frame`, `--help`. FemtoVG benötigt die obige Compile-Feature. `--hidden` fällt auf ein sichtbares Fenster zurück, wenn kein Tray-Host verfügbar ist. Die First-Frame-Instrumentierung meldet nur dann einen Wert, wenn der Renderer Slints Rendering-Notifier unterstützt.

Eine `portable`-Datei neben der ausführbaren Datei wählt ein benachbartes `data/`-Verzeichnis; sonst werden die Plattform-Konfigurationspfade genutzt. Windows-Portable-Archive enthalten diesen Marker. Es gibt keinen Import aus Pomatez.

Unsignierte macOS-Pakete erfordern evtl. **Open Anyway** unter Systemeinstellungen → Datenschutz & Sicherheit. Keine Apple-Signierdaten enthalten. APKs sind debug-signiert, bis ein Release-Keystore eingerichtet ist.

## Web und Android

```
rustup target add wasm32-unknown-unknown
cargo check -p heller-web --target wasm32-unknown-unknown
cargo install trunk --locked
cd crates/heller-web
trunk serve
```

Browser-Benachrichtigungen benötigen Berechtigung und einen sicheren Ursprung (HTTPS oder localhost); Sound braucht eine Nutzerinteraktion. Versteckte Tabs können Benachrichtigungen durch Browser-Throttling verzögern; die Restzeit wird bei Rückkehr aus Deadlines neu berechnet. Die Theme-Wahl wird lokal gespeichert. Dunkle/helle Einstellungen der Version 1 migrieren nach Version 2, ohne Aufgaben oder Konfiguration zu ändern. Der Speicher kann in privaten Sitzungen nicht verfügbar sein; Fehler werden geloggt. Das PWA-Manifest ermöglicht Standalone-Installation; Offline-Caching ist nicht implementiert.

Android: Java 17, Android SDK Platform 35, NDK und Gradle 8.11.1 sind nötig. Installiere `cargo-ndk`, füge die drei Rust-Android-Targets hinzu und folge `.github/workflows/release.yml`, um JNI-Bibliotheken zu erzeugen und `gradle assembleDebug` in `crates/heller-android/android` auszuführen. Verweigerte exakte Alarme fallen auf ungenaue zurück; die Benachrichtigungsberechtigung wird unter Android 13+ angefragt. Der Java-Receiver kann den nächsten Deadline ohne UI-Prozess ankündigen. **Das Fortsetzen mehrerer Phasen nach Prozess-Tod ist nicht implementiert:** Öffne die App erneut. Android-Builds und IME brauchen einen echten Gerätetest.

## Verifikation

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

Um echte Slint-Controls und Fenstergrößen in einer grafischen Sitzung zu testen:

```
cargo run -p heller-desktop --example ui-smoke
cargo run -p heller-desktop --example ui-smoke -- --start-mini
cargo run -p heller-desktop --example ui-smoke -- --frameless
```

Diese Szenarien senden Pointer-Events, prüfen jeden Mini-Fenster-Einstieg und die Wiederherstellung (inkl. versteckter Fenster), das Übergeben des nativen Ziehens und das Loslassen des Pointers, Timer-Steuerung, Aufgaben, Konfiguration, alle Themes, Russisch und Persistenz. Sie nutzen eine In-Memory-Plattform und ändern deine Nutzerdaten nie. RGBA-Screenshots und Maße landen in `$HELLER_SCREENSHOT_DIR` oder einem temporären `hellertimer-ui-smoke`-Verzeichnis. CI führt dieselben Szenarien unter Xvfb aus.

Der letzte Android-Befehl braucht SDK/JDK selbst für `cargo check`. Lange Packaging- und Release-Jobs laufen in CI. Ein `v*`-Tag-Push baut die Release-Pakete; der Workflow veröffentlicht sofort einen regulären Release als Latest und hängt danach die Pakete an. GitHub Pages wird per Actions aus `main` bereitgestellt. Kein PR nötig.

## Leistung

Siehe [Benchmark-Anleitung](bench/README.md). Ziele: ≤30 MiB sichtbares PSS, ≤20 MiB Tray-PSS, nahezu null versteckte CPU, ≤250 ms Start. Beide Renderer werden gegen den gesamten Pomatez-Prozessbaum auf derselben Maschine verglichen.

<!-- bench:start -->
Gemessen auf einem Desktop-Linux gegen Pomatez 1.11.0 (ganzer Prozessbaum, PSS). Siehe [Benchmark-Anleitung](bench/README.md). Diagramm: [SVG](site/bench/benchmark.svg).

| Application | Scenario | Private MiB | CPU % |
|---|---|---:|---:|
| HellerTimer/software | idle-visible | 16.9 | 0.00 |
| HellerTimer/femtovg | idle-visible | 51.9 | 0.00 |
| HellerTimer/software | running-static | 17.3 | 0.07 |
| HellerTimer/femtovg | running-static | 49.6 | 0.10 |
| HellerTimer/software | mini | 15.9 | 0.00 |
| Pomatez | idle-visible | 301.5 | 1.07 |
<!-- bench:end -->

Der eingebaute Slint-Tray nutzt ksni unter Linux; dessen interne Abhängigkeiten liegen außerhalb des reinen Timer-Cores. Native Audio-Helfer laufen nur während Effekten und geben das Gerät danach frei. Der Mini-Modus verwendet das Hauptfenster wieder. Sichtbare Fortschrittsinterpolation ist optional und standardmäßig deaktiviert.

## Verbleibende Paritätsarbeit

Diese erste Implementierung erfüllt noch nicht vollständig die Definition of Done aus PLAN.md: exakter visueller Paritätsabgleich, Drag-and-Drop für Aufgaben und beliebige Verschiebe-Menüs, System-Theme-Folgen, ein separates kompaktes Layout, Windows-Systembenachrichtigungen, Browser-/Android-Stimme, Android-Sound und Phasenfortsetzung nach Prozess-Tod, Automatisierung von Kaltstart/installierter Größe, macOS Universal/Signing/Notarisierung und Geräteverifikation. Sonderpausen-Zeitzonenversätze werden beim Start aufgelöst; starte nach einer DST-/Zeitzonenänderung neu. Timer-Sitzungen überleben keinen Prozessneustart.

## Lizenz und Attribution

Quellcode: [MIT](LICENSE). Slint wird unter der [Slint Royalty-free License 2.0](assets/licenses/Slint-Royalty-free-2.0.md) verwendet, mit dem AboutSlint-Widget in den Einstellungen und Attribution neben den Downloads. [Third-party notices](THIRD_PARTY.md) decken Pomatez' MIT-Assets, Noto Sans (OFL) und direkte Abhängigkeiten ab. Danke an die Pomatez- und Slint-Autoren.
