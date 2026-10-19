# HellerTimer

[![Made with Slint](https://img.shields.io/badge/Made_with-Slint-007bc7)](https://slint.dev)
[![Checks](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml/badge.svg)](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) · [Русский](README.ru.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [中文](README.zh.md) · [日本語](README.ja.md)

Un minuteur Pomodoro natif écrit en **Rust + Slint**, inspiré de [Pomatez](https://github.com/zidoro/pomatez). Nommé d'après Joseph Heller, l'auteur de *Catch-22*.

Le minuteur et l'interface de bureau fonctionnent ; la version web est une démo, l'application de bureau est la principale.

[Télécharger les versions](https://github.com/hlophlopgaming/HellerTimer/releases) · [Site et démo web](https://hlophlopgaming.github.io/HellerTimer/) · [Builds](https://github.com/hlophlopgaming/HellerTimer/actions)

<p align="center"><img src="site/screenshot.png" width="360" alt="HellerTimer sous Linux"></p>
<p align="center"><img src="site/mini.png" width="260" alt="Mini fenêtre compacte avec minuteur, pause et agrandir"></p>

## Fonctionnalités

- Concentration, pauses courtes et longues ; rondes configurables ; quatre pauses spéciales planifiées.
- Mode strict, démarrage automatique de la concentration, pauses optionnelles en plein écran et notifications anticipées supplémentaires.
- Échéances absolues sur `SystemTime` : les transitions manquées sont rejouées après la veille, seule la dernière est annoncée.
- Listes de tâches avec titres, descriptions, achèvement, réordonnancement et déplacement entre listes. Entrée enregistre les modifications ; les flèches réordonnent et → déplace vers la liste suivante. Seules les listes vides peuvent être supprimées.
- Interface en anglais et en russe (plus sept autres langues), Noto Sans avec cyrillique inclus, cinq thèmes (Light, Dark, Paper, Forest, Midnight) et les sons/voix originaux de Pomatez.
- Une icône de sablier originale inspirée de *Catch-22*.
- Barre d'état avec progression, mini fenêtre compacte, toujours au premier plan, barre de titre native/personnalisée, fermeture/réduction dans la barre d'état, instance unique et démarrage automatique.
- Vérification optionnelle des mises à jour 30 secondes après le démarrage, avec un délai de 5 secondes. Aucune requête réseau n'est nécessaire pour démarrer ni utiliser le minuteur.
- Stockage JSON atomique et versionné avec sauvegarde différée de 500 ms. Un fichier endommagé est sauvegardé en `data.json.bak`.
- Rendu logiciel par défaut ; FemtoVG optionnel pour les comparaisons. Un seul minuteur single-shot planifie les réveils du cœur ; pas de thread de minuteur ni de runtime asynchrone.

## Statut par plateforme

| Fonctionnalité | Linux | Windows | macOS | Web | Android |
|---|---|---|---|---|---|
| Minuteur, tâches, configuration, traductions | vérifié localement | build CI | build CI | WASM vérifié | SDK/appareil en attente |
| Barre d'état et progression | hôte StatusNotifier requis | implémenté | implémenté | — | notification de compte à rebours |
| Mini fenêtre / épingler | dépend du compositeur | implémenté | implémenté | dans le canevas | dans la mise en page |
| Raccourcis globaux | X11 | implémenté | implémenté | — | — |
| Son / voix anglaise | `paplay` | WinMM natif | `afplay` | cloche après interaction | en attente |
| Notifications système | `notify-send` | en attente ; masqué | `osascript` | permission du navigateur | pont Java |
| Démarrage automatique | implémenté | implémenté | implémenté | — | — |

Les options non prises en charge sont masquées dans Paramètres. Sous Linux, le son et les notifications ne sont proposés que si les utilitaires requis sont installés ; la barre d'état exige un watcher StatusNotifier actif. GNOME nécessite normalement l'extension AppIndicator. Les raccourcis globaux ne sont pas enregistrés sous Wayland. « Toujours au premier plan » y est masqué.

Raccourcis de bureau : **Ctrl+Alt+P** démarrer/pause, **Ctrl+Alt+N** passer, **Ctrl+Alt+M** mini fenêtre. Les raccourcis en conflit ou non pris en charge sont journalisés et l'application reste utilisable.

## Installation / exécution

Téléchargez un paquet depuis [Releases](https://github.com/hlophlopgaming/HellerTimer/releases). L'attribution « Made with Slint » et les avis complets accompagnent les paquets.

```
cargo run -p heller-desktop
cargo run -p heller-desktop -- --start
cargo run -p heller-desktop -- --mini
cargo build --release -p heller-desktop --features femtovg
```

Nécessite Rust stable 1.92+ et les bibliothèques de plateforme pour Slint/winit. Ubuntu :

```
sudo apt install libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libfontconfig1-dev libx11-xcb-dev
```

Utilitaires Linux optionnels : `pulseaudio-utils` (`paplay`, fonctionne aussi avec le serveur de compatibilité PulseAudio de PipeWire), `libnotify-bin`, `curl` et `busctl` de systemd. Aucune dépendance GTK.

CLI : `--start`, `--hidden`, `--mini`, `--renderer software|femtovg`, `--animation`, `--no-animation`, `--bench-first-frame`, `--help`. FemtoVG nécessite la feature de compilation ci-dessus. `--hidden` revient à afficher une fenêtre si aucune barre d'état n'est disponible. L'instrumentation du premier frame ne rapporte une valeur que si le renderer prend en charge le notificateur de rendu de Slint.

Un fichier `portable` à côté de l'exécutable sélectionne un dossier `data/` voisin ; sinon, les dossiers de configuration de la plateforme sont utilisés. Les archives portables Windows incluent ce marqueur. Il n'y a pas d'import depuis Pomatez.

Les paquets macOS non signés peuvent exiger **Open Anyway** dans Réglages Système → Confidentialité et sécurité. Aucune identité de signature Apple n'est incluse. Les APK sont signés en debug jusqu'à la configuration d'un keystore de release.

## Web et Android

```
rustup target add wasm32-unknown-unknown
cargo check -p heller-web --target wasm32-unknown-unknown
cargo install trunk --locked
cd crates/heller-web
trunk serve
```

Les notifications du navigateur exigent une permission et une origine sécurisée (HTTPS ou localhost) ; le son exige une interaction utilisateur. Les onglets masqués peuvent retarder les notifications à cause du throttling ; le temps restant est recalculé à partir des échéances au retour. Le choix du thème est enregistré localement. Les réglages sombre/clair de la version 1 migrent vers la version 2 sans changer les tâches ni la configuration. Le stockage peut être indisponible en navigation privée ; les erreurs sont journalisées. Le manifeste PWA permet l'installation autonome ; la mise en cache hors ligne n'est pas implémentée.

Android : Java 17, Android SDK platform 35, NDK et Gradle 8.11.1 sont requis. Installez `cargo-ndk`, ajoutez les trois cibles Rust Android, puis suivez `.github/workflows/release.yml` pour générer les bibliothèques JNI et exécutez `gradle assembleDebug` dans `crates/heller-android/android`. Un refus d'alarme exacte retombe sur une alarme inexacte ; la permission de notification est demandée sous Android 13+. Le récepteur Java peut annoncer la prochaine échéance sans le processus d'interface. **La poursuite de plusieurs phases après la mort du processus n'est pas implémentée :** rouvrez l'application. Les builds Android et l'IME nécessitent un test sur appareil réel.

## Vérification

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

Pour exercer de vrais contrôles Slint et des tailles de fenêtre dans une session graphique :

```
cargo run -p heller-desktop --example ui-smoke
cargo run -p heller-desktop --example ui-smoke -- --start-mini
cargo run -p heller-desktop --example ui-smoke -- --frameless
```

Ces scénarios envoient des événements de pointeur, testent chaque entrée de la mini fenêtre et sa restauration (y compris les fenêtres masquées), le transfert du glissement natif et le relâchement du pointeur, les contrôles du minuteur, les tâches, la configuration, tous les thèmes, le russe et la persistance. Ils utilisent une plateforme en mémoire et ne modifient jamais vos données. Les captures RGBA et les dimensions vont dans `$HELLER_SCREENSHOT_DIR` ou un dossier temporaire `hellertimer-ui-smoke`. La CI exécute les mêmes scénarios sous Xvfb.

La dernière commande Android exige un SDK/JDK même pour `cargo check`. Les tâches longues d'empaquetage et de release s'exécutent dans la CI. Un push de tag `v*` construit les paquets de release ; le workflow publie immédiatement une release régulière marquée Latest, puis joint les paquets. GitHub Pages est déployé depuis `main` via Actions. Aucun PR requis.

## Performance

Voir les [instructions de benchmark](bench/README.md). Objectifs : ≤30 MiB de PSS visible, ≤20 MiB de PSS en barre d'état, CPU masqué quasi nul, ≤250 ms de démarrage. Les deux renderers sont comparés à l'arbre de processus complet de Pomatez sur la même machine.

<!-- bench:start -->
Mesuré sur un Linux de bureau face à Pomatez 1.11.0 (arbre de processus complet, PSS). Voir les [instructions de benchmark](bench/README.md). Graphique : [SVG](site/bench/benchmark.svg).

| Application | Scenario | Private MiB | CPU % |
|---|---|---:|---:|
| HellerTimer/software | idle-visible | 16.9 | 0.00 |
| HellerTimer/femtovg | idle-visible | 51.9 | 0.00 |
| HellerTimer/software | running-static | 17.3 | 0.07 |
| HellerTimer/femtovg | running-static | 49.6 | 0.10 |
| HellerTimer/software | mini | 15.9 | 0.00 |
| Pomatez | idle-visible | 301.5 | 1.07 |
<!-- bench:end -->

La barre d'état intégrée de Slint utilise ksni sous Linux ; ses dépendances internes sont hors du cœur pur du minuteur. Les helpers audio natifs ne s'exécutent que pendant les effets et libèrent le périphérique. Le mode mini réutilise la fenêtre principale. L'interpolation visible de la progression est optionnelle et désactivée par défaut.

## Travail de parité restant

Cette implémentation ne satisfait pas encore entièrement la définition de fini de PLAN.md : parité visuelle exacte, glisser-déposer des tâches et menus de déplacement arbitraire, suivi du thème système, une mise en page compacte séparée, notifications système Windows, voix navigateur/Android, son Android et continuation des phases après la mort du processus, automatisation des benchmarks de démarrage à froid/taille, macOS universel/signature/notarisation et vérification sur appareils. Les décalages horaires des pauses spéciales sont résolus au démarrage ; redémarrez après un changement de fuseau/DST. Les sessions de minuteur ne survivent pas au redémarrage du processus.

## Licence et attribution

Code source : [MIT](LICENSE). Slint est utilisé sous la [Slint Royalty-free License 2.0](assets/licenses/Slint-Royalty-free-2.0.md), avec le widget AboutSlint dans Paramètres et l'attribution à côté des téléchargements. Les [avis tiers](THIRD_PARTY.md) couvrent les assets MIT de Pomatez, Noto Sans (OFL) et les dépendances directes. Merci aux auteurs de Pomatez et de Slint.
