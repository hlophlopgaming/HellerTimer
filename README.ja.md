# HellerTimer

[![Made with Slint](https://img.shields.io/badge/Made_with-Slint-007bc7)](https://slint.dev)
[![Checks](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml/badge.svg)](https://github.com/hlophlopgaming/HellerTimer/actions/workflows/check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[English](README.md) · [Русский](README.ru.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [中文](README.zh.md) · [日本語](README.ja.md)

**Rust + Slint** で書かれたネイティブのポモドーロタイマー。[Pomatez](https://github.com/zidoro/pomatez) に触発され、*Catch-22*（『キャッチ=22』）の著者ジョセフ・ヘラーにちなんで名付けられました。

タイマーとデスクトップ UI は動作しています。Web 版はデモで、デスクトップ版がメインのアプリです。

[リリースをダウンロード](https://github.com/hlophlopgaming/HellerTimer/releases) · [ウェブサイトと Web デモ](https://hlophlopgaming.github.io/HellerTimer/) · [ビルド](https://github.com/hlophlopgaming/HellerTimer/actions)

<p align="center"><img src="site/screenshot.png" width="360" alt="Linux 上で動作する HellerTimer"></p>
<p align="center"><img src="site/mini.png" width="260" alt="タイマー、一時停止、展開ボタンを備えたコンパクトなミニウィンドウ"></p>

## 機能

- 集中、短い休憩、長い休憩。設定可能なラウンドと 4 つの予定特別休憩。
- ストリクトモード、集中の自動開始、オプションの全画面休憩、追加の事前通知。
- `SystemTime` による絶対的なデッドライン：スリープ中に逃した遷移は復元され、最後の遷移のみが通知されます。
- タイトル・説明・完了・並べ替え・リスト間移動に対応したタスクリスト。Enter で編集を保存。矢印で並べ替え、→ で次のリストへ。削除できるのは空のリストのみです。
- 英語・ロシア語の UI（ほか 7 言語）、キリル文字対応の Noto Sans 同梱、5 つのテーマ（Light、Dark、Paper、Forest、Midnight）、Pomatez オリジナルのサウンド/音声。
- *Catch-22* に着想を得たオリジナルの砂時計アイコン。
- 進捗表示付きデスクトップトレイ、コンパクトなミニウィンドウ、常に手前に表示、ネイティブ/カスタムタイトルバー、トレイへのクローズ/最小化、単一インスタンス、自動起動。
- 起動 30 秒後にオプションで更新を確認（タイムアウト 5 秒）。タイマーの起動・利用にネットワークは不要です。
- アトミックでバージョン管理された JSON ストレージと 500 ms の保存デバウンス。破損したデスクトップファイルは `data.json.bak` としてバックアップされます。
- デフォルトはソフトウェアレンダリング。比較用に FemtoVG も選択可能。単一の single-shot タイマーがコアのウェイクアップをスケジュール。タイマーのワーカースレッドも async ランタイムもありません。

## プラットフォーム状況

| 機能 | Linux | Windows | macOS | Web | Android |
|---|---|---|---|---|---|
| タイマー、タスク、設定、翻訳 | ローカルで確認済み | CI ビルド | CI ビルド | WASM 確認済み | SDK/実機は保留 |
| トレイと進捗 | StatusNotifier ホストが必要 | 実装済み | 実装済み | — | カウントダウン通知 |
| ミニウィンドウ / 固定 | コンポジター依存 | 実装済み | 実装済み | キャンバス内 | レイアウト内 |
| グローバルショートカット | X11 | 実装済み | 実装済み | — | — |
| サウンド / 英語音声 | `paplay` | ネイティブ WinMM | `afplay` | 操作後のベル | 保留 |
| システム通知 | `notify-send` | 保留；非表示 | `osascript` | ブラウザの許可 | Java ブリッジ |
| 自動起動 | 実装済み | 実装済み | 実装済み | — | — |

非対応のオプションは設定で非表示になります。Linux では、必要なツールがインストールされている場合のみサウンドと通知が提供され、トレイにはアクティブな StatusNotifier ウォッチャーが必要です。GNOME では通常 AppIndicator 拡張が必要です。Wayland ではグローバルショートカットは登録されません。「常に手前に表示」もそこで非表示になります。

デスクトップショートカット：**Ctrl+Alt+P** 開始/一時停止、**Ctrl+Alt+N** スキップ、**Ctrl+Alt+M** ミニウィンドウ。競合・非対応のショートカットはログに記録され、アプリは引き続き使用できます。

## インストール / 実行

[Releases](https://github.com/hlophlopgaming/HellerTimer/releases) からパッケージをダウンロードします。「Made with Slint」の帰属表示と完全な通知がパッケージに同梱されます。

```
cargo run -p heller-desktop
cargo run -p heller-desktop -- --start
cargo run -p heller-desktop -- --mini
cargo build --release -p heller-desktop --features femtovg
```

安定版 Rust 1.92+ と Slint/winit 用のプラットフォームライブラリが必要です。Ubuntu：

```
sudo apt install libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libfontconfig1-dev libx11-xcb-dev
```

オプションの Linux ツール：`pulseaudio-utils`（`paplay`、PipeWire の PulseAudio 互換サーバーでも動作）、`libnotify-bin`、`curl`、systemd の `busctl`。GTK への依存はありません。

CLI：`--start`、`--hidden`、`--mini`、`--renderer software|femtovg`、`--animation`、`--no-animation`、`--bench-first-frame`、`--help`。FemtoVG には上記のコンパイルフィーチャーが必要です。`--hidden` はトレイホストが利用できない場合、ウィンドウ表示にフォールバックします。初回フレーム計測は、レンダラーが Slint のレンダリング通知をサポートする場合にのみ値を報告します。

実行ファイルの隣の `portable` ファイルは隣接する `data/` ディレクトリを選択します。それ以外はプラットフォームの設定ディレクトリが使われます。Windows のポータブルアーカイブにはこのマーカーが含まれます。Pomatez からのインポートはありません。

未署名の macOS パッケージは「システム設定 → プライバシーとセキュリティ」で **Open Anyway** が必要な場合があります。Apple の署名資格情報は含まれていません。APK はリリース用キーストアが設定されるまで debug 署名です。

## Web と Android

```
rustup target add wasm32-unknown-unknown
cargo check -p heller-web --target wasm32-unknown-unknown
cargo install trunk --locked
cd crates/heller-web
trunk serve
```

ブラウザ通知には許可と安全なオリジン（HTTPS または localhost）が必要で、サウンドにはユーザー操作が必要です。非表示タブではブラウザのスロットリングで通知が遅れることがあり、復帰時に残り時間がデッドラインから再計算されます。テーマ選択はローカルに保存されます。バージョン 1 のダーク/ライト設定はタスクやタイマー設定を変えずにバージョン 2 へ移行します。プライベート/制限付きセッションではブラウザストレージが利用できない場合があり、エラーはログに記録されます。PWA マニフェストによりスタンドアロンインストールが可能です。オフラインキャッシュは実装されていません。

Android：Java 17、Android SDK platform 35、NDK、Gradle 8.11.1 が必要です。`cargo-ndk` をインストールし、3 つの Rust Android ターゲットを追加して、`.github/workflows/release.yml` に従って JNI ライブラリを生成し、`crates/heller-android/android` で `gradle assembleDebug` を実行します。正確なアラームが拒否されると不正確なアラームにフォールバックし、Android 13+ では通知許可が要求されます。Java レシーバーは UI プロセスなしで次のデッドラインを通知できます。**プロセス終了後の複数フェーズの継続は未実装です：**アプリを開き直してください。Android ビルドと IME は実機でのテストが必要です。

## 検証

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

グラフィカルセッションで実際の Slint コントロールとウィンドウサイズを確認するには：

```
cargo run -p heller-desktop --example ui-smoke
cargo run -p heller-desktop --example ui-smoke -- --start-mini
cargo run -p heller-desktop --example ui-smoke -- --frameless
```

これらのシナリオはポインターイベントを送信し、ミニウィンドウの全エントリと復元（非表示ウィンドウを含む）、ネイティブドラッグの引き継ぎとポインター解放、タイマーコントロール、タスク、設定、全テーマ、ロシア語、永続化を検証します。インメモリのプラットフォームを使うため、ユーザーデータは一切変更されません。RGBA スクリーンショットと寸法は `$HELLER_SCREENSHOT_DIR` または一時的な `hellertimer-ui-smoke` ディレクトリに出力されます。CI は同じシナリオを Xvfb 上で実行します。

最後の Android コマンドは `cargo check` でも SDK/JDK が必要です。時間のかかるパッケージングとリリースのジョブは CI で実行されます。`v*` タグをプッシュするとリリースパッケージがビルドされ、ワークフローは Latest とマークされた通常リリースを即時公開し、全プラットフォームのビルド成功後にパッケージを添付します。GitHub Pages は Actions によって `main` からデプロイされます。PR は不要です。

## パフォーマンス

[ベンチマーク手順](bench/README.md)を参照してください。目標：可視 PSS 30 MiB 以下、トレイ PSS 20 MiB 以下、ほぼゼロの非表示 CPU、起動 250 ms 以下。両レンダラーを同一マシン上の Pomatez の全プロセスツリーと比較する必要があります。

<!-- bench:start -->
デスクトップ Linux で Pomatez 1.11.0 と比較して測定（全プロセスツリー、PSS）。[ベンチマーク手順](bench/README.md)を参照。グラフ：[SVG](site/bench/benchmark.svg)。

| Application | Scenario | Private MiB | CPU % |
|---|---|---:|---:|
| HellerTimer/software | idle-visible | 16.9 | 0.00 |
| HellerTimer/femtovg | idle-visible | 51.9 | 0.00 |
| HellerTimer/software | running-static | 17.3 | 0.07 |
| HellerTimer/femtovg | running-static | 49.6 | 0.10 |
| HellerTimer/software | mini | 15.9 | 0.00 |
| Pomatez | idle-visible | 301.5 | 1.07 |
<!-- bench:end -->

組み込みの Slint トレイは Linux で ksni を使用します。その内部依存は純粋なタイマーコアの外にあります。ネイティブオーディオヘルパーはエフェクト中のみ動作し、終了後にデバイスを解放します。ミニモードはメインウィンドウを再利用します。可視進捗の補間はオプションで、デフォルトでは無効です。

## 残っているパリティ作業

この初期実装は PLAN.md の完全な完成定義をまだ満たしていません。残る作業には、正確なビジュアル/間隔の一致、タスクのドラッグ＆ドロップと任意の移動メニュー、システムテーマへの追従、別のコンパクトレイアウト、Windows のシステム通知、ブラウザ/Android 音声、Android サウンドとプロセス終了後のフェーズ継続、コールドスタート/インストールサイズのベンチマーク自動化、macOS のユニバーサル/署名/公証、実機検証が含まれます。特別休憩のタイムゾーンオフセットは起動時に解決されます。DST/タイムゾーン変更後は再起動してください。タイマーセッションはプロセス再起動をまたいで保持されません。

## ライセンスと帰属

プロジェクトのソースコード：[MIT](LICENSE)。Slint は [Slint Royalty-free License 2.0](assets/licenses/Slint-Royalty-free-2.0.md) に基づいて使用され、設定内に AboutSlint ウィジェット、ダウンロード横に帰属表示があります。[サードパーティ通知](THIRD_PARTY.md)は Pomatez の MIT アセット、Noto Sans（OFL）、直接依存をカバーします。Pomatez と Slint の貢献者に感謝します。
