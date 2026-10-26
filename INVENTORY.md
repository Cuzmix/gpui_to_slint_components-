# Inventory: Ely-GPUI-Components → Slint 1.18.1

Source: https://github.com/ELY-Open-Source/Ely-GPUI-Components (shallow clone, default branch, 2026-10-26).

## Scale of the source

- 31 crates, about 296,000 lines of Rust. `ely-core` alone is 80,000 lines (chapters 1–14); `ely-gallery`, the story app, is 43,000.
- The source's own task lists (`tasks/ch01-10.md` … `tasks/ch44.md`) name 1,166 entries across 44 chapters. About 460 are aliases ("→ one home"), so roughly 700 distinct components remain.
- Every component exists in light and dark, plus a high-contrast variant of each palette and a color-blind-safe chart palette.
- The story app (`crates/ely-gallery`, index in `crates/ely-gallery/stories.json`) has one page per chapter and one story per component. Step 5 mirrors it page for page.

## Crate dependency graph (source)

| Crate | Depends on (Ely crates) | Lines |
|---|---|---|
| ely-core | none | 80,181 |
| ely-charts, ely-calendar, ely-canvas, ely-account, ely-dashboard, ely-files, ely-interaction, ely-rendering | core | 296 – 9,383 |
| ely-editor | core, charts | 11,252 |
| ely-finance, ely-debug | core, charts | 9,852 / 3,003 |
| ely-documents | core, editor | 29,589 |
| ely-git, ely-terminal | core, editor | 5,527 / 3,379 |
| ely-chat | core, documents, editor | 7,173 |
| ely-collab | core, documents | 2,952 |
| ely-settings, ely-maps | core, canvas | 3,717 / 3,955 |
| ely-mail | core, chat | 4,737 |
| ely-devtools | core, canvas, chat, editor | 7,696 |
| ely-generative | core, charts, chat, debug, documents | 7,001 |
| ely-agent | core, chat, collab, documents, git | 4,071 |
| ely-media | core, documents, generative | 8,011 |
| ely-messaging, ely-misc | core, media (+canvas) | 4,947 / 5,975 |
| ely-onboarding | core, documents, settings | 2,306 |
| ely-project | core, calendar, documents, mail | 4,879 |
| ely-tooling, ely-desktop | core, devtools | 2,438 / 2,401 |

## Shared foundations (port first)

| Piece | Source | What it holds | Slint plan |
|---|---|---|---|
| Theme tokens | `ely-core/src/theme/tokens.rs` | Density (compact/standard/comfortable, ±4 px heights, ±2 px padding), ControlSize sm/md/lg = 24/28/32 px, AvatarSize, IconSize 12–32 px × font_scale, TextSize 11–32 px × font_scale, Radius 4/6/8/12 × radius_scale, Elevation raised/floating/modal (separate light and dark shadow stacks), ~60 named measures (switch track 32×18, check 16, slider thumb 16, dialog 440, toast 360, menu 200…) | `export global Theme` with enums `Density`, `ControlSize`, `IconSize`, `TextSize`, `Radius`, `Elevation` and `pure function`s that map them to lengths |
| Palette | `ely-core/src/theme/palette.rs` | 36 semantic colors, 8 chart hues, 16 ANSI colors, 13 syntax colors; light, dark, high-contrast overrides, color-blind-safe chart hues | `Theme.palette` struct chosen by `Theme.mode` + `high-contrast` + `color-blind-safe`; all hex values copied verbatim |
| Theme fade | `theme/mod.rs` | Mode change cross-fades the whole palette over 280 ms (`motion::THEME`); `set_mode_now` cuts it | `animate` on each palette color in `Theme` (gap candidate: Slint cannot animate a struct as one, so each color property animates on its own) |
| Custom palettes | `theme/mod.rs`, `theme/syntax.rs`, ch39 | Owner palette per mode, imported themes, syntax themes | `Theme.custom-light` / `custom-dark` palettes set from Rust or Slint |
| Contrast math | `theme/contrast.rs` | WCAG luminance and contrast grades | Rust (ch39 theme editor only) |
| Motion curves | `ely-core/src/motion/curve.rs` | FAST 120 ms, BASE 200 ms, SLOW 320 ms, THEME 280 ms, NUDGE 4 px, ease-out-cubic, ease-in-out-cubic, damped spring (damping 0.75, ω 9.21), reduced motion = 1 ms and repeating motion held still | `Theme.motion-*` durations honoring `reduced-motion`; Slint's `cubic-bezier` for the cubics; the spring is a gap candidate (Slint has no spring easing; nearest is `ease-out-back` or a custom bezier) |
| Focus ring and scopes | `ely-core/src/primitives/focus.rs` | 1 px focus-colored border, transparent at rest; FocusScope with Tab / Shift-Tab and focus trap; root scope regains focus when the focused element leaves | Internal `FocusRing` component bound to `has-focus`; Slint's own Tab chain; traps via `FocusScope` key handling |
| Pressable / held | `primitives/pressable.rs`, `held.rs` | Press state, long press | `TouchArea.pressed` + `Timer` |
| Tooltip | `primitives/tooltip.rs` | Delay, follow-mouse, rich content, max width 256, cursor offset 18 | `PopupWindow` + `Timer` |
| Icon set | `ely-core/icons` (Lucide 1.48.0, ISC, 307 SVGs), `primitives/icon.rs` | Named icons tinted with one color | Copy the SVGs, `@image-url()` per icon, `colorize` for the tint; an `IconName` enum |
| Fonts | `ely-core/assets/fonts` | Inter 4.1 (regular, medium, semibold, italic), JetBrains Mono, IBM Plex Sans | `import "…ttf";` in `ui/index.slint` |
| Overlays / layers | `primitives/layer.rs`, `backdrop.rs`, `overlays/` | Portal, backdrop scrim, anchored popovers | `PopupWindow` with `close-policy`; backdrop as a full-window `Rectangle` |
| Text input | `ely-core/src/forms/text/` (≈3,500 lines) | Own text editing element: selection, IME, undo, multi-line, masks | Builtin `TextInput` for most; gaps for anything beyond it (undo stack, masks) go to Rust or PORTING_GAPS |
| Scrolling | `layout/scroll.rs`, `layout/scroll/` | ScrollArea, 6 px scrollbar, min thumb 24, shadows | `Flickable` + custom scrollbar component |
| i18n | `ely-core/src/i18n` | 7 built-in locales (en, de, fr, ja, he, zh-CN …), RTL direction | `@tr()` with gettext catalogs; RTL is a gap candidate |
| Animation helpers | `motion/` | Transition, Stagger, Entrance, Presence, Flip, Reorder, Skeleton, Spinner, Shimmer, Pulse, Shake, Confetti… | Pure Slint `animate` / `states`; particles and confetti need a Rust-driven model |

## Port order

Foundations first, then each chapter in the source's own dependency order. A tier only starts once the tiers it imports are done.

1. **Tier 0, foundations**: Theme (tokens, palettes, mode fade), motion constants, Icon, fonts, FocusRing, Pressable, Divider, Tooltip.
2. **Tier 1, ely-core controls** (ch01–05): primitives, typography, layout, shell, buttons. Button and IconButton are the first real components, because the source built them in T00 and everything else uses them.
3. **Tier 2, ely-core forms and overlays** (ch06–10): text inputs, selection controls, date/time (Rust), color, files (Rust), form structure, navigation, menus, overlays, feedback.
4. **Tier 3, ely-core data** (ch11–14): loading and motion, data display, lists and trees (Rust models), tables (Rust models).
5. **Tier 4, verticals on core only**: charts, calendar, canvas, account, dashboard, files, interaction.
6. **Tier 5, verticals on charts/editor**: editor, finance, debug, then documents, git, terminal.
7. **Tier 6, the rest**: chat, collab, settings, maps, mail, devtools, generative, agent, media, messaging, misc, onboarding, project, tooling, theme editing, i18n/a11y.
8. **Gallery**: mirrors `stories.json` as each chapter lands.

## Rust-or-Slint rule used below

- **Pure Slint**: visuals, states, keyboard and mouse can be expressed in `.slint` alone.
- **Slint + Rust**: the component's look is `.slint`, but it needs a Rust model or service the host wires up: date/time math, large or tree data (flattening, sorting, filtering), clipboard, file dialogs, parsing/highlighting, locale formatting, chart geometry, color-space math, encoding (QR, barcode), or a domain model (editor buffer, terminal grid, git graph, map tiles).
- **Rust (host/platform)**: the GPUI entry is a platform call (tray, dock badge, jump list, native menu). It is not a Slint component; it would be host code.
- **Alias**: the source points this entry at another component. No separate port.
- **Not portable**: the source itself marks it blocked.

Entries tagged `gpui-native` were GPUI builtins the source only demonstrates (for example `div()`, `svg()`, `.flex()`); most map to a Slint builtin and need no component of their own.

## Totals

| Plan | Entries |
|---|---|
| Alias | 456 |
| Slint + Rust | 354 |
| Pure Slint | 319 |
| Pure Slint (builtin element) | 29 |
| Rust (host/platform) | 4 |
| Not portable (blocked in GPUI too) | 4 |
| **All entries** | **1166** |

710 entries are real components or builtins; the other 456 are aliases that point at one of them.

## Every entry, by chapter

Columns: the GPUI entry name, its tag in the source task list (`built`, `alias`, `gpui-native`, `host`, `prove`, `blocked`, `open`), the Slint plan, and why Rust is needed when it is. The plan column is a first pass from the entry name and chapter; Step 2 specs confirm or correct each one from the source.

### T01 Primitives (`src/primitives`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Box / View | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Text | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Icon | built | Pure Slint |  |
| IconSet / IconRegistry | built | Pure Slint |  |
| Svg | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Image | built | Pure Slint |  |
| Canvas | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Divider / Separator | built | Pure Slint |  |
| Spacer | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Portal | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Overlay / Backdrop | built | Pure Slint |  |
| Slot | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Show / When | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| For / Each | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Fragment | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| VisuallyHidden | prove | Pure Slint |  |
| FocusRing | built | Pure Slint |  |
| FocusScope / FocusTrap | built | Pure Slint |  |
| ClickOutside | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| HoverArea | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Pressable | built | Pure Slint |  |
| KeyboardHandler | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Measure | built | Pure Slint |  |
| ResizeObserver | alias | Alias | covered by `primitives::Measure` |
| IntersectionObserver | built | Pure Slint |  |
| Clipboard | gpui-native | Rust (host/platform) | GPUI calls the platform; Slint needs winit/platform code in the host |
| Tooltip Trigger | built | Pure Slint |  |

### T02 Typography (`src/typography`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Heading (H1–H6) | built | Pure Slint |  |
| Title / Subtitle | built | Pure Slint |  |
| Paragraph | built | Slint + Rust | scales, ticks and path geometry |
| Label | built | Pure Slint |  |
| Caption | built | Pure Slint |  |
| Overline | built | Pure Slint |  |
| Code | built | Slint + Rust | parsing / highlighting |
| Kbd | built | Pure Slint |  |
| KbdCombo | built | Pure Slint |  |
| Blockquote | built | Pure Slint |  |
| Highlight / Mark | built | Slint + Rust | parsing / highlighting |
| Truncate | alias | Alias | covered by `typography::Ellipsis` |
| LineClamp | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| MiddleEllipsis | built | Pure Slint |  |
| EllipsisTooltip | built | Pure Slint |  |
| Link | built | Pure Slint |  |
| ExternalLink | built | Pure Slint |  |
| RelativeTime | built | Slint + Rust | date/time math |
| DateTimeText | built | Slint + Rust | date/time math |
| NumberText | built | Slint + Rust | locale-aware formatting / validation |
| CurrencyText | built | Slint + Rust | locale-aware formatting / validation |
| PercentText | built | Slint + Rust | locale-aware formatting / validation |
| FileSizeText | built | Slint + Rust | file dialogs / filesystem; locale-aware formatting / validation |
| DurationText | built | Slint + Rust | date/time math |
| PluralText | built | Slint + Rust | locale-aware formatting / validation |
| CopyableText | built | Slint + Rust | clipboard |
| SelectableText | prove | Slint + Rust | large data / flattening / sorting |
| Emoji | built | Slint + Rust | emoji database |
| Latex / MathInline | alias | Alias | covered by `documents::Latex` |
| AnimatedNumber | built | Slint + Rust | locale-aware formatting / validation |
| Typewriter | built | Pure Slint |  |
| GradientText | built | Slint + Rust | color-space conversion (HSV/RGB) |
| ShimmerText | built | Pure Slint |  |

### T03 Layout (`src/layout`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Stack / HStack / VStack | built | Pure Slint |  |
| Flex | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Grid / GridItem | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| SimpleGrid | built | Slint + Rust | large data / flattening / sorting |
| Masonry | built | Pure Slint |  |
| Center | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Container | built | Pure Slint |  |
| AspectRatio | built | Pure Slint |  |
| Inset / Padding | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Wrap | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Absolute / Positioned | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Layer / ZStack | built | Pure Slint |  |
| ScrollArea | built | Pure Slint |  |
| Scrollbar | built | Pure Slint |  |
| ScrollShadow | built | Pure Slint |  |
| ScrollToTop | built | Pure Slint |  |
| StickyHeader | built | Pure Slint |  |
| SplitPane | built | Pure Slint |  |
| ResizablePanel / ResizablePanelGroup | alias | Alias | covered by `layout::SplitPane` |
| ResizeHandle | built | Pure Slint |  |
| Collapsible | built | Pure Slint |  |
| Accordion / AccordionItem | built | Pure Slint |  |
| Card / CardHeader / CardBody / CardFooter | built | Pure Slint |  |
| Panel / PanelHeader | built | Pure Slint |  |
| Section / SectionHeader | built | Pure Slint |  |
| Group / Fieldset | built | Pure Slint |  |
| Frame | built | Pure Slint |  |
| Well | built | Pure Slint |  |
| Sidebar | built | Pure Slint |  |
| Drawer | built | Slint + Rust | freehand path building |
| Sheet | built | Slint + Rust | large data / flattening / sorting |
| Page / PageHeader / PageContent | built | Pure Slint |  |
| AppShell | built | Pure Slint |  |
| MasterDetail | built | Pure Slint |  |
| ThreeColumnLayout | alias | Alias | covered by `shell::ColumnShell` |
| Dock / DockPanel | built | Pure Slint |  |
| DockZone / DropIndicator | built | Pure Slint |  |
| FloatingPanel | built | Pure Slint |  |
| Pane / PaneGroup | built | Pure Slint |  |
| Workspace | built | Pure Slint |  |
| Viewport | built | Pure Slint |  |

### T04 Window & Shell (`src/shell`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| TitleBar | built | Pure Slint |  |
| ColumnShell / ShellColumn | built | Pure Slint |  |
| WindowControls | built | Slint + Rust | platform/window API |
| TrafficLights | alias | Alias | covered by `shell::WindowControls` |
| WindowDragRegion | built | Slint + Rust | platform/window API |
| WindowResizeBorder | built | Slint + Rust | platform/window API |
| MenuBar | alias | Alias | covered by `menus::MenuBar` |
| NativeMenu | gpui-native | Rust (host/platform) | GPUI calls the platform; Slint needs winit/platform code in the host |
| Toolbar / ToolbarGroup / ToolbarSeparator | built | Pure Slint |  |
| StatusBar / StatusBarItem | built | Pure Slint |  |
| ActivityBar | built | Pure Slint |  |
| NavigationRail | alias | Alias | covered by `shell::ActivityBar` |
| TabBar | built | Pure Slint |  |
| SplashScreen | built | Pure Slint |  |
| AboutDialog | built | Pure Slint |  |
| TrayIcon / TrayMenu | built | Slint + Rust | platform/window API |
| DockBadge | built | Slint + Rust | platform/window API |
| JumpList | gpui-native | Rust (host/platform) | GPUI calls the platform; Slint needs winit/platform code in the host |
| SystemNotification | prove | Slint + Rust | platform/window API |
| MultiWindow Manager | built | Slint + Rust | platform/window API |
| WindowSwitcher | built | Slint + Rust | fuzzy filtering of a model; platform/window API |
| QuickLauncher | alias | Alias | covered by `navigation::QuickLauncher` |
| MiniWindow / CompactMode | built | Slint + Rust | platform/window API |
| AlwaysOnTop | gpui-native | Rust (host/platform) | GPUI calls the platform; Slint needs winit/platform code in the host |
| PictureInPicture | host | Slint + Rust | platform/window API |
| UpdateBanner / UpdateDialog | built | Slint + Rust | date/time math |
| CrashReporter | built | Pure Slint |  |
| OfflineIndicator | built | Pure Slint |  |
| ZoomControl | built | Pure Slint |  |

### T05 Buttons & Actions (`src/buttons`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Button | built | Pure Slint |  |
| IconButton | built | Pure Slint |  |
| ButtonGroup | built | Pure Slint |  |
| SplitButton | alias | Alias | covered by `menus::SplitButton` |
| DropdownButton / MenuButton | alias | Alias | covered by `menus::DropdownMenu` |
| ToggleButton | built | Pure Slint |  |
| ToggleGroup | built | Pure Slint |  |
| SegmentedControl | built | Pure Slint |  |
| FloatingActionButton | built | Pure Slint |  |
| LoadingButton | alias | Alias | covered by `buttons::Button` |
| ConfirmButton | built | Pure Slint |  |
| HoldToConfirm | alias | Alias | covered by `buttons::ConfirmButton` |
| CopyButton | built | Slint + Rust | clipboard |
| ShareButton | built | Pure Slint |  |
| CloseButton | built | Pure Slint |  |
| BackButton | built | Pure Slint |  |
| MoreButton | built | Pure Slint |  |
| ActionBar | built | Pure Slint |  |
| BulkActionBar | built | Pure Slint |  |
| ActionSheet | built | Slint + Rust | large data / flattening / sorting |
| QuickActions | built | Pure Slint |  |
| ShortcutHint | alias | Alias | covered by `buttons::Button::shortcut` |

### T06a Forms · Text (`src/forms`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Input / TextField | built | Pure Slint |  |
| TextArea | alias | Alias | covered by `forms::TextInput::multi_line` |
| PasswordInput | built | Pure Slint |  |
| SearchInput | built | Slint + Rust | fuzzy filtering of a model |
| NumberInput | built | Slint + Rust | locale-aware formatting / validation |
| ScrubInput | built | Pure Slint |  |
| CurrencyInput | alias | Alias | covered by `forms::NumberInput::currency` |
| PercentInput | alias | Alias | covered by `forms::NumberInput::percent` |
| UnitInput | built | Slint + Rust | locale-aware formatting / validation |
| EmailInput | built | Slint + Rust | locale-aware formatting / validation |
| UrlInput | built | Slint + Rust | locale-aware formatting / validation |
| PhoneInput | built | Slint + Rust | locale-aware formatting / validation |
| MaskedInput | built | Slint + Rust | locale-aware formatting / validation |
| PinInput / OTPInput | built | Pure Slint |  |
| InlineEdit / EditableText | built | Slint + Rust | large data / flattening / sorting |
| InputGroup / InputAddon | built | Pure Slint |  |
| ClearableInput | alias | Alias | covered by `forms::Input::clearable` |
| AutoResizeInput | alias | Alias | covered by `forms::TextInput::multi_line` |
| MentionInput | built | Slint + Rust | fuzzy filtering of a model |
| HashtagInput | alias | Alias | covered by `forms::MentionInput` |
| TagInput / TokenInput | built | Pure Slint |  |
| KeyValueInput | built | Pure Slint |  |
| ListInput | built | Pure Slint |  |
| PathInput | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| RegexInput | built | Slint + Rust | parsing / highlighting |
| ExpressionInput | built | Slint + Rust | parsing / highlighting |
| HotkeyInput / ShortcutRecorder | built | Pure Slint |  |

### T06b Forms · Selection (`src/forms`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Checkbox | built | Pure Slint |  |
| CheckboxGroup | built | Pure Slint |  |
| CheckboxCard | built | Pure Slint |  |
| Radio / RadioGroup | built | Pure Slint |  |
| RadioCard | built | Pure Slint |  |
| Switch / Toggle | built | Pure Slint |  |
| Select | built | Pure Slint |  |
| NativeSelect | blocked | Not portable (blocked in GPUI too) |  |
| Combobox | built | Slint + Rust | fuzzy filtering of a model |
| Autocomplete | alias | Alias | covered by `forms::Combobox::free` |
| MultiSelect | built | Pure Slint |  |
| CascadeSelect / Cascader | built | Pure Slint |  |
| TreeSelect | alias | Alias | covered by `lists::TreeSelect` |
| TransferList | built | Slint + Rust | large data / flattening / sorting |
| ListBox | built | Pure Slint |  |
| ChoiceChips / FilterChips | built | Slint + Rust | fuzzy filtering of a model |
| Rating | built | Pure Slint |  |
| Slider | built | Pure Slint |  |
| RangeSlider | built | Pure Slint |  |
| VerticalSlider | alias | Alias | covered by `forms::Slider::vertical` |
| Knob / Dial | built | Pure Slint |  |
| Stepper | built | Pure Slint |  |

### T06c Forms · Date & Time (`src/forms`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Calendar | built | Slint + Rust | date/time math |
| DatePicker | built | Slint + Rust | date/time math |
| DateRangePicker | built | Slint + Rust | date/time math |
| TimePicker | built | Slint + Rust | date/time math |
| DateTimePicker | built | Slint + Rust | date/time math |
| MonthPicker / YearPicker / QuarterPicker | built | Slint + Rust | date/time math |
| WeekPicker | built | Slint + Rust | date/time math |
| RelativeDatePicker | built | Slint + Rust | date/time math |
| TimezoneSelect | built | Slint + Rust | date/time math |
| DurationPicker | built | Slint + Rust | date/time math |
| CronEditor | built | Slint + Rust | date/time math |

### T06d Forms · Color (`src/forms`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ColorPicker | built | Slint + Rust | color-space conversion (HSV/RGB) |
| ColorSwatch | alias | Alias | covered by `forms::ColorSwatch` |
| ColorPalette | alias | Alias | covered by `forms::ColorPalette` |
| EyeDropper | prove | Slint + Rust | color-space conversion (HSV/RGB) |
| GradientEditor | alias | Alias | covered by `forms::GradientEditor` |

### T06e Forms · Files (`src/forms`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| FilePicker / FileInput | alias | Alias | covered by `forms::FileInput` |
| FolderPicker | alias | Alias | covered by `forms::PathInput::directories` |
| DropZone | alias | Alias | covered by `forms::DropZone` |
| UploadList | alias | Alias | covered by `forms::UploadList` |
| ImageUpload | built | Slint + Rust | file dialogs / filesystem |
| AvatarUpload | alias | Alias | covered by `forms::AvatarUpload` |

### T06f Forms · Other (`src/forms`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| SignaturePad | alias | Alias | covered by `forms::SignaturePad` |
| CodeInput | alias | Alias | covered by `forms::CodeInput` |
| JsonInput | alias | Alias | covered by `forms::JsonInput` |
| FontPicker | alias | Alias | covered by `forms::FontPicker` |
| IconPicker | alias | Alias | covered by `forms::IconPicker` |
| EmojiPicker | alias | Alias | covered by `forms::EmojiPicker` |
| LanguageSelect | alias | Alias | covered by `forms::Combobox::languages` |
| CountrySelect | alias | Alias | covered by `forms::Combobox::countries` |
| CurrencySelect | alias | Alias | covered by `forms::Combobox::currencies` |

### T06g Forms · Structure (`src/forms`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Form | alias | Alias | covered by `forms::Form` |
| FormField | alias | Alias | covered by `forms::FormField` |
| FormLabel | built | Pure Slint |  |
| FormDescription / HelperText | alias | Alias | covered by `typography::Caption` |
| FormError / ValidationMessage | alias | Alias | covered by `forms::FormError` |
| FormSection | alias | Alias | covered by `forms::FormSection` |
| FormActions | alias | Alias | covered by `buttons::ActionBar` |
| FieldArray | alias | Alias | covered by `forms::FieldArray` |
| InlineForm | alias | Alias | covered by `forms::InlineForm` |
| WizardForm / MultiStepForm | alias | Alias | covered by another entry |
| DirtyIndicator | alias | Alias | covered by `forms::DirtyIndicator` |

### T07a Navigation · Tabs & paths (`src/navigation`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Tabs | built | Pure Slint |  |
| EditorTabs | alias | Alias | covered by `navigation::EditorTabs` |
| TabOverflowMenu | alias | Alias | covered by `navigation::TabOverflowMenu` |
| Breadcrumb | alias | Alias | covered by `navigation::Breadcrumb` |
| Pagination | alias | Alias | covered by `navigation::Pagination` |
| LoadMore | alias | Alias | covered by another entry |
| Stepper / Steps / Wizard | alias | Alias | covered by `navigation::Steps` |

### T07b Navigation · Places (`src/navigation`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Sidebar Navigation / NavItem / NavGroup | alias | Alias | covered by `navigation::NavItem` |
| NavigationMenu | alias | Alias | covered by `navigation::NavigationMenu` |
| ActivityBar Item | alias | Alias | covered by `shell::ActivityBar` |
| BackForwardNavigation | alias | Alias | covered by `navigation::BackForwardNavigation` |
| History Navigation | alias | Alias | covered by `navigation::BackForwardNavigation` |
| Anchor / TableOfContents | alias | Alias | covered by `navigation::Anchor` |
| JumpTo / GoToLine | alias | Alias | covered by `navigation::GoToLine` |

### T07c Navigation · Palettes & tour (`src/navigation`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| CommandPalette | alias | Alias | covered by `navigation::CommandPalette` |
| QuickOpen | alias | Alias | covered by `navigation::QuickOpen` |
| QuickSwitcher | alias | Alias | covered by `navigation::QuickSwitcher` |
| SearchPalette | alias | Alias | covered by `navigation::SearchPalette` |
| SpotlightSearch | alias | Alias | covered by `navigation::SearchPalette` |
| Tour / Onboarding | alias | Alias | covered by `overlays::Tour` |

### T08a Menus · Menu & hosts (`src/menus`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Menu / MenuItem / MenuGroup / MenuSeparator | alias | Alias | covered by `menus::Menu` |
| SubMenu | alias | Alias | covered by `menus::MenuItem::submenu` |
| CheckboxMenuItem | alias | Alias | covered by `menus::MenuItem::check` |
| RadioMenuItem | alias | Alias | covered by `menus::MenuItem::radio` |
| MenuItemShortcut | alias | Alias | covered by `menus::MenuItem::keys` |
| MenuItemIcon | alias | Alias | covered by `menus::MenuItem::icon` |
| ContextMenu | alias | Alias | covered by `menus::ContextMenu` |
| DropdownMenu | alias | Alias | covered by `menus::DropdownMenu` |
| OverflowMenu | alias | Alias | covered by `menus::OverflowMenu` |

### T08b Menus · Bar, search & pie (`src/menus`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| MenuBar Menu | alias | Alias | covered by `menus::MenuBar` |
| MegaMenu | alias | Alias | covered by `navigation::NavigationMenu` |
| SearchableMenu | alias | Alias | covered by `menus::SearchableMenu` |
| PieMenu / RadialMenu | alias | Alias | covered by `menus::PieMenu` |

### T09a Overlays · Popovers & dialogs (`src/overlays`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Tooltip | alias | Alias | covered by `primitives::Tooltip` |
| Popover | alias | Alias | covered by `overlays::Popover` |
| HoverCard | alias | Alias | covered by `overlays::HoverCard` |
| Dialog / Modal | alias | Alias | covered by `overlays::Dialog` |
| AlertDialog | alias | Alias | covered by `overlays::AlertDialog` |
| ConfirmDialog | alias | Alias | covered by `overlays::ConfirmDialog` |
| PromptDialog | alias | Alias | covered by `overlays::PromptDialog` |
| FullscreenDialog | alias | Alias | covered by `overlays::Dialog::fullscreen` |
| Sheet | alias | Alias | covered by `layout::Sheet` |
| Drawer | alias | Alias | covered by `layout::Drawer` |

### T09b Overlays · Guides & floats (`src/overlays`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Lightbox | alias | Alias | covered by `overlays::Lightbox` |
| Spotlight / Coachmark | alias | Alias | covered by `overlays::Spotlight` |
| FloatingToolbar | alias | Alias | covered by `overlays::FloatingToolbar` |
| Callout / Peek | alias | Alias | covered by `overlays::Peek` |
| DropdownPanel | alias | Alias | covered by `overlays::Popover` |
| InlinePopup | alias | Alias | covered by `overlays::Popover` |
| ToastViewport | alias | Alias | covered by another entry |

### T10a Feedback · Messages (`src/feedback`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Toast | built | Pure Slint |  |
| Notification | alias | Alias | covered by `feedback::Notification` |
| NotificationCenter | alias | Alias | covered by `feedback::NotificationCenter` |
| Alert | alias | Alias | covered by `feedback::Alert` |
| Banner | alias | Alias | covered by `feedback::Banner` |
| Callout / Admonition | alias | Alias | covered by `feedback::Callout` |
| InlineMessage | alias | Alias | covered by `feedback::InlineMessage` |
| StatusMessage | alias | Alias | covered by `feedback::StatusMessage` |
| Snackbar | alias | Alias | covered by `feedback::Toast` |
| UndoToast | alias | Alias | covered by `feedback::Toast` |

### T10b Feedback · States (`src/feedback`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ErrorView | alias | Alias | covered by `feedback::ErrorView` |
| ErrorBoundary | built | Pure Slint |  |
| NotFound / 404 | alias | Alias | covered by `feedback::ErrorView::not_found` |
| NoPermission / 403 | alias | Alias | covered by `feedback::ErrorView::no_permission` |
| Maintenance | alias | Alias | covered by `feedback::ErrorView::maintenance` |
| Offline | alias | Alias | covered by `feedback::ErrorView::offline` |
| EmptyState | alias | Alias | covered by `feedback::EmptyState` |
| Result | alias | Alias | covered by `feedback::ResultView` |
| ConfirmationCard | alias | Alias | covered by `feedback::ConfirmationCard` |
| SavingIndicator | alias | Alias | covered by `feedback::SavingIndicator` |
| SyncStatus | alias | Alias | covered by `feedback::SyncStatus` |
| ConnectionStatus | alias | Alias | covered by `feedback::ConnectionStatus` |
| Countdown | alias | Alias | covered by `feedback::Countdown` |
| Timer | alias | Alias | covered by `feedback::Timer` |

### T11a Loading · Spinners & skeletons (`src/motion`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Spinner | built | Pure Slint |  |
| DotsLoader | alias | Alias | covered by `motion::Spinner` |
| BarsLoader | alias | Alias | covered by `motion::Spinner` |
| PulseLoader | alias | Alias | covered by `motion::Spinner` |
| RingLoader | alias | Alias | covered by `motion::Spinner` |
| OrbitLoader | alias | Alias | covered by `motion::Spinner` |
| WaveLoader | alias | Alias | covered by `motion::Spinner` |
| Skeleton / SkeletonText / SkeletonAvatar / SkeletonCard / SkeletonTable | alias | Alias | covered by `motion::Skeleton` |
| Shimmer | alias | Alias | covered by `motion::Shimmer` |

### T11b Loading · Progress & states (`src/motion`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ProgressBar | built | Pure Slint |  |
| ProgressCircle / ProgressRing | alias | Alias | covered by `motion::ProgressRing` |
| StepProgress | alias | Alias | covered by `navigation::Steps` |
| LoadingOverlay | alias | Alias | covered by `motion::LoadingOverlay` |
| LoadingScreen | alias | Alias | covered by `shell::SplashScreen` |
| InlineLoader | alias | Alias | covered by `motion::Spinner` |
| LazyLoad | alias | Alias | covered by `motion::LazyLoad` |
| Suspense / AsyncView | alias | Alias | covered by `feedback::Suspense` |
| RefreshIndicator | alias | Alias | covered by `motion::RefreshIndicator` |
| TypingIndicator | alias | Alias | covered by `motion::TypingIndicator` |

### T11c Motion · Presence & order (`src/motion`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Transition | blocked | Not portable (blocked in GPUI too) |  |
| AnimatePresence | alias | Alias | covered by `motion::AnimatePresence` |
| Motion | alias | Alias | covered by another entry |
| Spring | alias | Alias | covered by `motion::spring` |
| Stagger | alias | Alias | covered by `motion::Stagger` |
| Reorder | alias | Alias | covered by `motion::Reorder` |
| Flip | alias | Alias | covered by `motion::Flip` |
| CountUp | alias | Alias | covered by `typography::AnimatedNumber` |

### T11d Motion · Effects (`src/motion`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Marquee | alias | Alias | covered by `motion::Marquee` |
| Ripple | alias | Alias | covered by `motion::Ripple` |
| Confetti | alias | Alias | covered by `motion::Confetti` |
| Glow / Pulse | alias | Alias | covered by `motion::Glow` |
| Blink / Flash | alias | Alias | covered by `motion::Blink` |
| Shake | alias | Alias | covered by `motion::Shake` |
| ParticleBackground | alias | Alias | covered by `motion::ParticleBackground` |
| AnimatedGradient | built | Slint + Rust | color-space conversion (HSV/RGB) |
| Lottie Player | blocked | Not portable (blocked in GPUI too) |  |

### T12a Data Display · People & numbers (`src/data_display`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Badge / CountBadge / DotBadge | alias | Alias | covered by `data_display::Badge` |
| Tag / Chip | alias | Alias | covered by `data_display::Tag` |
| Avatar | built | Pure Slint |  |
| AvatarGroup | alias | Alias | covered by `data_display::AvatarGroup` |
| UserChip | alias | Alias | covered by `data_display::UserChip` |
| Statistic / KPI Card | alias | Alias | covered by `data_display::Statistic` |
| Metric | alias | Alias | covered by `data_display::Statistic` |
| TrendIndicator | alias | Alias | covered by `data_display::TrendIndicator` |

### T12b Data Display · Records (`src/data_display`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| DescriptionList | alias | Alias | covered by `data_display::DescriptionList` |
| PropertyGrid | alias | Alias | covered by `data_display::PropertyGrid` |
| KeyValueList | alias | Alias | covered by `data_display::DescriptionList` |
| InfoRow | alias | Alias | covered by `data_display::DescriptionList` |
| Timeline / TimelineItem | alias | Alias | covered by `data_display::Timeline` |
| ActivityFeed | alias | Alias | covered by `data_display::ActivityFeed` |
| Changelog | alias | Alias | covered by `data_display::Changelog` |

### T12c Data Display · Media (`src/data_display`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Carousel | alias | Alias | covered by `data_display::Carousel` |
| Gallery | alias | Alias | covered by `data_display::Gallery` |
| Accordion list | alias | Alias | covered by `layout::Accordion` |
| Card Grid | alias | Alias | covered by `layout::SimpleGrid` |
| BeforeAfter | alias | Alias | covered by `data_display::BeforeAfter` |
| Watermark | alias | Alias | covered by `data_display::Watermark` |

### T12d Data Display · Codes & measures (`src/data_display`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| QRCode | alias | Alias | covered by `data_display::QrCode` |
| Barcode | alias | Alias | covered by `data_display::Barcode` |
| ColorSwatch display | alias | Alias | covered by `forms::ColorSwatch` |
| Meter | alias | Alias | covered by `data_display::Meter` |
| Gauge | alias | Alias | covered by `data_display::Gauge` |
| UsageBar | alias | Alias | covered by `data_display::UsageBar` |
| Rating display | alias | Alias | covered by `data_display::Stars` |
| Comparison | alias | Alias | covered by `data_display::Comparison` |

### T13a Lists & Trees · Lists (`src/lists`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| List / ListItem | alias | Alias | covered by `lists::List` |
| VirtualList | alias | Alias | covered by `lists::VirtualList` |
| InfiniteList | alias | Alias | covered by `lists::InfiniteList` |
| SortableList | alias | Alias | covered by `lists::SortableList` |
| SelectableList | built | Slint + Rust | large data / flattening / sorting |
| GroupedList | alias | Alias | covered by `lists::GroupedList` |
| SectionList | alias | Alias | covered by `lists::GroupedList` |
| ReorderableList | alias | Alias | covered by `lists::SortableList` |
| SwipeableListItem | alias | Alias | covered by `lists::SwipeableListItem` |

### T13b Lists & Trees · Trees (`src/lists`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Tree / TreeView | alias | Alias | covered by `lists::Tree` |
| VirtualTree | alias | Alias | covered by `lists::Tree` |
| CheckboxTree | alias | Alias | covered by `lists::Tree` |
| Outline | alias | Alias | covered by `lists::Outline` |
| NestedList | alias | Alias | covered by `lists::Outline` |

### T13c Lists & Trees · Files (`src/lists`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| FileTree | alias | Alias | covered by `lists::FileTree` |
| DirectoryListing | alias | Alias | covered by `lists::DirectoryListing` |
| Menu List | alias | Alias | covered by `menus::Menu` |
| Transfer List | alias | Alias | covered by `forms::TransferList` |

### T14a Tables · Core (`src/tables`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Table | alias | Alias | covered by `tables::Table` |
| DataTable | built | Slint + Rust | large data / flattening / sorting |
| VirtualTable | alias | Alias | covered by `tables::DataTable` |
| CellRenderer | alias | Alias | covered by `tables::Cell` |
| RowSelection | alias | Alias | covered by `tables::DataTable` |
| TableDensity | alias | Alias | covered by `tables::DataTable` |
| AggregationFooter | alias | Alias | covered by `tables::Column` |
| ConditionalFormatting | alias | Alias | covered by `tables::Column` |
| HeatmapTable | alias | Alias | covered by `tables::HeatmapTable` |
| ComparisonTable | alias | Alias | covered by `data_display::Comparison` |

### T14b Tables · Columns & rows (`src/tables`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ColumnResizer | alias | Alias | covered by `tables::DataTable` |
| ColumnReorder | alias | Alias | covered by `tables::DataTable` |
| ColumnVisibility | alias | Alias | covered by `tables::DataTable` |
| ColumnPinning | alias | Alias | covered by `tables::Column` |
| RowExpansion | alias | Alias | covered by `tables::DataTable` |
| RowGrouping | alias | Alias | covered by `tables::DataTable` |
| InlineRowEdit | alias | Alias | covered by `tables::Column` |
| TableToolbar | alias | Alias | covered by `tables::TableToolbar` |
| FilterBuilder | alias | Alias | covered by `tables::FilterBuilder` |
| SortBuilder | alias | Alias | covered by `tables::SortBuilder` |

### T14c Tables · Grids (`src/tables`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| DataGrid | alias | Alias | covered by `tables::DataGrid` |
| Spreadsheet | built | Slint + Rust | large data / flattening / sorting |
| TreeTable | alias | Alias | covered by `tables::TreeTable` |
| PivotTable | alias | Alias | covered by `tables::PivotTable` |

### T15a Charts · Cartesian (`src/charts`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| LineChart | built | Slint + Rust | scales, ticks and path geometry |
| AreaChart / StackedArea | alias | Alias | covered by `charts::AreaChart::stacked` |
| BarChart | built | Slint + Rust | scales, ticks and path geometry |
| ScatterChart | built | Slint + Rust | scales, ticks and path geometry |
| BubbleChart | alias | Alias | covered by `charts::ScatterChart` |
| Sparkline / SparkBar / SparkArea | alias | Alias | covered by `data_display::Sparkline` |
| RealtimeChart | built | Slint + Rust | date/time math; scales, ticks and path geometry |
| ChartLegend | built | Slint + Rust | scales, ticks and path geometry |
| ChartTooltip | built | Slint + Rust | scales, ticks and path geometry |
| ChartCrosshair | alias | Alias | covered by another entry |
| ChartAxis / ChartGrid | alias | Alias | covered by another entry |
| ChartZoom / Brush | alias | Alias | covered by `LineChart::zoom` |
| ChartAnnotation | alias | Alias | covered by `rule` |
| ChartExport | built | Slint + Rust | scales, ticks and path geometry |

### T15b Charts · Parts and spreads (`src/charts`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| PieChart / DonutChart | alias | Alias | covered by `charts::PieChart::donut` |
| RadarChart | built | Slint + Rust | scales, ticks and path geometry |
| HeatmapChart | built | Slint + Rust | scales, ticks and path geometry |
| CalendarHeatmap | built | Slint + Rust | date/time math; scales, ticks and path geometry |
| Treemap | built | Slint + Rust | large data / flattening / sorting |
| Sunburst | built | Slint + Rust | domain data model (charts) |
| WaterfallChart | built | Slint + Rust | scales, ticks and path geometry |
| BoxPlot | built | Slint + Rust | scales, ticks and path geometry |
| Histogram | built | Slint + Rust | domain data model (charts) |
| ViolinPlot | built | Slint + Rust | scales, ticks and path geometry |

### T15c Charts · Flows and relations (`src/charts`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| SankeyChart | built | Slint + Rust | scales, ticks and path geometry |
| FunnelChart | built | Slint + Rust | scales, ticks and path geometry |
| GanttChart | built | Slint + Rust | scales, ticks and path geometry |
| NetworkGraph / ForceGraph | alias | Alias | covered by `charts::NetworkGraph` |
| ChordDiagram | built | Slint + Rust | domain data model (charts) |
| ParallelCoordinates | built | Slint + Rust | domain data model (charts) |
| Bullet Chart | alias | Alias | covered by `charts::BulletChart` |
| ProgressChart | built | Slint + Rust | scales, ticks and path geometry |

### T16a Finance · Market charts (`src/finance`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| CandlestickChart | built | Slint + Rust | scales, ticks and path geometry |
| OHLCChart | alias | Alias | covered by `finance::CandlestickChart` |
| HeikinAshiChart | alias | Alias | covered by `finance::CandlestickChart` |
| LineQuoteChart | alias | Alias | covered by `finance::CandlestickChart` |
| VolumeChart | alias | Alias | covered by `CandlestickChart::volume` |
| DepthChart | built | Slint + Rust | scales, ticks and path geometry |
| MarketProfile / VolumeProfile | alias | Alias | covered by `CandlestickChart::profile` |
| RenkoChart / PointAndFigure | alias | Alias | covered by `RenkoChart` |
| MultiChartLayout | built | Slint + Rust | scales, ticks and path geometry |
| ChartSync | built | Slint + Rust | scales, ticks and path geometry |
| TimeRangeSelector | built | Slint + Rust | date/time math |
| IntervalSelector | built | Slint + Rust | domain data model (finance) |
| ChartTypeSwitcher | built | Slint + Rust | scales, ticks and path geometry; fuzzy filtering of a model |

### T16b Finance · Technical analysis (`src/finance`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| IndicatorOverlay | built | Slint + Rust | domain data model (finance) |
| IndicatorPane | built | Slint + Rust | domain data model (finance) |
| IndicatorSelector | built | Slint + Rust | domain data model (finance) |
| DrawingTools | alias | Alias | covered by `CandlestickChart::tool` |
| DrawingToolbar | built | Slint + Rust | freehand path building |
| CompareSymbol | alias | Alias | covered by `CandlestickChart::compare` |
| PriceAlertLine | alias | Alias | covered by `CandlestickChart::alert` |

### T16c Finance · Quotes & book (`src/finance`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Ticker / TickerTape | alias | Alias | covered by `TickerTape` |
| QuoteCard | built | Slint + Rust | domain data model (finance) |
| PriceText | built | Slint + Rust | domain data model (finance) |
| PriceChangeBadge | built | Slint + Rust | domain data model (finance) |
| OrderBook | built | Slint + Rust | domain data model (finance) |
| Level2 Quotes | alias | Alias | covered by `Level2Quotes` |
| TimeAndSales / TradeTape | alias | Alias | covered by `TimeAndSales` |
| MarketDepthLadder / DOM | alias | Alias | covered by `DomLadder` |
| SpreadIndicator | built | Slint + Rust | domain data model (finance) |
| BidAskBar | built | Slint + Rust | domain data model (finance) |

### T16d Finance · Trading (`src/finance`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| OrderEntry / TradePanel | alias | Alias | covered by `OrderEntry` |
| QuickTradeButtons | built | Slint + Rust | domain data model (finance) |
| PositionTable | built | Slint + Rust | large data / flattening / sorting |
| OrderTable | built | Slint + Rust | large data / flattening / sorting |
| TradeHistoryTable | built | Slint + Rust | large data / flattening / sorting |
| PnLDisplay | built | Slint + Rust | domain data model (finance) |
| LeverageSlider | built | Slint + Rust | domain data model (finance) |
| MarginIndicator | built | Slint + Rust | domain data model (finance) |
| RiskMeter | built | Slint + Rust | domain data model (finance) |
| OrderConfirmDialog | built | Slint + Rust | domain data model (finance) |
| OptionChain | built | Slint + Rust | domain data model (finance) |
| GreeksTable | built | Slint + Rust | large data / flattening / sorting |
| PayoffDiagram | built | Slint + Rust | domain data model (finance) |

### T16e Finance · Markets & assets (`src/finance`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Watchlist | built | Slint + Rust | domain data model (finance) |
| SymbolSearch | built | Slint + Rust | fuzzy filtering of a model |
| SymbolBadge | built | Slint + Rust | domain data model (finance) |
| MarketHeatmap | built | Slint + Rust | scales, ticks and path geometry |
| SectorTreemap | alias | Alias | covered by `finance::MarketHeatmap` |
| Screener / FilterPanel | alias | Alias | covered by `Screener` |
| MarketOverview | built | Slint + Rust | domain data model (finance) |
| IndexCard | alias | Alias | covered by `finance::QuoteCard` |
| MarketStatus | built | Slint + Rust | domain data model (finance) |
| TradingSessionClock | built | Slint + Rust | date/time math |
| EconomicCalendar | built | Slint + Rust | date/time math |
| EarningsCalendar | built | Slint + Rust | date/time math |
| NewsFeed | built | Slint + Rust | domain data model (finance) |
| SentimentGauge | alias | Alias | covered by `data_display::Gauge` |
| PortfolioSummary | built | Slint + Rust | domain data model (finance) |
| AssetAllocationChart | alias | Alias | covered by `charts::PieChart` |
| PerformanceChart | built | Slint + Rust | scales, ticks and path geometry |
| DividendTable | built | Slint + Rust | large data / flattening / sorting |
| FinancialStatementTable | built | Slint + Rust | large data / flattening / sorting |
| CurrencyConverter | built | Slint + Rust | locale-aware formatting / validation |
| CryptoWalletCard | built | Slint + Rust | domain data model (finance) |
| TransactionList | built | Slint + Rust | domain data model (finance) |
| Candle Countdown | alias | Alias | covered by `feedback::Countdown` |

### T17a Editor · Core (`src/editor`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| CodeEditor | built | Slint + Rust | parsing / highlighting |
| EditorGutter | alias | Alias | covered by `editor::CodeEditor` |
| LineNumbers | built | Slint + Rust | locale-aware formatting / validation |
| FoldingControl | alias | Alias | covered by `CodeEditor` |
| Minimap | alias | Alias | covered by `CodeEditor::minimap` |
| Ruler / IndentGuide | alias | Alias | covered by `CodeEditor::rulers` |
| BracketMatcher / RainbowBrackets | alias | Alias | covered by `CodeEditor::rainbow_brackets` |
| WhitespaceRenderer | alias | Alias | covered by `CodeEditor::show_whitespace` |
| CursorRenderer | built | Slint + Rust | domain data model (editor) |
| SelectionHighlight | alias | Alias | covered by `CodeEditor` |
| MultiCursor | alias | Alias | covered by `CodeEditor` |
| VimModeIndicator | built | Slint + Rust | domain data model (editor) |
| StickyScroll | alias | Alias | covered by `CodeEditor::sticky_scroll` |
| Breadcrumb | alias | Alias | covered by `navigation::Breadcrumb` |
| InlayHints | alias | Alias | covered by `CodeEditor::set_inlay_hints` |
| CodeLens | alias | Alias | covered by `CodeEditor::set_code_lenses` |
| InlineDecoration | alias | Alias | covered by `CodeEditor::set_diagnostics` |
| GhostText | alias | Alias | covered by `CodeEditor::set_ghost_text` |
| InlineChat | alias | Alias | covered by `CodeEditor::set_chat` |
| InlineDiff | alias | Alias | covered by `CodeEditor::set_diff` |
| ReadOnlyBanner | built | Slint + Rust | domain data model (editor) |

### T17b Editor · Intelligence (`src/editor`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| CompletionMenu | built | Slint + Rust | domain data model (editor) |
| SignatureHelp | built | Slint + Rust | freehand path building |
| HoverInfo | built | Slint + Rust | domain data model (editor) |
| QuickFix / CodeActionMenu | alias | Alias | covered by `editor::CodeActionMenu` |
| RenameInput | built | Slint + Rust | domain data model (editor) |
| PeekView | alias | Alias | covered by `overlays::Peek` |
| ReferencesPanel | built | Slint + Rust | domain data model (editor) |
| SymbolOutline | built | Slint + Rust | domain data model (editor) |
| GoToSymbol | built | Slint + Rust | domain data model (editor) |
| CallHierarchy / TypeHierarchy | alias | Alias | covered by `editor::CallHierarchy` |

### T17c Editor · Search (`src/editor`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| FindWidget | built | Slint + Rust | domain data model (editor) |
| ReplaceWidget | alias | Alias | covered by `editor::FindWidget::replace` |
| SearchPanel | built | Slint + Rust | fuzzy filtering of a model |
| SearchResultItem | built | Slint + Rust | fuzzy filtering of a model |
| SearchFilters | built | Slint + Rust | fuzzy filtering of a model |

### T17d Editor · Panels (`src/editor`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ProblemsPanel / DiagnosticsList | alias | Alias | covered by `editor::ProblemsPanel` |
| OutputPanel | built | Slint + Rust | domain data model (editor) |
| DebugConsole | built | Slint + Rust | domain data model (editor) |
| ExtensionsPanel / PluginList | alias | Alias | covered by `editor::ExtensionsPanel` |
| SettingsEditor | built | Slint + Rust | domain data model (editor) |
| KeybindingsEditor | built | Slint + Rust | domain data model (editor) |
| ThemePreview | built | Slint + Rust | domain data model (editor) |
| WelcomePage | built | Slint + Rust | domain data model (editor) |
| RecentProjects | built | Slint + Rust | domain data model (editor) |
| ProjectSwitcher | built | Slint + Rust | fuzzy filtering of a model |
| TaskRunner / RunConfiguration | alias | Alias | covered by `editor::TaskRunner` |

### T17e Editor · Status items (`src/editor`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| CursorPosition | built | Slint + Rust | domain data model (editor) |
| LanguageMode | built | Slint + Rust | domain data model (editor) |
| Encoding | built | Slint + Rust | domain data model (editor) |
| LineEnding | built | Slint + Rust | domain data model (editor) |
| IndentSettings | built | Slint + Rust | domain data model (editor) |
| BranchIndicator | built | Slint + Rust | domain data model (editor) |
| LspStatus | built | Slint + Rust | domain data model (editor) |
| NotificationBell | built | Slint + Rust | domain data model (editor) |

### T18 Terminal (`src/terminal`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Terminal | prove | Slint + Rust | domain data model (terminal) |
| TerminalTabs | alias | Alias | covered by `navigation::EditorTabs` |
| TerminalSplit | alias | Alias | covered by `layout::PaneGroup` |
| TerminalToolbar | built | Slint + Rust | domain data model (terminal) |
| TerminalSearch | alias | Alias | covered by `editor::FindWidget` |
| TerminalLink | alias | Alias | covered by `Terminal` |
| ShellSelector | built | Slint + Rust | domain data model (terminal) |
| CommandBlock | built | Slint + Rust | domain data model (terminal) |
| CommandHistory | built | Slint + Rust | domain data model (terminal) |
| AnsiText | built | Slint + Rust | domain data model (terminal) |
| LogViewer | built | Slint + Rust | domain data model (terminal) |
| ProcessList | built | Slint + Rust | domain data model (terminal) |

### T19 Git (`src/git`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| DiffViewer | built | Slint + Rust | domain data model (git) |
| InlineDiff | alias | Alias | covered by `editor::CodeEditor::set_diff` |
| ThreeWayMerge | built | Slint + Rust | domain data model (git) |
| ConflictResolver | built | Slint + Rust | domain data model (git) |
| ChangesList | built | Slint + Rust | domain data model (git) |
| CommitInput | built | Slint + Rust | domain data model (git) |
| CommitGraph | alias | Alias | covered by `git::CommitList::graph` |
| CommitList / CommitItem | built | Slint + Rust | domain data model (git) |
| BranchSelector | built | Slint + Rust | domain data model (git) |
| BranchList | built | Slint + Rust | domain data model (git) |
| TagList | built | Slint + Rust | domain data model (git) |
| StashList | built | Slint + Rust | domain data model (git) |
| BlameView / GitBlameAnnotation | built | Slint + Rust | freehand path building |
| FileHistory | built | Slint + Rust | file dialogs / filesystem |
| PullRequestCard | built | Slint + Rust | domain data model (git) |
| ReviewComment | built | Slint + Rust | domain data model (git) |
| DiffStat | built | Slint + Rust | domain data model (git) |
| GitStatusBadge | built | Slint + Rust | domain data model (git) |

### T20 Debug (`src/debug`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| DebugToolbar | built | Slint + Rust | domain data model (debug) |
| BreakpointList | built | Slint + Rust | domain data model (debug) |
| BreakpointGutter | alias | Alias | covered by `editor::CodeEditor` |
| CallStack | built | Slint + Rust | domain data model (debug) |
| VariablesPanel | built | Slint + Rust | domain data model (debug) |
| WatchPanel | built | Slint + Rust | domain data model (debug) |
| ThreadList | built | Slint + Rust | domain data model (debug) |
| MemoryViewer / HexViewer | alias | Alias | covered by `debug::HexViewer` |
| Disassembly | built | Slint + Rust | domain data model (debug) |
| PerformanceProfiler / Flamegraph | alias | Alias | covered by `debug::Flamegraph` |
| TimelineProfiler | built | Slint + Rust | file dialogs / filesystem |
| NetworkInspector | built | Slint + Rust | domain data model (debug) |
| ConsoleREPL | alias | Alias | covered by `editor::DebugConsole` |

### T21a Documents · Editing (`src/documents`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| RichTextEditor | built | Slint + Rust | domain data model (documents) |
| MarkdownEditor | built | Slint + Rust | parsing / highlighting |
| BlockEditor | built | Slint + Rust | domain data model (documents) |
| Block types | built | Slint + Rust | domain data model (documents) |
| BlockHandle | alias | Alias | covered by another entry |
| SlashMenu | alias | Alias | covered by `/` |
| FloatingFormatToolbar | alias | Alias | covered by `overlays::FloatingToolbar` |
| FixedFormatToolbar | built | Slint + Rust | domain data model (documents) |
| LinkEditor | built | Slint + Rust | domain data model (documents) |
| MentionMenu | alias | Alias | covered by `@` |
| EmojiAutocomplete | alias | Alias | covered by another entry |
| TableEditor | alias | Alias | covered by another entry |
| ImageEditorBlock | alias | Alias | covered by another entry |
| MathEditor / LatexBlock | alias | Alias | covered by another entry |
| MermaidBlock / DiagramBlock | host | Slint + Rust | domain data model (documents) |
| CodeBlockEditor | alias | Alias | covered by another entry |
| TodoItem / Checklist | built | Slint + Rust | domain data model (documents) |
| PlaceholderText | alias | Alias | covered by another entry |
| WordCount / CharacterCount | alias | Alias | covered by `WordCount` |
| ReadingTime | built | Slint + Rust | date/time math |

### T21b Documents · Reading (`src/documents`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| MarkdownRenderer | built | Slint + Rust | parsing / highlighting |
| DocumentViewer | host | Slint + Rust | domain data model (documents) |
| PDFViewer | host | Slint + Rust | domain data model (documents) |
| EpubReader | host | Slint + Rust | domain data model (documents) |
| OfficePreview | host | Slint + Rust | domain data model (documents) |
| PageThumbnailList | built | Slint + Rust | domain data model (documents) |
| TableOfContents | alias | Alias | covered by `navigation::TableOfContents` |
| DocumentOutline | alias | Alias | covered by `navigation::TableOfContents` |
| Footnote | built | Slint + Rust | domain data model (documents) |
| Glossary / TooltipTerm | built | Slint + Rust | domain data model (documents) |
| ReadingProgress | built | Slint + Rust | domain data model (documents) |
| FocusMode / ZenMode | alias | Alias | covered by `documents::ZenMode` |
| PrintPreview | host | Slint + Rust | domain data model (documents) |

### T21c Documents · Knowledge (`src/documents`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| PageTree | alias | Alias | covered by `lists::Tree` |
| Backlinks | built | Slint + Rust | domain data model (documents) |
| GraphView | alias | Alias | covered by `charts::NetworkGraph` |
| PageCover / PageIcon | built | Slint + Rust | domain data model (documents) |
| PageProperties | alias | Alias | covered by `data_display::PropertyGrid` |
| DatabaseView | built | Slint + Rust | domain data model (documents) |
| TemplatePicker | built | Slint + Rust | domain data model (documents) |
| TrashBin | built | Slint + Rust | domain data model (documents) |
| VersionHistory | built | Slint + Rust | domain data model (documents) |
| PageHistoryDiff | built | Slint + Rust | domain data model (documents) |
| Favorites / Pinned | alias | Alias | covered by `documents::Favorites` |

### T22 Collaboration (`src/collab`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| PresenceAvatars | built | Pure Slint |  |
| RemoteCursor | alias | Alias | covered by another entry |
| RemoteSelection | alias | Alias | covered by another entry |
| CommentThread | built | Pure Slint |  |
| CommentBubble / CommentMarker | alias | Alias | covered by `collab::CommentMarker` |
| CommentSidebar | built | Pure Slint |  |
| Reply / ReactionPicker | built | Pure Slint |  |
| Reactions | built | Pure Slint |  |
| Annotation / Highlight | alias | Alias | covered by `collab::AnnotatedText` |
| SuggestionMode | alias | Alias | covered by another entry |
| TrackChanges | built | Pure Slint |  |
| ShareDialog | alias | Alias | covered by another entry |
| PermissionSelect | built | Pure Slint |  |
| InviteInput | alias | Alias | covered by another entry |
| AccessList | built | Pure Slint |  |
| ActivityLog | alias | Alias | covered by `data_display::ActivityFeed` |
| FollowMode | built | Pure Slint |  |
| LiveIndicator | built | Pure Slint |  |

### T23a AI Chat · Messages (`src/chat`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ChatContainer | built | Pure Slint |  |
| MessageList | built | Pure Slint |  |
| MessageBubble | built | Pure Slint |  |
| MessageAvatar | built | Pure Slint |  |
| MessageHeader | built | Pure Slint |  |
| MessageFooter | built | Pure Slint |  |
| StreamingText | built | Pure Slint |  |
| StreamingMarkdown | built | Slint + Rust | parsing / highlighting |
| StreamingCursor | built | Pure Slint |  |
| CodeBlock | built | Slint + Rust | parsing / highlighting |
| MathBlock | alias | Alias | covered by `documents::Latex` |
| TableBlock | alias | Alias | covered by `documents::MarkdownRenderer` |
| MermaidRenderer | alias | Alias | covered by another entry |
| ImageMessage / ImageGrid | built | Slint + Rust | large data / flattening / sorting |
| FileMessage | built | Slint + Rust | file dialogs / filesystem |
| AudioMessage | built | Pure Slint |  |
| VideoMessage | host | Pure Slint |  |
| LinkPreviewCard | built | Pure Slint |  |
| QuoteReply | built | Pure Slint |  |
| MessageActions | built | Pure Slint |  |
| MessageEditor | built | Pure Slint |  |
| BranchNavigator | built | Pure Slint |  |
| RegenerateButton | alias | Alias | covered by `chat::regenerate_button` |
| StopGeneratingButton | alias | Alias | covered by `chat::stop_button` |
| ContinueButton | alias | Alias | covered by `chat::continue_button` |
| ErrorMessage / RetryMessage | alias | Alias | covered by `chat::ErrorMessage` |
| RateLimitNotice | built | Pure Slint |  |
| FeedbackForm | built | Pure Slint |  |
| ScrollToBottomButton | built | Pure Slint |  |
| DateSeparator | built | Slint + Rust | date/time math |
| ThinkingBlock / ReasoningPanel | alias | Alias | covered by `chat::ThinkingBlock` |
| ThinkingIndicator | built | Pure Slint |  |
| ThinkingDuration | built | Slint + Rust | date/time math |

### T23b AI Chat · Citations (`src/chat`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| CitationBadge / InlineCitation | alias | Alias | covered by `chat::CitationBadge` |
| SourceCard | built | Pure Slint |  |
| SourceList / SourcesPanel | alias | Alias | covered by `chat::SourceList` |
| SearchProgress | built | Slint + Rust | fuzzy filtering of a model |
| WebResultCard | built | Pure Slint |  |
| DocumentChunkPreview | built | Pure Slint |  |

### T23c AI Chat · Input (`src/chat`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| PromptInput / Composer | alias | Alias | covered by `chat::PromptInput` |
| AttachmentButton | built | Pure Slint |  |
| AttachmentChip / AttachmentPreview | alias | Alias | covered by `chat::AttachmentChip` |
| PasteImagePreview | alias | Alias | covered by another entry |
| DragDropOverlay | built | Slint + Rust | file dialogs / filesystem |
| VoiceInputButton | built | Pure Slint |  |
| VoiceWaveform / RecordingIndicator | alias | Alias | covered by `chat::VoiceWaveform` |
| SlashCommandMenu | alias | Alias | covered by `/` |
| PromptTemplateMenu | alias | Alias | covered by `menus::DropdownMenu` |
| ContextMentionMenu | alias | Alias | covered by `@` |
| ContextChips | built | Pure Slint |  |
| ModelSelector | alias | Alias | covered by `forms::Select` |
| ModeSelector | alias | Alias | covered by `buttons::SegmentedControl` |
| ToolToggleMenu | alias | Alias | covered by `menus::DropdownMenu` |
| TemperatureSlider / ParameterPanel | alias | Alias | covered by `chat::ParameterPanel` |
| SystemPromptEditor | built | Pure Slint |  |
| TokenCounter | built | Pure Slint |  |
| ContextWindowMeter | alias | Alias | covered by `data_display::UsageBar` |
| CostEstimator | built | Pure Slint |  |
| SendButton | built | Pure Slint |  |
| InputHint | built | Pure Slint |  |

### T23d AI Chat · Conversations (`src/chat`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ConversationList | built | Pure Slint |  |
| ConversationItem | built | Pure Slint |  |
| ConversationSearch | alias | Alias | covered by `forms::SearchInput` |
| NewChatButton | alias | Alias | covered by `chat::new_chat_button` |
| FolderList / ProjectList | alias | Alias | covered by `chat::ProjectList` |
| ProjectKnowledgePanel | built | Pure Slint |  |
| SharedConversationView | built | Pure Slint |  |
| ConversationExport | built | Pure Slint |  |

### T23e AI Chat · Welcome (`src/chat`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| WelcomeScreen / Greeting | alias | Alias | covered by `chat::WelcomeScreen` |
| SuggestionChips / StarterPrompts | alias | Alias | covered by `chat::SuggestionChips` |
| PromptLibrary | alias | Alias | covered by `documents::TemplatePicker` |
| FollowUpSuggestions | built | Pure Slint |  |
| CapabilityCards | built | Pure Slint |  |

### T24a Agent · Tool calls (`src/agent`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ToolCallCard | built | Pure Slint |  |
| ToolCallGroup | built | Pure Slint |  |
| ToolApprovalDialog | built | Pure Slint |  |
| PermissionPrompt | built | Pure Slint |  |

### T24b Agent · Progress (`src/agent`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| AgentStepList / TaskTimeline | alias | Alias | covered by `agent::AgentStepList` |
| AgentPlan / TodoList | alias | Alias | covered by `agent::AgentPlan` |
| AgentStatus | built | Pure Slint |  |
| AgentProgress | built | Pure Slint |  |
| SubAgentTree | alias | Alias | covered by `lists::Tree` |

### T24c Agent · Changes (`src/agent`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| FileChangeCard | built | Slint + Rust | file dialogs / filesystem |
| MultiFileDiff Review | alias | Alias | covered by `agent::MultiFileDiffReview` |
| CommandExecutionCard | alias | Alias | covered by `terminal::CommandBlock` |

### T24d Agent · Previews (`src/agent`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| BrowserPreview / ScreenshotStream | host | Pure Slint |  |
| ComputerUseViewer | host | Pure Slint |  |
| ArtifactPanel | built | Pure Slint |  |
| ArtifactVersionSwitcher | alias | Alias | covered by `chat::BranchNavigator` |
| LivePreview | host | Pure Slint |  |

### T24e Agent · Environment (`src/agent`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| SandboxStatus | built | Pure Slint |  |
| MCPServerList / ConnectorList | alias | Alias | covered by `agent::McpServerList` |
| ToolRegistry Panel | alias | Alias | covered by `agent::ToolRegistry` |
| MemoryPanel | built | Pure Slint |  |
| CheckpointList / Rewind | alias | Alias | covered by `agent::CheckpointList` |

### T24f Agent · Control and cost (`src/agent`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| InterruptButton | alias | Alias | covered by `chat::stop_button` |
| HumanInputRequest | built | Pure Slint |  |
| TokenUsageChart | alias | Alias | covered by `charts::BarChart` |
| CostBreakdown | built | Pure Slint |  |
| TraceViewer | alias | Alias | covered by `debug::TimelineProfiler` |
| EvalResultTable | alias | Alias | covered by `tables::DataTable` |

### T25a Generative · Prompt and settings (`src/generative`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ImageGenerationPanel | alias | Alias | covered by `forms::Form` |
| PromptEnhancer | built | Pure Slint |  |
| NegativePromptInput | alias | Alias | covered by `forms::TagInput` |
| StylePresetPicker | built | Pure Slint |  |
| SeedInput | built | Pure Slint |  |
| AspectRatioPicker | built | Pure Slint |  |

### T25b Generative · Queue and results (`src/generative`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| GenerationQueue | built | Pure Slint |  |
| GenerationGrid | built | Slint + Rust | large data / flattening / sorting |
| VariationPicker | built | Pure Slint |  |
| ImageCompare | alias | Alias | covered by `data_display::BeforeAfter` |
| ABCompareView | built | Pure Slint |  |

### T25c Generative · Canvas (`src/generative`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| InpaintCanvas | built | Slint + Rust | freehand path building |
| MaskBrush | built | Pure Slint |  |

### T25d Generative · Sound and motion (`src/generative`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| AudioGenerationPlayer | built | Pure Slint |  |
| VideoGenerationTimeline | built | Pure Slint |  |
| TTSVoicePicker | built | Pure Slint |  |

### T25e Generative · Models (`src/generative`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ModelCard | built | Pure Slint |  |
| ModelDownloadManager | built | Pure Slint |  |
| ModelStatus | built | Pure Slint |  |
| HardwareMonitor | built | Pure Slint |  |
| FineTuneJobCard | built | Pure Slint |  |

### T25f Generative · Data and prompts (`src/generative`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| DatasetViewer | alias | Alias | covered by `tables::DataTable` |
| EmbeddingVisualizer | built | Pure Slint |  |
| PromptPlayground | built | Pure Slint |  |
| PromptDiff / PromptVersionHistory | built | Pure Slint |  |

### T26a Media · Images (`src/media`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ImageViewer | built | Slint + Rust | domain data model (media) |
| ImageThumbnail | built | Slint + Rust | domain data model (media) |
| Lightbox / Gallery Viewer | alias | Alias | covered by `overlays::Lightbox` |

### T26b Media · Crop and annotate (`src/media`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ImageCropper | built | Slint + Rust | domain data model (media) |
| ImageAnnotator | built | Slint + Rust | freehand path building |

### T26c Media · Video (`src/media`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| VideoPlayer | host | Slint + Rust | domain data model (media) |
| VideoTimeline / Scrubber | built | Slint + Rust | domain data model (media) |
| VideoThumbnailStrip | built | Slint + Rust | domain data model (media) |
| SubtitleEditor | built | Slint + Rust | domain data model (media) |

### T26d Media · Audio and controls (`src/media`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| AudioPlayer | host | Slint + Rust | domain data model (media) |
| AudioWaveform | built | Slint + Rust | domain data model (media) |
| AudioSpectrum / Visualizer | built | Slint + Rust | domain data model (media) |
| VolumeControl | built | Slint + Rust | domain data model (media) |
| PlaybackSpeedControl | built | Slint + Rust | domain data model (media) |
| MediaControls | built | Slint + Rust | domain data model (media) |
| Playlist | built | Slint + Rust | domain data model (media) |

### T26e Media · Capture and devices (`src/media`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ScreenRecorderControls | built | Slint + Rust | domain data model (media) |
| CameraPreview | host | Slint + Rust | domain data model (media) |
| MicLevelMeter | built | Slint + Rust | domain data model (media) |
| DeviceSelector | built | Slint + Rust | domain data model (media) |
| 3DModelViewer | host | Slint + Rust | domain data model (media) |

### T27a Files · Items (`src/files`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| FileIcon | built | Slint + Rust | file dialogs / filesystem |
| FileItem | built | Slint + Rust | file dialogs / filesystem |
| FileList | alias | Alias | covered by `lists::DirectoryListing` |
| FileGrid | built | Slint + Rust | large data / flattening / sorting; file dialogs / filesystem |

### T27b Files · Explorer (`src/files`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| FileExplorer | built | Slint + Rust | file dialogs / filesystem |
| FileBreadcrumb / PathBar | alias | Alias | covered by `navigation::Breadcrumb` |
| FolderTree | alias | Alias | covered by `lists::FileTree` |
| FileContextMenu | alias | Alias | covered by `menus::ContextMenu` |
| RenameInline | alias | Alias | covered by `forms::InlineEdit` |
| DragDropFiles | alias | Alias | covered by `forms::DropZone` |

### T27c Files · Find and look (`src/files`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| FilePreview / QuickLook | built | Slint + Rust | file dialogs / filesystem |
| RecentFiles | built | Slint + Rust | file dialogs / filesystem |
| Favorites / Bookmarks | alias | Alias | covered by `documents::Favorites` |
| FileSearch | built | Slint + Rust | file dialogs / filesystem; fuzzy filtering of a model |
| DuplicateFinder | built | Pure Slint |  |
| ArchiveViewer | host | Pure Slint |  |
| HexViewer | alias | Alias | covered by `debug::HexViewer` |

### T27d Files · Transfers (`src/files`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| FileOperationProgress | built | Slint + Rust | file dialogs / filesystem |
| TransferQueue | built | Slint + Rust | large data / flattening / sorting |
| DownloadManager | built | Pure Slint |  |
| StorageUsage | alias | Alias | covered by `data_display::UsageBar` |

### T28a Messaging · Channels (`src/messaging`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ChannelList / ChatList | built | Pure Slint |  |
| ChannelItem | built | Pure Slint |  |
| DirectMessageItem | built | Pure Slint |  |
| ChannelHeader | built | Pure Slint |  |
| PinnedMessages | built | Pure Slint |  |

### T28b Messaging · Messages (`src/messaging`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ChatMessage | built | Pure Slint |  |
| MessageThread | built | Pure Slint |  |
| ThreadPanel | built | Pure Slint |  |
| UnreadDivider | built | Pure Slint |  |
| MessageReactions | alias | Alias | covered by `collab::Reactions` |
| ReadReceipt | built | Pure Slint |  |
| TypingIndicator | alias | Alias | covered by `motion::TypingIndicator` |

### T28c Messaging · People (`src/messaging`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| MemberList | built | Pure Slint |  |
| UserProfileCard | built | Slint + Rust | file dialogs / filesystem |
| OnlineStatus | built | Pure Slint |  |
| StatusSetter | built | Pure Slint |  |

### T28d Messaging · Calls (`src/messaging`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| CallControls | built | Pure Slint |  |
| ParticipantTile | built | Pure Slint |  |
| VideoCallGrid | host | Slint + Rust | large data / flattening / sorting |
| ScreenShareView | host | Pure Slint |  |
| VoiceCallBar | built | Pure Slint |  |
| IncomingCallDialog | built | Pure Slint |  |
| HuddleIndicator | built | Pure Slint |  |

### T28e Messaging · Stickers and GIFs (`src/messaging`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| StickerPicker | built | Pure Slint |  |
| GifPicker | host | Pure Slint |  |

### T29a Mail · Boxes and lists (`src/mail`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| MailboxList / FolderList | built | Slint + Rust | file dialogs / filesystem |
| MailList | built | Pure Slint |  |
| MailItem | built | Pure Slint |  |
| InboxZeroState | alias | Alias | covered by `feedback::EmptyState` |

### T29b Mail · Reading (`src/mail`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| MailReader | built | Pure Slint |  |
| MailThreadView | built | Pure Slint |  |
| QuotedText | built | Pure Slint |  |

### T29c Mail · Writing (`src/mail`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| MailComposer | built | Pure Slint |  |
| RecipientInput | alias | Alias | covered by `forms::TagInput` |
| SignatureEditor | built | Slint + Rust | freehand path building |
| ScheduleSend | built | Slint + Rust | date/time math |

### T29d Mail · Sorting (`src/mail`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| LabelPicker | built | Pure Slint |  |
| SnoozePicker | built | Pure Slint |  |

### T30a Calendar · Months (`src/calendar`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| CalendarMonthView | built | Slint + Rust | date/time math |
| YearView | built | Slint + Rust | date/time math |
| MiniCalendar | alias | Alias | covered by `forms::Calendar` |
| EventCard / EventChip | built | Pure Slint |  |

### T30b Calendar · Time grid (`src/calendar`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| CalendarWeekView | built | Slint + Rust | date/time math |
| CalendarDayView | built | Slint + Rust | date/time math |
| TimeGrid | built | Slint + Rust | date/time math; large data / flattening / sorting |
| AllDayRow | built | Pure Slint |  |
| CurrentTimeIndicator | built | Slint + Rust | date/time math |
| TimezoneOverlay | built | Slint + Rust | date/time math |

### T30c Calendar · Agenda and people (`src/calendar`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| AgendaView | built | Slint + Rust | date/time math |
| EventPopover | built | Pure Slint |  |
| AttendeeList | built | Pure Slint |  |

### T30d Calendar · Editing (`src/calendar`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| EventEditor | built | Pure Slint |  |
| RecurrenceEditor | built | Pure Slint |  |
| ReminderPicker | built | Pure Slint |  |
| AvailabilityPicker | built | Pure Slint |  |

### T31a Project · Tasks (`src/project`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| PriorityIndicator | built | Pure Slint |  |
| IssueIdBadge | built | Pure Slint |  |
| StatusSelect | built | Pure Slint |  |
| AssigneePicker | built | Pure Slint |  |
| DueDatePicker | alias | Alias | covered by `forms::DatePicker` |
| TaskItem | built | Pure Slint |  |
| TaskList | built | Pure Slint |  |
| SubtaskList | alias | Alias | covered by `documents::Checklist` |

### T31b Project · Issues and boards (`src/project`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| IssueCard | built | Pure Slint |  |
| LabelManager | built | Pure Slint |  |
| KanbanBoard / KanbanColumn / KanbanCard | built | Pure Slint |  |
| Sprint Board | alias | Alias | covered by `project::KanbanBoard` |
| TaskDetailPanel | built | Pure Slint |  |

### T31c Project · Plans (`src/project`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| GanttChart / TimelineView | alias | Alias | covered by `charts::GanttChart` |
| Roadmap | built | Pure Slint |  |
| Burndown Chart | alias | Alias | covered by `charts::BurndownChart` |
| MilestoneProgress | built | Pure Slint |  |
| WorkloadView | built | Pure Slint |  |

### T31d Project · Time and databases (`src/project`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| TimeTracker | built | Slint + Rust | date/time math |
| Pomodoro Timer | built | Slint + Rust | date/time math |
| DatabaseView (from T21c) | built | Pure Slint |  |

### T32a Canvas · Plane (`src/canvas`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| InfiniteCanvas | built | Slint + Rust | large data / flattening / sorting; freehand path building |
| CanvasGrid / Guides | built | Slint + Rust | large data / flattening / sorting; freehand path building |
| Ruler | built | Slint + Rust | domain data model (canvas) |
| ZoomControls / ZoomIndicator | built | Slint + Rust | domain data model (canvas) |
| MiniMap | built | Slint + Rust | domain data model (canvas) |
| ArtboardFrame | built | Slint + Rust | domain data model (canvas) |

### T32b Canvas · Tools (`src/canvas`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| SelectionBox | built | Slint + Rust | domain data model (canvas) |
| TransformHandles | built | Slint + Rust | domain data model (canvas) |
| SnapIndicator | built | Slint + Rust | domain data model (canvas) |
| ToolPalette | built | Slint + Rust | fuzzy filtering of a model |
| ShapeTools | built | Slint + Rust | domain data model (canvas) |
| PenTool / PathEditor | built | Slint + Rust | file dialogs / filesystem |
| TextTool | built | Slint + Rust | domain data model (canvas) |
| BrushTool / BrushSettings | built | Slint + Rust | domain data model (canvas) |
| HistoryPanel | built | Slint + Rust | domain data model (canvas) |

### T32c Canvas · Panels (`src/canvas`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| LayerPanel | built | Slint + Rust | domain data model (canvas) |
| AssetPanel | built | Slint + Rust | domain data model (canvas) |
| InspectorPanel | built | Slint + Rust | domain data model (canvas) |
| AlignmentToolbar | built | Slint + Rust | domain data model (canvas) |
| ColorPanel | built | Slint + Rust | domain data model (canvas) |
| GradientPanel | alias | Alias | covered by `forms::GradientEditor` |
| ShadowEditor | built | Slint + Rust | domain data model (canvas) |
| BorderEditor | built | Slint + Rust | domain data model (canvas) |
| TypographyPanel | built | Slint + Rust | scales, ticks and path geometry |
| ConstraintsEditor | built | Slint + Rust | domain data model (canvas) |
| AutoLayoutControls | built | Slint + Rust | domain data model (canvas) |
| ExportPanel | built | Slint + Rust | domain data model (canvas) |

### T32d Canvas · Graphs and boards (`src/canvas`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| NodeEditor / NodeGraph | built | Slint + Rust | scales, ticks and path geometry |
| Node / Port / Edge / Connection | built | Slint + Rust | domain data model (canvas) |
| FlowChart / DiagramEditor | built | Slint + Rust | scales, ticks and path geometry |
| MindMap | built | Slint + Rust | domain data model (canvas) |
| Whiteboard | built | Slint + Rust | domain data model (canvas) |
| StickyNote | built | Slint + Rust | domain data model (canvas) |
| Connector Line | built | Slint + Rust | domain data model (canvas) |

### T33a DB & Dev Tools · Databases (`src/devtools`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ConnectionManager | built | Slint + Rust | domain data model (devtools) |
| ConnectionForm | built | Slint + Rust | domain data model (devtools) |
| SchemaTree | built | Slint + Rust | large data / flattening / sorting |
| QueryEditor | alias | Alias | covered by `editor::CodeEditor` |
| QueryResultGrid | alias | Alias | covered by `tables::DataGrid` |
| QueryHistory | built | Slint + Rust | domain data model (devtools) |
| QueryPlanViewer | built | Slint + Rust | domain data model (devtools) |
| TableStructureEditor | built | Slint + Rust | large data / flattening / sorting |
| ERDiagram | built | Slint + Rust | domain data model (devtools) |
| IndexManager | built | Slint + Rust | domain data model (devtools) |
| RedisKeyBrowser | built | Slint + Rust | domain data model (devtools) |

### T33b DB & Dev Tools · Data formats (`src/devtools`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| DocumentViewer (MongoDB / JSON) | alias | Alias | covered by `devtools::JsonViewer` |
| JsonViewer / JsonTree | built | Slint + Rust | large data / flattening / sorting; parsing / highlighting |
| JsonEditor | built | Slint + Rust | parsing / highlighting |
| YamlViewer | built | Slint + Rust | domain data model (devtools) |
| XmlViewer | built | Slint + Rust | domain data model (devtools) |

### T33c DB & Dev Tools · APIs (`src/devtools`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ApiRequestBuilder | built | Slint + Rust | domain data model (devtools) |
| ResponseViewer | built | Slint + Rust | domain data model (devtools) |
| HeadersTable | built | Slint + Rust | large data / flattening / sorting |
| QueryParamsEditor | alias | Alias | covered by `forms::KeyValueInput` |
| EnvironmentSelector | built | Slint + Rust | domain data model (devtools) |
| CollectionTree | built | Slint + Rust | large data / flattening / sorting |
| WebSocketConsole | built | Slint + Rust | domain data model (devtools) |
| GraphQLExplorer | built | Slint + Rust | scales, ticks and path geometry |

### T33d DB & Dev Tools · Tools and machines (`src/devtools`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| RegexTester | built | Slint + Rust | parsing / highlighting |
| Base64 / Encoder | built | Slint + Rust | parsing / highlighting |
| ColorContrastChecker | built | Slint + Rust | domain data model (devtools) |
| ContainerList | built | Slint + Rust | domain data model (devtools) |
| PodList | built | Slint + Rust | domain data model (devtools) |
| ResourceMonitor | built | Slint + Rust | domain data model (devtools) |

### T34 Dashboard (`src/dashboard`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Dashboard Grid | built | Slint + Rust | large data / flattening / sorting |
| Widget / DashboardCard | built | Pure Slint |  |
| StatCard | alias | Alias | covered by `data_display::Statistic` |
| MetricsChart | alias | Alias | covered by `charts::LineChart` |
| AlertList | built | Pure Slint |  |
| IncidentCard | built | Pure Slint |  |
| UptimeBar | built | Slint + Rust | date/time math |
| ServiceStatus | built | Pure Slint |  |
| HealthCheck Indicator | built | Pure Slint |  |
| LogStream | alias | Alias | covered by `terminal::LogViewer` |
| TraceWaterfall | alias | Alias | covered by `debug::TimelineProfiler` |
| EventStream | built | Pure Slint |  |
| ResourceGauge | alias | Alias | covered by `data_display::Gauge` |
| SystemMonitor | alias | Alias | covered by `devtools::ResourceMonitor` |
| ProcessTable | alias | Alias | covered by `terminal::ProcessList` |
| NetworkGraph (throughput) | alias | Alias | covered by `charts::RealtimeChart` |
| MapView / GeoHeatmap | alias | Alias | covered by `maps::MapView` |
| WorldMap | alias | Alias | covered by `maps::WorldMap` |
| RefreshIntervalSelector | built | Pure Slint |  |
| DashboardFilterBar | built | Slint + Rust | fuzzy filtering of a model |

### T35 Settings (`src/settings`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| SettingsLayout | built | Pure Slint |  |
| SettingsSection | built | Pure Slint |  |
| SettingsRow | built | Pure Slint |  |
| SettingsSearch | built | Slint + Rust | fuzzy filtering of a model |
| ThemeSelector | built | Pure Slint |  |
| AccentColorPicker | built | Slint + Rust | color-space conversion (HSV/RGB) |
| FontSizeControl | built | Pure Slint |  |
| FontFamilySelect | alias | Alias | covered by `forms::FontPicker` |
| DensitySelector | built | Pure Slint |  |
| LanguageSelector | alias | Alias | covered by `forms::Combobox::languages` |
| KeyboardShortcutsList | built | Pure Slint |  |
| ShortcutEditor | alias | Alias | covered by `editor::KeybindingsEditor` |
| ProxySettings | built | Pure Slint |  |
| StorageSettings / CacheClear | built | Pure Slint |  |
| PrivacySettings | built | Pure Slint |  |
| NotificationSettings | built | Pure Slint |  |
| StartupSettings | built | Pure Slint |  |
| ImportExportSettings | built | Pure Slint |  |
| ResetToDefault | built | Pure Slint |  |
| AdvancedSettings / FeatureFlags | built | Pure Slint |  |
| DeveloperMode Toggle | built | Pure Slint |  |

### T36a Account · Sign in (`src/account`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| LoginForm | built | Pure Slint |  |
| SignupForm | built | Pure Slint |  |
| OAuthButtons | built | Pure Slint |  |
| MagicLinkForm | built | Pure Slint |  |
| TwoFactorInput | built | Pure Slint |  |
| ForgotPassword | built | Pure Slint |  |

### T36b Account · Profile and access (`src/account`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ProfileCard | built | Slint + Rust | file dialogs / filesystem |
| ProfileEditor | built | Slint + Rust | file dialogs / filesystem |
| AccountSwitcher | built | Slint + Rust | fuzzy filtering of a model |
| WorkspaceSwitcher / OrgSwitcher | built | Slint + Rust | fuzzy filtering of a model |
| UserMenu | built | Pure Slint |  |
| SessionList | built | Pure Slint |  |
| ApiKeyManager | built | Pure Slint |  |

### T36c Account · Billing and team (`src/account`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| SubscriptionCard / PlanBadge | built | Pure Slint |  |
| UsageQuota | built | Pure Slint |  |
| BillingHistory | built | Pure Slint |  |
| PaymentMethodForm | built | Pure Slint |  |
| UpgradePrompt / Paywall | built | Pure Slint |  |
| TeamMemberTable | built | Slint + Rust | large data / flattening / sorting |
| RoleSelector | built | Pure Slint |  |
| InvitationList | built | Pure Slint |  |

### T37a Onboarding · Flows and highlights (`src/onboarding`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| OnboardingWizard | built | Pure Slint |  |
| WelcomeTour | alias | Alias | covered by `overlays::Tour` |
| Coachmark / Hotspot | built | Pure Slint |  |
| FeatureHighlight / NewBadge | built | Slint + Rust | parsing / highlighting |
| Checklist (first-run tasks) | alias | Alias | covered by `onboarding::SetupChecklist` |

### T37b Onboarding · Help (`src/onboarding`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| HelpTooltip | alias | Alias | covered by `primitives::HelpTooltip` |
| HelpPanel / DocsSidebar | built | Pure Slint |  |
| KeyboardShortcutCheatsheet | built | Slint + Rust | large data / flattening / sorting |
| WhatsNewDialog | built | Pure Slint |  |
| FeedbackWidget | built | Pure Slint |  |
| ContactSupport | built | Pure Slint |  |
| InlineHelp | built | Pure Slint |  |
| EmptyStateWithAction | alias | Alias | covered by `feedback::EmptyState` |

### T38a Interaction · Pointer (`src/interaction`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Draggable | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Droppable / DropTarget | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| DragOverlay / DragPreview | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| DragGhost | alias | Alias | covered by another entry |
| Sortable / SortableContext | alias | Alias | covered by `motion::Reorder` |
| Resizable | built | Pure Slint |  |
| Rotatable | built | Slint + Rust | large data / flattening / sorting |
| Selectable / SelectionArea | built | Slint + Rust | large data / flattening / sorting |
| RubberBandSelection | alias | Alias | covered by `interaction::SelectionArea` |
| LongPress | alias | Alias | covered by `primitives::Pressable` |
| DoubleClick | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Gesture | prove | Pure Slint |  |
| PanZoom | alias | Alias | covered by `layout::Viewport` |

### T38b Interaction · Keys, scroll and focus (`src/interaction`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Hotkeys / KeyBinding | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| KeyChord | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| Undo / Redo Manager | alias | Alias | covered by `forms::History` |
| InfiniteScroll Trigger | alias | Alias | covered by `lists::InfiniteList` |
| VirtualScroller | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| ScrollSync | built | Pure Slint |  |
| Autofocus | gpui-native | Pure Slint (builtin element) | Slint has a builtin for it |
| RovingTabIndex | alias | Alias | covered by `interaction::RovingFocus` |
| TypeAhead | alias | Alias | covered by `lists::SelectableList` |
| FocusVisible | alias | Alias | covered by `primitives::FocusRing` |

### T39a Theme · Editing (`src/theme`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ThemeProvider | built | Pure Slint |  |
| ThemeSwitcher | alias | Alias | covered by `settings::ThemeSelector` |
| ColorTokens / DesignTokens | built | Pure Slint |  |
| ThemeEditor | alias | Alias | covered by `settings::ThemeEditor` |
| DensityProvider | built | Pure Slint |  |
| RadiusProvider | built | Pure Slint |  |
| FontProvider | built | Pure Slint |  |
| HighContrastMode | built | Pure Slint |  |
| ReducedMotionProvider | built | Pure Slint |  |
| Elevation / Shadow | built | Pure Slint |  |

### T39b Theme · Importing (`src/theme`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ThemeImporter | built | Pure Slint |  |
| SyntaxThemes | alias | Alias | covered by `theme::syntax_themes` |

### T39c Theme · Platform (`src/theme`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| IconThemeProvider | alias | Alias | covered by `primitives::IconTheme` |
| Platform Style | alias | Alias | covered by `theme::Platform` |
| VibrancyBackground / Acrylic | alias | Alias | covered by `shell::Vibrancy` |
| Mica | prove | Pure Slint |  |

### T40a i18n (`src/i18n`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| I18nProvider | alias | Alias | covered by `i18n::I18n` |
| Trans / LocalizedText | alias | Alias | covered by `I18n::text` |
| LocaleNumber / LocaleDate | alias | Alias | covered by `Locale::number` |
| RTL layout | prove | Pure Slint |  |

### T40b a11y (`src/i18n`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| LiveRegion / Announcer | prove | Pure Slint |  |
| SkipLink | alias | Alias | covered by `i18n::SkipLink` |
| AccessibleIcon | prove | Pure Slint |  |
| AriaLabel helper | prove | Pure Slint |  |
| KeyboardNavigationHelper | alias | Alias | covered by `interaction::RovingFocus` |
| ScreenReaderOnly | alias | Alias | covered by another entry |
| ColorBlindSafe Palette | alias | Alias | covered by `Theme::set_color_blind_safe` |

### T41a Maps · View (`src/maps`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| MapView | host | Slint + Rust | domain data model (maps) |
| MapMarker | alias | Alias | covered by `maps::MapMarker` |
| MapPopup | alias | Alias | covered by `maps::MapPopup` |
| LocationPicker | alias | Alias | covered by `maps::LocationPicker` |
| CoordinateDisplay | alias | Alias | covered by `maps::CoordinateDisplay` |

### T41b Maps · Layers (`src/maps`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| MapCluster | alias | Alias | covered by `MapView::cluster_markers` |
| RouteLine | alias | Alias | covered by `maps::RouteLine` |
| GeoJSON Layer | alias | Alias | covered by `maps::read_geojson` |
| GeoHeatmap (from T34) | alias | Alias | covered by `maps::GeoHeatmap` |

### T41c Maps · World (`src/maps`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ChoroplethMap | alias | Alias | covered by `maps::ChoroplethMap` |
| WorldMap (from T34) | alias | Alias | covered by `maps::WorldMap` |

### T42a Misc · Time (`src/misc`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Clock / WorldClock | alias | Alias | covered by `misc::Clock` |
| Stopwatch | alias | Alias | covered by `misc::Stopwatch` |

### T42b Misc · Numbers (`src/misc`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Calculator | alias | Alias | covered by `misc::Calculator` |
| UnitConverter | alias | Alias | covered by `misc::UnitConverter` |

### T42c Misc · Codes and checks (`src/misc`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| QRCodeScanner | prove | Slint + Rust | parsing / highlighting; encoding |
| BarcodeDisplay | alias | Alias | covered by `data_display::Barcode` |
| Captcha | alias | Alias | covered by `misc::Captcha` |
| Signature | alias | Alias | covered by `forms::SignaturePad` |
| Rating Widget | alias | Alias | covered by `forms::Rating` |

### T42d Misc · Asking and learning (`src/misc`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| Poll / Survey | alias | Alias | covered by `misc::Poll` |
| Quiz / Flashcard | alias | Alias | covered by `misc::Quiz` |

### T42e Misc · Terms and notices (`src/misc`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ChangeLog Viewer | alias | Alias | covered by `data_display::Changelog` |
| LicenseViewer | alias | Alias | covered by `misc::LicenseViewer` |
| Terms / Consent Dialog | alias | Alias | covered by `misc::ConsentDialog` |
| CookieBanner | alias | Alias | covered by `misc::CookieBanner` |

### T42f Misc · Web (`src/misc`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| WebView | prove | Pure Slint |  |
| IframeEmbed / EmbedCard | alias | Alias | covered by `misc::IframeEmbed` |
| HTMLPreview | alias | Alias | covered by `misc::WebView` |
| PrintButton | blocked | Not portable (blocked in GPUI too) |  |
| ShareSheet | alias | Alias | covered by `buttons::ShareButton` |

### T42g Misc · Export and import (`src/misc`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ExportDialog | built | Pure Slint |  |
| ImportDialog | built | Pure Slint |  |
| CSVImporter | alias | Alias | covered by `misc::CsvImporter` |

### T43a Tooling · Workbench (`src/tooling`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| ComponentGallery / Storybook | alias | Alias | covered by `examples/gallery` |
| PlaygroundPanel | alias | Alias | covered by `tooling::Playground` |

### T43b Tooling · Inspection (`src/tooling`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| InspectorOverlay | alias | Alias | covered by `tooling::install_inspector` |
| FPSMeter / PerfOverlay | alias | Alias | covered by `tooling::FpsMeter` |
| RenderCounter | alias | Alias | covered by `tooling::RenderCounter` |
| EventLogger | alias | Alias | covered by `tooling::EventLogger` |

### T43c Tooling · Catalogs (`src/tooling`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| DesignTokenViewer | alias | Alias | covered by `tooling::DesignTokenViewer` |
| IconBrowser | alias | Alias | covered by `forms::IconPicker` |
| A11yChecker | built | Pure Slint |  |
| ScreenshotTest Harness | built | Pure Slint |  |

### T50b Rendering · Surface (`src/rendering`)

| Component | GPUI tag | Slint plan | Rust needed for |
|---|---|---|---|
| RenderSurface | built | Pure Slint |  |
| FramebufferView / OffscreenRenderSurface / CustomRendererSurface / TextureView | alias | Alias | covered by `rendering::RenderSurface` |
| ShaderView / GpuSurface / WebGPUSurface / Compute Visualization / GPU Particle View | built | Pure Slint |  |
| CameraTexture / GStreamerTexture / ExternalTexture | built | Pure Slint |  |
| ZeroCopyTexture / SharedTextureView | built | Slint + Rust | clipboard |
| MetalSurface / OpenGLSurface / VulkanSurface / D3D11/D3D12Surface | built | Pure Slint |  |
| 3D Scene / 3D Viewport | alias | Alias | covered by `media::ModelViewer` |
