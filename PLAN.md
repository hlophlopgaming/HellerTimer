# PLAN: HellerTimer — нативный клон Pomatez на Rust + Slint (one-shot)

> Задание для агента. Выполнить целиком за один проход: от пустого репозитория до рабочего приложения, сайта, бенчмарка и упаковки.
> **Не открывать PR и не ждать CI.** Для самопроверки хватит `cargo check` по таргетам и `cargo clippy --workspace`; юнит-тесты `heller-core` написать, прогон необязателен.
> Референс поведения и дизайна: склонировать `https://github.com/zidoro/pomatez` во временную папку и сверяться с исходниками (`app/renderer`, `app/electron`) по логике таймера, текстам, цветам, шрифтам, иконкам, размерам и звукам. Где план и оригинал расходятся в мелочах — следовать оригиналу и оставить комментарий `// parity:`.
> Перед началом прочитать актуальную документацию Slint (docs.slint.dev) для последней стабильной версии: названия фич, winit-аксессоров и Android-бэкенда меняются между версиями.

- Название: **HellerTimer** (в честь Джозефа Хеллера, автора «Уловки-22»).
- Репозиторий: `github.com/hlophlopgaming/HellerTimer`.
- App ID: `io.github.hlophlopgaming.HellerTimer`. Бинарь: `hellertimer`.
- UI: **Slint** (`.slint`-разметка + Rust).

### Лицензии
- Код проекта — **MIT**.
- Slint используется под **Slint Royalty-free License 2.0** (бесплатно для desktop, mobile и web, совместимо с MIT-кодом). Условия, которые обязательно выполнить:
  1. Виджет `AboutSlint` на экране «О программе», доступном из верхнего уровня навигации.
  2. Бейдж «Made with Slint» на публичной странице, откуда скачиваются бинарники (сайт и README).
  3. Не удалять лицензионные уведомления Slint.
- Pomatez — MIT: дизайн, иконки, звуки и голос можно переносить; копирайт Pomatez, лицензии Slint и шрифтов — в `THIRD_PARTY.md`.

---

## 1. Цели и бюджет производительности

Паритет функций и внешнего вида с Pomatez при на порядок меньшем потреблении. Бенчмарк (раздел 11) проверяет цифры против установленного Pomatez.

| Метрика | Цель |
|---|---|
| PSS в простое, окно открыто (software renderer) | ≤ 30 МБ |
| PSS, окно в трее | ≤ 20 МБ |
| CPU, таймер идёт, окно в трее | ~0%, пробуждения только на события |
| CPU, окно видно, анимация выкл. | одна смена свойства и перерисовка в секунду |
| CPU, окно видно, анимация вкл. | плавная дуга только пока окно видно |
| Холодный старт до первого кадра | ≤ 250 мс |
| Размер бинаря (release, stripped) | ≤ 10 МБ |
| WASM (gzip) | ≤ 5 МБ |
| Дрейф таймера за 50 мин | 0 (дедлайны, не подсчёт тиков) |

Без tokio, без webview, без сети на старте.

---

## 2. Платформы и матрица функций

| Функция | Windows | macOS | Linux | Web | Android |
|---|---|---|---|---|---|
| Таймер, раунды, special breaks, strict mode | ✅ | ✅ | ✅ | ✅ | ✅ |
| Задачи | ✅ | ✅ | ✅ | ✅ | ✅ |
| Звук, голос | ✅ | ✅ | ✅ | ✅ (после первого клика) | ✅ |
| Уведомления | ✅ | ✅ | ✅ | ✅ Notification API | ✅ через Java-прослойку |
| Трей + прогресс в трее | ✅ | ✅ | ✅ | — | — (постоянное уведомление с обратным отсчётом) |
| **Мини-окно поверх всех окон** | ✅ | ✅ | ✅* | — | — |
| Fullscreen-перерыв | ✅ | ✅ | ✅ | оверлей в канвасе | полноэкранный оверлей |
| Глобальные хоткеи | ✅ | ✅ | X11 ✅, Wayland — | — | — |
| Автозапуск, проверка обновлений, одна копия | ✅ | ✅ | ✅ | — | — |

\* На Wayland always-on-top зависит от композитора. Опцию показывать с пояснением «недоступно в этой сессии»; не падать.

Веб-сборка Slint официально позиционируется как демо-уровень — для живого примера на сайте это ровно то, что нужно; в README так и написать: «веб-версия — демо, основная — десктоп».

Неподдерживаемое на платформе — скрыто в Settings, а не сломано.

---

## 3. Стек

Последние стабильные версии, default-features отключать, где можно.

| Задача | Крейт / инструмент | Примечание |
|---|---|---|
| UI | `slint`, `slint-build` | `.slint` компилируются в Rust в `build.rs` |
| Рендерер desktop | `renderer-software` (по умолчанию), `renderer-femtovg` (фича) | Software не создаёт GL-контекст → заметно меньше памяти на маленьком окне; поддерживает частичную перерисовку. Оба варианта сравниваются в бенчмарке, по итогам выбрать дефолт. Skia не использовать (размер). |
| Бэкенд окон | `backend-winit` | + доступ к winit-окну через `WinitWindowAccessor` для drag и always-on-top |
| Web | `slint` (winit + femtovg/WebGL) + `wasm-bindgen`, `web-sys` | |
| Android | Android-бэкенд Slint (`android-activity`), `jni`, `cargo-ndk` + Gradle | |
| i18n | `@tr()` в `.slint` + gettext `.po`, бандл переводов через `slint-build` | en, ru; работает и в вебе, и на Android |
| Сериализация | `serde`, `serde_json` | |
| Пути (desktop) | `directories` | |
| Уведомления (desktop) | `notify-rust` | |
| Звук (desktop, Android) | `rodio` | только `wav`, `vorbis`; вывод открывается лениво |
| Трей Win/macOS | `tray-icon` + `muda` | |
| Трей Linux | `ksni` | без GTK |
| Глобальные хоткеи | `global-hotkey` | |
| Автозапуск | `auto-launch` | |
| Тема системы | `Palette.color-scheme` Slint или `dark-light` | |
| Обновления | `ureq` (rustls) + `semver` | |
| Одна копия | `single-instance` | |
| Web-сборка | `trunk` или `wasm-pack` | |
| Упаковка desktop | `cargo-packager` | AppImage, dmg, NSIS |

---

## 4. Архитектура

### 4.1 Крейты

```
crates/
  heller-core/      # чистая логика без IO: FSM таймера, конфиг, задачи, расписание
  heller-ui/        # ui/*.slint + Rust-склейка (модели, колбэки, тема, i18n), трейт Platform
  heller-desktop/   # bin: Windows/macOS/Linux — трей, хоткеи, мини-окно, автозапуск
  heller-web/       # cdylib, index.html
  heller-android/   # cdylib + android/ (Gradle-проект с Java-прослойкой)
xtask/              # пакеты, ассеты, запуск бенчмарка
bench/              # замеры против Pomatez
site/               # GitHub Pages: лендинг + веб-демо
```

### 4.2 Трейт `Platform`

```rust
trait Platform {
    fn notify(&self, n: Notification);
    fn play(&self, sound: SoundId);
    fn speak(&self, phrase: VoiceId);
    fn load(&self, key: &str) -> Option<String>;
    fn save(&self, key: &str, data: &str);
    fn capabilities(&self) -> Caps;            // что показывать в Settings
    fn on_state_changed(&self, s: &Snapshot);  // трей, Android-будильники
    fn open_url(&self, url: &str);
}
```

### 4.3 Рантайм таймера — на event loop Slint

Slint — retained mode: перерисовка только при изменении свойств, а event loop продолжает жить при скрытом окне (`slint::run_event_loop_until_quit()`). Поэтому отдельный поток таймера не нужен:

- Ядро живёт в `Rc<RefCell<Core>>` в UI-потоке.
- Один `slint::Timer` в режиме single-shot заводится на `core.next_wakeup()`; при срабатывании — `core.handle(Tick, now)`, применить эффекты, обновить свойства, перезавести таймер. Одинаково на desktop и в вебе.
- Трей, хоткеи, вторая копия, потоки аудио шлют команды через `slint::invoke_from_event_loop` / `Weak::upgrade_in_event_loop`.
- **Android**: в фоне процесс может быть остановлен. При каждом изменении состояния `Platform::on_state_changed` ставит через Java-прослойку `AlarmManager.setExactAndAllowWhileIdle` на ближайшее событие; уведомление по будильнику показывает сама прослойка. При возвращении — пересчёт от дедлайна.
- **Web**: `slint::Timer` в скрытой вкладке может срабатывать с задержкой (браузерный троттлинг) — корректность не страдает, только точность уведомления. Указать в README.

Время: дедлайн в `SystemTime`. Если после сна дедлайн в прошлом — обработать все пропущенные переходы разом, уведомить только о последнем.

---

## 5. Ядро (`heller-core`)

### 5.1 Config
- `focus_minutes`, `short_break_minutes`, `long_break_minutes`, `sessions_per_round` — диапазоны и дефолты как в оригинале (ожидаемо 25/5/15/4).
- `special_breaks: [Option<SpecialBreak>; 4]`, `SpecialBreak { time: HH:MM, duration_minutes }`.

### 5.2 Settings
`always_on_top`, `fullscreen_breaks`, `strict_mode`, `auto_start_work`, `minimize_to_tray`, `close_to_tray`, `progress_on_tray`, `progress_animation`, `native_titlebar`, `voice_assistance`, `sound_enabled`, `notification: None|Normal|Extra`, `theme: System|Light|Dark`, `compact_mode`, `mini_mode`, `mini_window_pos`, `open_at_login`, `check_updates`, `language: En|Ru`.

### 5.3 Конечный автомат
- `Phase = Focus | ShortBreak | LongBreak | Special(idx)`; `RunState = Idle | Running { deadline } | Paused { remaining }`; `round: u8`.
- Focus закончился → LongBreak, если раунд последний (`round = 1`), иначе ShortBreak и `round += 1`. Перерыв стартует сам.
- Перерыв закончился → Focus; стартует сам только при `auto_start_work`, иначе `Idle`.
- Special break: в заданное время суток прерывает текущую фазу, длится `duration`, затем → Focus; раунд не меняется; не чаще раза в сутки на слот.
- Команды: `Start`, `Pause`, `Resume`, `Toggle`, `Skip`, `Reset`, `ResetAll`, `ApplyConfig`.
- Strict mode: после старта `Pause/Skip/Reset` игнорируются.
- `next_wakeup(now)` — ближайшее из: смена секунды (только если окно видно), дедлайн, предупреждения Extra, special break, следующий 1% (для трея).
- `handle(input, now) -> Vec<Effect>`, `Effect = Notify | PlaySound | Speak | ShowFullscreen | HideFullscreen | TrayUpdate`.

### 5.4 Уведомления
`None` — ничего. `Normal` — на начале каждого перерыва и фокуса. `Extra` — плюс за 60 с до начала перерыва и за 30 с до его конца. `voice_assistance` добавляет голосовую фразу.

### 5.5 Задачи
Как в Pomatez: списки с карточками (текст, описание, done); добавление, правка, удаление, отметка, перестановка внутри списка и перенос между списками.

### 5.6 Тесты
FSM на `FakeClock`: полный раунд, strict mode, auto-start, special break посреди фокуса, пропуск времени при сне, моменты Extra-уведомлений.

---

## 6. UI на Slint — «как в Pomatez»

### 6.1 Дизайн
1. Из исходников Pomatez выписать в `ui/theme.slint` (`export global Theme`): цвета светлой и тёмной темы, цвета фаз, радиусы, отступы, размеры шрифтов, размеры окна (обычного, компактного). `Theme.dark` переключает палитру.
2. Шрифт оригинала импортировать в `.slint` (`import "fonts/….ttf";`), проверить кириллицу, при нехватке — fallback-шрифт. Лицензию шрифта в `THIRD_PARTY.md`.
3. SVG-иконки оригинала подключать через `@image-url(...)`, перекрашивать через `colorize`. Для software renderer включить в `slint-build` встраивание ресурсов с предрастеризацией текстур, чтобы не тащить растеризацию SVG в рантайм (если версия Slint это позволяет для winit-бэкенда; иначе оставить SVG).
4. **Не использовать стили std-widgets** (fluent/material выглядят иначе). Свои компоненты в `ui/widgets/`: `IconButton`, `Toggle`, `Slider`, `RingProgress`, `NavBar`, `TitleBar`, `TaskCard`, `TextField` (на базе `TextInput`). Из std-widgets — только `AboutSlint` и, при необходимости, `ScrollView`.
5. Анимации — декларативные `animate` в `.slint`, короткие.

### 6.2 Окна и компоненты
- `MainWindow` (`ui/main.slint`): навигация Tasks / Config / Timer / Settings, как в оригинале.
- **TitleBar**: кастомный; кнопки свернуть / закрыть / компакт / мини; перетаскивание окна через `drag_window()` winit-окна. `native_titlebar` → свойство `no-frame` у `Window` = false, кастомный заголовок скрыт.
- **Timer**: `RingProgress` — `Path` с дугой, угол из свойства `progress`, цвет фазы; `MM:SS`, название фазы, индикатор раундов, reset / play-pause / skip, звук вкл/выкл.
  - Плавность без нагрузки: Rust обновляет `progress` раз в секунду, а в `.slint` стоит `animate progress { duration: 1s; easing: linear; }` — Slint сам интерполирует дугу. При `progress_animation = false` анимация отключается условием, дуга шагает раз в секунду.
- **Compact mode** (как в оригинале): уменьшенное окно с полосой прогресса.
- **Tasks**: `VecModel` списков и карточек, `for`-циклы. Перестановка — собственный drag через `TouchArea` (pressed + moved, вычисление индекса по координате); дублировать действием «Переместить в…» в меню карточки.
- **Config**: слайдеры длительностей и раундов, special breaks, «по умолчанию».
- **Settings**: все переключатели, уведомления, тема, язык; раздел **«О программе»** с версией, ссылками и обязательным виджетом **`AboutSlint`**.
- **FullscreenBreak** — отдельный компонент `Window`: `no-frame: true`, `always-on-top: true`, показ через `set_fullscreen(true)` на основном мониторе. Кнопка «Пропустить» скрыта в strict mode.

### 6.3 Мини-окно поверх всех окон
- Режим главного окна (не второе окно — экономия памяти): размер ~200×56, `no-frame: true`, `always-on-top: true`, состояние `mini-mode` в `.slint` переключает раскладку.
- Содержимое: тонкое кольцо или полоса цвета фазы, `MM:SS`, play/pause при наведении.
- Перетаскивание за любую точку (`drag_window()`), позиция запоминается (`window.position()` / `set_position()`).
- Двойной клик (`double-clicked` у `TouchArea`) или кнопка — вернуть полный вид. Глобальный хоткей и пункт в трее для переключения.

### 6.4 Поведение окна
- `close_to_tray`: `window.on_close_requested(|| CloseRequestResponse::KeepWindowShown)` + `hide()`; event loop продолжает работать через `run_event_loop_until_quit()`.
- `minimize_to_tray`: перехват минимизации (через winit-событие/аксессор) → `hide()`.
- `always_on_top`: свойство `always-on-top` окна, привязанное к настройке.

---

## 7. Платформенные интеграции

### Desktop
- **Трей**: меню «Показать/Скрыть», «Мини-окно», «Старт/Пауза», «Пропустить», «О программе», «Выход». Прогресс-иконка в RGBA 32×32 (22×22 на Linux), перерисовка только при изменении на ≥1%. Linux — `ksni` (свой поток, команды через `invoke_from_event_loop`). Win/macOS — `tray-icon`; на macOS создавать иконку в главном потоке после старта event loop (например, из `slint::Timer::single_shot(Duration::ZERO, …)`).
- **Автозапуск** с флагом `--hidden`.
- **Обновления**: через 30 с после старта, фоновый поток, GET `releases/latest`, semver, уведомление + бейдж в Settings. Таймаут 5 с.
- **Одна копия**: второй запуск будит первую и выходит.
- **Portable (Windows)**: файл `portable` рядом с exe → данные в `./data`.
- **CLI** (нужно и бенчмарку): `--hidden`, `--start`, `--mini`, `--bench-first-frame` (печать времени до первого кадра), `--renderer software|femtovg` (через `SLINT_BACKEND`/выбор при старте).

### Web
- `heller-web`: точка входа `#[wasm_bindgen(start)]`, канвас на всю страницу, PWA manifest.
- Хранение — `localStorage`; уведомления — запрос разрешения по кнопке; звук разблокируется первым кликом.
- Fullscreen-перерыв — оверлей в канвасе.

### Android
- `heller-android`: `cdylib` с Android-бэкендом Slint; `android/` — Gradle-проект, `minSdk 26`, ABI `arm64-v8a`, `armeabi-v7a`, `x86_64`; `.so` собираются `cargo-ndk` в `jniLibs`.
- Java-прослойка (`HellerBridge.java` + `AlarmReceiver`, ~150–200 строк): канал уведомлений, `AlarmManager` для событий таймера, постоянное уведомление с обратным отсчётом (`setUsesChronometer` + `setChronometerCountDown`), разрешения `POST_NOTIFICATIONS` и `SCHEDULE_EXACT_ALARM`. Вызовы из Rust через `jni`.
- Хранение — внутренний каталог приложения.
- Экранная клавиатура: Android-бэкенд Slint поддерживает IME для `TextInput` — проверить ввод в задачах.
- Android — самый рискованный пункт; делать после десктопа и веба. Если что-то не собирается — изолировать за `cfg`, оставить TODO, не бросать остальное.

---

## 8. Хранение

- Ключи `settings`, `config`, `tasks` — JSON с полем `version`.
- Desktop/Android: файлы, атомарная запись (temp + rename), дебаунс 500 мс через `slint::Timer`, flush при выходе. Битый файл → `.bak` и дефолты.
- Web: `localStorage`.

---

## 9. Производительность

1. Retained mode: свойства обновляются только при реальной смене значения (сравнивать перед `set_*`), секундный тик — только пока окно видно.
2. Таймер на дедлайнах, один single-shot `slint::Timer` до ближайшего события; никаких интервальных тиков в трее.
3. Software renderer по умолчанию (без GL-контекста), femtovg — альтернатива; выбор по бенчмарку.
4. Иконка трея — только при смене процента или фазы.
5. Аудио-устройство не держится открытым; проигрывание в коротком потоке.
6. Минимум потоков: UI + трей (Linux) + эпизодические (аудио, обновления).
7. Ассеты сжаты (Ogg Vorbis) и встроены.
8. Release-профиль:
   ```toml
   [profile.release]
   opt-level = 3
   lto = "fat"
   codegen-units = 1
   panic = "abort"
   strip = true

   [profile.release-web]
   inherits = "release"
   opt-level = "z"
   ```
   Плюс `wasm-opt -Oz`.

---

## 10. Сайт на GitHub Pages

`site/` — статический сайт без фреймворков, в фирменных цветах, светлая/тёмная тема, адаптив.

- `/` — лендинг: название, слоган, «Попробовать в браузере», скриншоты (плейсхолдеры до первого релиза), функции, **таблица и график сравнения с Pomatez** (из `site/bench/latest.json`), кнопки загрузки — JS берёт ассеты последнего релиза из GitHub API и подсвечивает подходящий для ОС посетителя.
- **Бейдж «Made with Slint»** рядом с кнопками загрузки (условие лицензии).
- `/app/` — живое веб-демо.
- Строка про Хеллера и «Уловку-22» в футере.
- `.github/workflows/pages.yml`: на push в `main` собрать wasm, положить в `site/app/`, задеплоить через `actions/deploy-pages`.

---

## 11. Бенчмарк против Pomatez

Запускается пользователем у себя, где Pomatez установлен. `bench/bench.py` (Python 3 + `psutil`, кроссплатформенный) + `bench/README.md`.

### Метрики
| Метрика | Как |
|---|---|
| Память | **PSS** (Linux) или USS (Windows/macOS) и RSS — **суммой по всему дереву процессов** (у Electron: main, renderer, GPU, утилиты) |
| CPU | средний % за окно замера, по дереву |
| Пробуждения | Linux: дельта `voluntary_ctxt_switches` по `/proc/<pid>/task/*/status` в секунду |
| Старт | до появления окна: Linux — `xdotool search --sync --pid`, Windows — опрос `EnumWindows` через `ctypes`; для HellerTimer ещё `--bench-first-frame` |
| Размер | установленное приложение на диске и размер пакета |
| Процессы и потоки | по дереву |

### Сценарии
1. Холодный старт ×5, медиана.
2. Простой с открытым окном, 2 мин (после 30 с прогрева).
3. Таймер идёт, окно видно, анимация вкл. — 2 мин.
4. То же, анимация выкл. — 2 мин.
5. Таймер идёт, окно в трее — 2 мин.
6. Только HellerTimer: мини-окно — 2 мин.

HellerTimer прогоняется **дважды: software и femtovg renderer**. Им управляют флаги; Pomatez — нет, поэтому скрипт делает паузу и просит нажать нужное в его окне. Настройки обоих приложений выровнять по чек-листу из README.

```
python bench/bench.py --heller ./target/release/hellertimer --pomatez /usr/bin/pomatez
python bench/bench.py --pomatez-attach pomatez   # snap/flatpak: найти дерево по имени процесса
```

### Результат
- `bench/results/<host>-<date>.json` — сырые данные + система (ОС, CPU, RAM, версии приложений).
- `bench/report.py` → `.md` с таблицей («в N раз меньше») и SVG-график; `--publish` копирует JSON в `site/bench/latest.json` и обновляет README между `<!-- bench:start -->` / `<!-- bench:end -->`.

---

## 12. Сборка и релизы

`.github/workflows/release.yml`, по тегу `v*`:

| Артефакт | Runner / таргет |
|---|---|
| Linux x86_64: AppImage, `.deb`, `.tar.gz` | `ubuntu-24.04`, `x86_64-unknown-linux-gnu` |
| Linux aarch64: AppImage, `.deb`, `.tar.gz` | `ubuntu-24.04-arm`, `aarch64-unknown-linux-gnu` |
| Windows x86_64: NSIS-инсталлятор, portable `.zip` | `windows-latest`, `x86_64-pc-windows-msvc` |
| Windows ARM64: NSIS-инсталлятор, portable `.zip` | `windows-11-arm`, `aarch64-pc-windows-msvc` |
| macOS Apple Silicon (M1+): `.dmg` | `macos-latest`, `aarch64-apple-darwin` |
| macOS Intel: `.dmg` | кросс-сборка на arm-runner, `x86_64-apple-darwin` |
| macOS Universal: `.dmg` | `lipo` из двух предыдущих |
| Android: `.apk` (split по ABI + universal) | `ubuntu-24.04`, NDK, `cargo-ndk` + Gradle |
| Web: `.zip` со статикой | wasm-сборка |

- AppImage, dmg, NSIS — через `cargo-packager` (`[package.metadata.packager]` в `heller-desktop`).
- Portable Windows — zip: `hellertimer.exe` + пустой `portable` + `LICENSE` + `THIRD_PARTY.md`.
- Имена: `HellerTimer-<версия>-<os>-<arch>.<ext>`; `SHA256SUMS.txt`.
- macOS: ad-hoc `codesign -s -`; при секретах `APPLE_*` — подпись и нотаризация, иначе шаг пропускается, в README — как обойти Gatekeeper.
- Android: debug-подпись, если нет секрета с keystore.
- Linux-раннерам поставить системные зависимости winit/Slint (`libxkbcommon-dev`, `libwayland-dev`, `libfontconfig-dev` и т.п. по документации Slint).
- Flatpak-манифест в `packaging/flatpak/` (вне CI).

---

## 13. Порядок реализации (для one-shot)

1. Workspace, профили, MIT `LICENSE`, `THIRD_PARTY.md` (Pomatez, Slint Royalty-free 2.0, шрифты), `.gitignore`, README-каркас с бейджем Made with Slint.
2. `heller-core` + тесты.
3. `heller-ui`: `theme.slint`, шрифт, иконки, свои виджеты, все экраны, `AboutSlint`, i18n en/ru, Rust-склейка, трейт `Platform`, рантайм на `slint::Timer`.
4. `heller-desktop`: хранение, эффекты, окно (трей, close/minimize to tray, always-on-top, native titlebar), **мини-окно**, fullscreen-перерыв, хоткеи, автозапуск, одна копия, обновления, portable, CLI-флаги, переключение рендерера.
5. `heller-web`.
6. `heller-android` + Java-прослойка + Gradle.
7. Ассеты (звуки, голос, иконки приложения для всех платформ).
8. `site/` и `pages.yml`.
9. `bench/`.
10. `release.yml`, конфиг cargo-packager.
11. `cargo check` для `x86_64-unknown-linux-gnu`, `wasm32-unknown-unknown`, `aarch64-linux-android` (если таргеты установлены) и `cargo clippy --workspace -- -D warnings`; исправить всё.
12. README: функции, матрица платформ, установка, сборка из исходников, бенчмарк, благодарность Pomatez, бейдж Slint.

---

## 14. Definition of Done

- Все функции из README Pomatez работают на десктопе; дизайн визуально совпадает с оригиналом в светлой и тёмной теме.
- Мини-окно поверх всех окон работает на Windows, macOS и Linux X11.
- `AboutSlint` есть в «О программе»; бейдж Made with Slint — на сайте рядом с загрузками и в README.
- Веб-демо открывается на GitHub Pages, таймер и задачи работают, данные сохраняются.
- Android-сборка запускается, таймер корректен после сворачивания, уведомления приходят по будильнику, клавиатура вызывается в задачах.
- Таймер корректно идёт со скрытым окном и после сна системы.
- Бенчмарк запускается одной командой, сравнивает оба рендерера с Pomatez; сайт и README подхватывают результат.
- Workflow собирает все артефакты из раздела 12.
- Нет `unwrap()` на путях IO/платформы; `cargo check` и `clippy -D warnings` чистые.

## 15. Вне объёма

Статистика/история сессий, синхронизация, iOS, fullscreen-перерывы на всех мониторах (TODO; в оригинале тоже один), self-update бинаря, импорт данных из Pomatez, виджет рабочего стола Android, мини-окно в вебе.
