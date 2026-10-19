# HellerTimer

[![Made with Slint](https://img.shields.io/badge/Made_with-Slint-007bc7)](https://slint.dev)
[![Checks](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml/badge.svg)](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) · [Русский](README.ru.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [中文](README.zh.md) · [日本語](README.ja.md)

Un timer Pomodoro nativo scritto in **Rust + Slint**, ispirato a [Pomatez](https://github.com/zidoro/pomatez). Prende il nome da Joseph Heller, autore di *Catch-22*.

Il timer e l'interfaccia desktop funzionano; la versione web è una demo, il desktop è l'applicazione principale.

[Scarica le release](https://github.com/hlophlopgaming/HellerTimer/releases) · [Sito e demo web](https://hlophlopgaming.github.io/HellerTimer/) · [Build](https://github.com/hlophlopgaming/HellerTimer/actions)

<p align="center"><img src="site/screenshot.png" width="360" alt="HellerTimer su Linux"></p>
<p align="center"><img src="site/mini.png" width="260" alt="Mini finestra compatta con timer, pausa ed espandi"></p>

## Funzionalità

- Focus, pause brevi e lunghe; cicli configurabili; quattro pause speciali programmate.
- Modalità rigorosa, avvio automatico del focus, pause opzionali a schermo intero e notifiche anticipate extra.
- Scadenze assolute su `SystemTime`: le transizioni perse vengono riprodotte dopo la sospensione, annunciando solo l'ultima.
- Liste di attività con titoli, descrizioni, completamento, riordino e spostamento tra liste. Invio salva le modifiche; le frecce riordinano e → sposta nella lista successiva. Solo le liste vuote possono essere eliminate.
- Interfaccia in inglese e russo (più altre sette lingue), Noto Sans con cirillico incluso, cinque temi (Light, Dark, Paper, Forest, Midnight) e i suoni/registrazioni vocali originali di Pomatez.
- Un'icona a clessidra originale ispirata a *Catch-22*.
- Barra delle applicazioni con progresso, mini finestra compatta, sempre in primo piano, barra del titolo nativa/personalizzata, chiudi/minimizza nella barra, istanza singola e avvio automatico.
- Controllo opzionale degli aggiornamenti 30 secondi dopo l'avvio, con timeout di 5 secondi. Nessuna richiesta di rete è necessaria per avviare o usare il timer.
- Archiviazione JSON atomica e versionata con salvataggio differito di 500 ms. Un file danneggiato viene salvato come `data.json.bak`.
- Rendering software di default; FemtoVG opzionale per i confronti. Un solo timer single-shot pianifica i risvegli del core; nessun thread timer né runtime asincrono.

## Stato delle piattaforme

| Funzionalità | Linux | Windows | macOS | Web | Android |
|---|---|---|---|---|---|
| Timer, attività, configurazione, traduzioni | verificato localmente | build CI | build CI | WASM verificato | SDK/dispositivo in sospeso |
| Barra e progresso | serve host StatusNotifier | implementato | implementato | — | notifica di conto alla rovescia |
| Mini finestra / blocca | dipende dal compositor | implementato | implementato | nel canvas | nel layout |
| Scorciatoie globali | X11 | implementato | implementato | — | — |
| Audio / voce inglese | `paplay` | WinMM nativo | `afplay` | campana dopo interazione | in sospeso |
| Notifiche di sistema | `notify-send` | in sospeso; nascosto | `osascript` | permesso del browser | ponte Java |
| Avvio automatico | implementato | implementato | implementato | — | — |

Le opzioni non supportate sono nascoste in Impostazioni. Su Linux, audio e notifiche sono offerti solo se gli strumenti richiesti sono installati; la barra richiede un watcher StatusNotifier attivo. GNOME di solito necessita dell'estensione AppIndicator. Le scorciatoie globali non sono registrate sotto Wayland. «Sempre in primo piano» è nascosto lì.

Scorciatoie desktop: **Ctrl+Alt+P** avvia/pausa, **Ctrl+Alt+N** salta, **Ctrl+Alt+M** mini finestra. Le scorciatoie in conflitto o non supportate vengono registrate e l'app resta utilizzabile.

## Installazione / esecuzione

Scarica un pacchetto da [Releases](https://github.com/hlophlopgaming/HellerTimer/releases). L'attribuzione «Made with Slint» e gli avvisi completi accompagnano i pacchetti.

```
cargo run -p heller-desktop
cargo run -p heller-desktop -- --start
cargo run -p heller-desktop -- --mini
cargo build --release -p heller-desktop --features femtovg
```

Richiede Rust stabile 1.92+ e le librerie di piattaforma per Slint/winit. Ubuntu:

```
sudo apt install libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libfontconfig1-dev libx11-xcb-dev
```

Strumenti Linux opzionali: `pulseaudio-utils` (`paplay`, funziona anche con il server di compatibilità PulseAudio di PipeWire), `libnotify-bin`, `curl` e `busctl` di systemd. Nessuna dipendenza da GTK.

CLI: `--start`, `--hidden`, `--mini`, `--renderer software|femtovg`, `--animation`, `--no-animation`, `--bench-first-frame`, `--help`. FemtoVG richiede la feature di compilazione sopra. `--hidden` torna a mostrare una finestra se non c'è una barra disponibile. La strumentazione del primo frame riporta un valore solo se il renderer supporta il notificatore di rendering di Slint.

Un file `portable` accanto all'eseguibile seleziona una directory `data/` adiacente; altrimenti si usano le directory di configurazione della piattaforma. Gli archivi portatili Windows includono questo marcatore. Non c'è importazione da Pomatez.

I pacchetti macOS non firmati possono richiedere **Open Anyway** in Impostazioni di Sistema → Privacy e sicurezza. Nessuna credenziale di firma Apple è inclusa. Gli APK sono firmati in debug finché non si configura un keystore di release.

## Web e Android

```
rustup target add wasm32-unknown-unknown
cargo check -p heller-web --target wasm32-unknown-unknown
cargo install trunk --locked
cd crates/heller-web
trunk serve
```

Le notifiche del browser richiedono permesso e un'origine sicura (HTTPS o localhost); il suono richiede un'interazione dell'utente. Le schede nascoste possono ritardare le notifiche per il throttling; il tempo rimanente viene ricalcolato dalle scadenze al ritorno. La scelta del tema è salvata localmente. Le impostazioni scuro/chiaro della versione 1 migrano alla versione 2 senza cambiare attività o configurazione. Lo storage può non essere disponibile in sessioni private; gli errori sono registrati. Il manifest PWA consente l'installazione autonoma; il caching offline non è implementato.

Android: servono Java 17, Android SDK platform 35, NDK e Gradle 8.11.1. Installa `cargo-ndk`, aggiungi i tre target Rust Android e segui `.github/workflows/release.yml` per generare le librerie JNI ed eseguire `gradle assembleDebug` in `crates/heller-android/android`. Il rifiuto di un allarme esatto ricade su uno inesatto; il permesso di notifica è richiesto su Android 13+. Il receiver Java può annunciare la prossima scadenza senza il processo UI. **Continuare più fasi dopo la morte del processo non è implementato:** riapri l'app. Le build Android e l'IME richiedono un test su dispositivo reale.

## Verifica

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

Per esercitare controlli Slint reali e dimensioni di finestra in una sessione grafica:

```
cargo run -p heller-desktop --example ui-smoke
cargo run -p heller-desktop --example ui-smoke -- --start-mini
cargo run -p heller-desktop --example ui-smoke -- --frameless
```

Questi scenari inviano eventi puntatore, testano ogni ingresso della mini finestra e il suo ripristino (incluse finestre nascoste), il passaggio del trascinamento nativo e il rilascio del puntatore, i controlli del timer, le attività, la configurazione, tutti i temi, il russo e la persistenza. Usano una piattaforma in memoria e non modificano mai i tuoi dati. Screenshot RGBA e dimensioni vanno in `$HELLER_SCREENSHOT_DIR` o in una directory temporanea `hellertimer-ui-smoke`. La CI esegue gli stessi scenari sotto Xvfb.

L'ultimo comando Android richiede SDK/JDK anche per `cargo check`. Le attività lunghe di packaging e release girano in CI. Un push di un tag `v*` compila i pacchetti di release; il workflow pubblica subito una release regolare marcata Latest e poi allega i pacchetti. GitHub Pages viene distribuito da `main` tramite Actions. Nessun PR richiesto.

## Prestazioni

Vedi le [istruzioni di benchmark](bench/README.md). Obiettivi: ≤30 MiB di PSS visibile, ≤20 MiB di PSS in barra, CPU nascosta quasi nulla, ≤250 ms di avvio. Entrambi i renderer vengono confrontati con l'albero di processi completo di Pomatez sulla stessa macchina.

<!-- bench:start -->
Misurato su un Linux desktop contro Pomatez 1.11.0 (intero albero di processi, PSS). Vedi le [istruzioni di benchmark](bench/README.md). Grafico: [SVG](site/bench/benchmark.svg).

| Application | Scenario | Private MiB | CPU % |
|---|---|---:|---:|
| HellerTimer/software | idle-visible | 16.9 | 0.00 |
| HellerTimer/femtovg | idle-visible | 51.9 | 0.00 |
| HellerTimer/software | running-static | 17.3 | 0.07 |
| HellerTimer/femtovg | running-static | 49.6 | 0.10 |
| HellerTimer/software | mini | 15.9 | 0.00 |
| Pomatez | idle-visible | 301.5 | 1.07 |
<!-- bench:end -->

La barra integrata di Slint usa ksni su Linux; le sue dipendenze interne sono fuori dal core puro del timer. Gli helper audio nativi girano solo durante gli effetti e rilasciano il dispositivo. La modalità mini riutilizza la finestra principale. L'interpolazione visibile del progresso è opzionale e disattivata di default.

## Lavoro di parità rimanente

Questa implementazione non soddisfa ancora completamente la definizione di completato di PLAN.md: parità visiva esatta, trascinamento delle attività e menu di spostamento arbitrario, seguire il tema di sistema, un layout compatto separato, notifiche di sistema Windows, voce browser/Android, audio Android e continuazione delle fasi dopo la morte del processo, automazione dei benchmark di avvio a freddo/dimensione, macOS universale/firma/notarizzazione e verifica su dispositivi. Gli offset di fuso delle pause speciali sono risolti all'avvio; riavvia dopo un cambio di fuso/DST. Le sessioni del timer non sopravvivono al riavvio del processo.

## Licenza e attribuzione

Codice sorgente: [MIT](LICENSE). Slint è usato sotto la [Slint Royalty-free License 2.0](assets/licenses/Slint-Royalty-free-2.0.md), con il widget AboutSlint in Impostazioni e l'attribuzione accanto ai download. Gli [avvisi di terze parti](THIRD_PARTY.md) coprono gli asset MIT di Pomatez, Noto Sans (OFL) e le dipendenze dirette. Grazie agli autori di Pomatez e Slint.
