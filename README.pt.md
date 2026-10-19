# HellerTimer

[![Made with Slint](https://img.shields.io/badge/Made_with-Slint-007bc7)](https://slint.dev)
[![Checks](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml/badge.svg)](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) · [Русский](README.ru.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [中文](README.zh.md) · [日本語](README.ja.md)

Um temporizador Pomodoro nativo escrito em **Rust + Slint**, inspirado no [Pomatez](https://github.com/zidoro/pomatez). Nomeado em homenagem a Joseph Heller, autor de *Catch-22*.

O temporizador e a interface de desktop funcionam; a versão web é uma demonstração, o desktop é o aplicativo principal.

[Baixar versões](https://github.com/hlophlopgaming/HellerTimer/releases) · [Site e demo web](https://hlophlopgaming.github.io/HellerTimer/) · [Builds](https://github.com/hlophlopgaming/HellerTimer/actions)

<p align="center"><img src="site/screenshot.png" width="360" alt="HellerTimer no Linux"></p>
<p align="center"><img src="site/mini.png" width="260" alt="Mini janela compacta com temporizador, pausa e expandir"></p>

## Funcionalidades

- Foco, pausas curtas e longas; rodadas configuráveis; quatro pausas especiais programadas.
- Modo estrito, início automático do foco, pausas opcionais em tela cheia e notificações antecipadas extras.
- Prazos absolutos em `SystemTime`: transições perdidas são reproduzidas após o sono, anunciando apenas a última.
- Listas de tarefas com títulos, descrições, conclusão, reordenação e movimentação entre listas. Enter salva as edições; as setas reordenam e → move para a próxima lista. Apenas listas vazias podem ser excluídas.
- Interface em inglês e russo (e mais sete idiomas), Noto Sans com cirílico incluído, cinco temas (Light, Dark, Paper, Forest, Midnight) e os sons/vozes originais do Pomatez.
- Um ícone de ampulheta original inspirado em *Catch-22*.
- Bandeja com progresso, mini janela compacta, sempre no topo, barra de título nativa/própria, fechar/minimizar na bandeja, instância única e início automático.
- Verificação opcional de atualizações 30 segundos após o início, com limite de 5 segundos. Nenhuma requisição de rede é necessária para iniciar ou usar o temporizador.
- Armazenamento JSON atômico e versionado com salvamento diferido de 500 ms. Um arquivo danificado é copiado como `data.json.bak`.
- Renderização por software por padrão; FemtoVG opcional para comparações. Um único temporizador single-shot agenda os despertares do núcleo; sem thread de temporizador nem runtime assíncrono.

## Status por plataforma

| Funcionalidade | Linux | Windows | macOS | Web | Android |
|---|---|---|---|---|---|
| Temporizador, tarefas, configuração, traduções | verificado localmente | build CI | build CI | WASM verificado | SDK/dispositivo pendente |
| Bandeja e progresso | requer host StatusNotifier | implementado | implementado | — | notificação de contagem |
| Mini janela / fixar | depende do compositor | implementado | implementado | no canvas | no layout |
| Atalhos globais | X11 | implementado | implementado | — | — |
| Som / voz em inglês | `paplay` | WinMM nativo | `afplay` | sino após interação | pendente |
| Notificações do sistema | `notify-send` | pendente; oculto | `osascript` | permissão do navegador | ponte Java |
| Início automático | implementado | implementado | implementado | — | — |

Opções não suportadas ficam ocultas em Configurações. No Linux, som e notificações só são oferecidos se os utilitários estiverem instalados; a bandeja requer um watcher StatusNotifier ativo. O GNOME normalmente precisa da extensão AppIndicator. Atalhos globais não são registrados no Wayland. «Sempre no topo» fica oculto lá.

Atalhos de desktop: **Ctrl+Alt+P** iniciar/pausar, **Ctrl+Alt+N** pular, **Ctrl+Alt+M** mini janela. Atalhos conflitantes ou não suportados são registrados e o app continua utilizável.

## Instalação / execução

Baixe um pacote de [Releases](https://github.com/hlophlopgaming/HellerTimer/releases). A atribuição «Made with Slint» e os avisos completos acompanham os pacotes.

```
cargo run -p heller-desktop
cargo run -p heller-desktop -- --start
cargo run -p heller-desktop -- --mini
cargo build --release -p heller-desktop --features femtovg
```

Requer Rust estável 1.92+ e as bibliotecas de plataforma para Slint/winit. Ubuntu:

```
sudo apt install libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libfontconfig1-dev libx11-xcb-dev
```

Utilitários opcionais do Linux: `pulseaudio-utils` (`paplay`, funciona também com o servidor de compatibilidade PulseAudio do PipeWire), `libnotify-bin`, `curl` e `busctl` do systemd. Não há dependência de GTK.

CLI: `--start`, `--hidden`, `--mini`, `--renderer software|femtovg`, `--animation`, `--no-animation`, `--bench-first-frame`, `--help`. FemtoVG requer a feature de compilação acima. `--hidden` volta a mostrar uma janela quando não há bandeja disponível. A instrumentação do primeiro frame só reporta valor se o renderizador suportar o notificador de renderização do Slint.

Um arquivo `portable` ao lado do executável seleciona um diretório `data/` vizinho; caso contrário, usam-se os diretórios de configuração da plataforma. Os arquivos portáteis do Windows incluem esse marcador. Não há importação do Pomatez.

Pacotes macOS não assinados podem exigir **Open Anyway** em Ajustes do Sistema → Privacidade e Segurança. Nenhuma credencial de assinatura Apple está incluída. APKs são assinados em debug até que um keystore de release seja configurado.

## Web e Android

```
rustup target add wasm32-unknown-unknown
cargo check -p heller-web --target wasm32-unknown-unknown
cargo install trunk --locked
cd crates/heller-web
trunk serve
```

Notificações do navegador exigem permissão e origem segura (HTTPS ou localhost); o som precisa de interação do usuário. Abas ocultas podem atrasar notificações por throttling do navegador; o tempo restante é recalculado a partir dos prazos ao retornar. A escolha do tema é salva localmente. Configurações claro/escuro da versão 1 migram para a versão 2 sem alterar tarefas ou configuração. O armazenamento pode ficar indisponível em sessões privadas; erros são registrados. O manifesto PWA permite instalação independente; o cache offline não é implementado.

Android: Java 17, Android SDK platform 35, NDK e Gradle 8.11.1 são necessários. Instale o `cargo-ndk`, adicione os três targets Rust Android e siga `.github/workflows/release.yml` para gerar as bibliotecas JNI e executar `gradle assembleDebug` em `crates/heller-android/android`. A negação de alarme exato cai para um inexato; a permissão de notificação é pedida no Android 13+. O receptor Java pode anunciar o próximo prazo sem o processo de UI. **Continuar várias fases após a morte do processo não está implementado:** reabra o app. Builds Android e IME precisam de teste em dispositivo real.

## Verificação

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

Para exercitar controles Slint reais e tamanhos de janela em uma sessão gráfica:

```
cargo run -p heller-desktop --example ui-smoke
cargo run -p heller-desktop --example ui-smoke -- --start-mini
cargo run -p heller-desktop --example ui-smoke -- --frameless
```

Esses cenários enviam eventos de ponteiro, testam cada entrada da mini janela e sua restauração (incluindo janelas ocultas), a transferência do arrasto nativo e a liberação do ponteiro, controles do temporizador, tarefas, configuração, todos os temas, russo e persistência. Eles usam uma plataforma em memória e nunca modificam seus dados. Capturas RGBA e dimensões vão para `$HELLER_SCREENSHOT_DIR` ou um diretório temporário `hellertimer-ui-smoke`. A CI executa os mesmos cenários sob Xvfb.

O último comando Android requer SDK/JDK até para `cargo check`. Tarefas longas de empacotamento e release rodam na CI. Um push de tag `v*` constrói os pacotes de release; o workflow publica imediatamente uma release regular marcada como Latest e depois anexa os pacotes. O GitHub Pages é implantado a partir de `main` via Actions. Nenhum PR é necessário.

## Desempenho

Veja as [instruções de benchmark](bench/README.md). Metas: ≤30 MiB de PSS visível, ≤20 MiB de PSS na bandeja, CPU oculta quase nula, ≤250 ms de início. Ambos os renderizadores são comparados com a árvore de processos completa do Pomatez na mesma máquina.

<!-- bench:start -->
Medido em um Linux de desktop contra o Pomatez 1.11.0 (árvore de processos completa, PSS). Veja as [instruções de benchmark](bench/README.md). Gráfico: [SVG](site/bench/benchmark.svg).

| Application | Scenario | Private MiB | CPU % |
|---|---|---:|---:|
| HellerTimer/software | idle-visible | 16.9 | 0.00 |
| HellerTimer/femtovg | idle-visible | 51.9 | 0.00 |
| HellerTimer/software | running-static | 17.3 | 0.07 |
| HellerTimer/femtovg | running-static | 49.6 | 0.10 |
| HellerTimer/software | mini | 15.9 | 0.00 |
| Pomatez | idle-visible | 301.5 | 1.07 |
<!-- bench:end -->

A bandeja integrada do Slint usa ksni no Linux; suas dependências internas estão fora do núcleo puro do temporizador. Os helpers de áudio nativos rodam apenas durante efeitos e liberam o dispositivo. O modo mini reutiliza a janela principal. A interpolação visível do progresso é opcional e desativada por padrão.

## Trabalho de paridade restante

Esta implementação ainda não satisfaz totalmente a definição de pronto do PLAN.md: paridade visual exata, arrastar-e-soltar de tarefas e menus de movimentação arbitrária, seguir o tema do sistema, um layout compacto separado, notificações de sistema do Windows, voz em navegador/Android, som do Android e continuação de fases após a morte do processo, automação de benchmark de início frio/tamanho, macOS universal/assinatura/notarização e verificação em dispositivos. Os deslocamentos de fuso das pausas especiais são resolvidos no início; reinicie após uma mudança de fuso/DST. As sessões do temporizador não sobrevivem ao reinício do processo.

## Licença e atribuição

Código-fonte: [MIT](LICENSE). O Slint é usado sob a [Slint Royalty-free License 2.0](assets/licenses/Slint-Royalty-free-2.0.md), com o widget AboutSlint em Configurações e atribuição junto aos downloads. Os [avisos de terceiros](THIRD_PARTY.md) cobrem os assets MIT do Pomatez, o Noto Sans (OFL) e as dependências diretas. Obrigado aos autores do Pomatez e do Slint.
