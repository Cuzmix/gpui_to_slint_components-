# Porting status

One row per ported component. A component is done only when every column is ✓.
Visuals: compared state by state against the GPUI source values (no GPUI captures exist for Linux; the gallery's own captures are macOS-only), and checked in light and dark screenshots of the Slint gallery (`slint-viewer ui/gallery.slint --screenshot`).

| Component | Source | Props | Callbacks | Keyboard | Mouse/touch | Animation | Visuals | A11y | Tests | Gaps | Done |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Theme (tokens, palettes, fade) | theme/, motion/curve.rs | ✓ | n/a | n/a | n/a | ✓ (ThemeFader) | ✓ | n/a | ✓ `tests/theme.rs` | G1 G2 | ✓ |
| Elevated | theme/tokens.rs `elevation` | ✓ | n/a | n/a | n/a | n/a | ✓ | n/a | ✓ (shadow tokens in `tests/theme.rs`) | G3 | ✓ |
| Icon | primitives/icon.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/primitives.rs` | | ✓ |
| Spinner | motion/spinner.rs | ✓ | n/a | n/a | n/a | ✓ periods, reduced motion | ✓ | ✓ progress indicator | ✓ `tests/primitives.rs` | G5 | ✓ |
| TooltipController / TooltipSurface | primitives/tooltip.rs (`Tooltip`) | ✓ | n/a | n/a | ✓ 500 ms, hide on leave/press/scroll | ✓ 120 ms ease-out rise | ✓ | n/a | ✓ `tests/icon_button.rs` | G6 G7 | ✓ |
| Button | buttons/button.rs | ✓ | ✓ | ✓ Enter/Space rules | ✓ | n/a (GPUI states are instant) | ✓ | ✓ | ✓ `tests/button.rs` | G8 G9 G11 | ✓ |
| IconButton, CloseButton, BackButton, MoreButton | buttons/icon_button.rs | ✓ | ✓ | ✓ | ✓ | n/a | ✓ | ✓ | ✓ `tests/icon_button.rs` | G10 G11 | ✓ |
| Divider, VerticalDivider | primitives/divider.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/divider_pressable.rs` | | ✓ |
| Pressable | primitives/pressable.rs | ✓ | ✓ | n/a | ✓ 500 ms long press, release outside | n/a | n/a | ✓ (no node) | ✓ `tests/divider_pressable.rs` | | ✓ |
| Severity, Severities | primitives/severity.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | n/a | ✓ `tests/primitives2.rs` | | ✓ |
| Disclosure | primitives/disclosure.rs | ✓ | n/a | n/a | n/a | ✓ 120 ms ease-out, still on first paint | ✓ | ✓ (no node) | ✓ `tests/primitives2.rs` | G14 | ✓ |
| DragGhost | primitives/ghost.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | n/a | ✓ `tests/primitives2.rs` | G3 | ✓ |
| Picture (GPUI `Image`) | primitives/image.rs | ✓ | n/a | n/a | n/a | ✓ 1600 ms pulse, reduced motion | ✓ | ✓ (no node) | ✓ `tests/primitives2.rs` | G13 | ✓ |
| Backdrop, Place | primitives/backdrop.rs | ✓ | ✓ | n/a | ✓ dismiss outside children | ✓ 200 ms fade | ✓ | n/a | ✓ `tests/primitives2.rs` | G12 | ✓ |
| FocusRing | primitives/focus.rs | ✓ | n/a | ✓ | n/a | n/a | ✓ | n/a | ✓ `tests/primitives2.rs` | | ✓ |
| FocusTrap (FocusScope `.trap()`) | primitives/focus.rs | ✓ | n/a | ✓ wrap both ways, enter, pass over empty | n/a | n/a | n/a | ✓ (owner sets role/label) | ✓ `tests/focus_trap.rs` | G18 G19 | ✓ |
| Measure, IntersectionObserver | primitives/measure.rs | ✓ | ✓ | n/a | n/a | n/a | n/a | n/a | ✓ `tests/measure.rs` | G16 G17 | ✓ |
| HelpTooltip, TooltipTrigger | primitives/tooltip.rs | ✓ | n/a | n/a | ✓ 500 / 400 ms, hide on leave and press | ✓ (TooltipController) | ✓ | n/a | ✓ `tests/tooltip.rs` (rich content too) | G7 | ✓ |
| VisuallyHidden | tasks/ch01-10.md (GPUI shipped none) | ✓ | n/a | n/a | n/a | n/a | ✓ draws nothing | ✓ text role and label | ✓ `tests/primitives2.rs` | | ✓ |
| KeyStrokes (gpui `Keystroke::unparse`) | gpui platform/keystroke.rs | ✓ | n/a | ✓ | n/a | n/a | n/a | n/a | ✓ `tests/keystroke.rs` | | ✓ |
| Title, Subtitle, Paragraph, Label, Caption, Overline, Heading (TypeText) | typography/text.rs | ✓ | n/a | n/a | n/a | n/a | ✓ sizes, weights, inks, pixel-snapped leading | ✓ text role | ✓ `tests/typography.rs` | G20 | ✓ |
| Code | typography/inline.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/inline.rs` | G20 | ✓ |
| Blockquote | typography/inline.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/inline.rs` | G21 | ✓ |
| KeyCap, Kbd, KbdCombo | typography/inline.rs, keys.rs | ✓ | n/a | n/a | n/a | n/a | ✓ per platform | ✓ each cap reads its label | ✓ `tests/inline.rs` | G20 | ✓ |
| Highlight | typography/inline.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ reads the whole text | ✓ `tests/inline.rs` | G22 | ✓ |
| Ellipsis (Truncate), MiddleEllipsis | typography/fit.rs | ✓ | n/a | n/a | n/a | n/a | ✓ cuts, alignment, nothing when no room | ✓ reads the whole text | ✓ `tests/fit.rs` | G23 | ✓ |
| EllipsisTooltip | typography/fit.rs | ✓ | n/a | n/a | ✓ tip only while cut | ✓ (TooltipController) | ✓ | ✓ reads the whole text | ✓ `tests/fit.rs` | G7 G23 | ✓ |
| LineClamp (gpui `.line_clamp`) | gpui text style | ✓ | n/a | n/a | n/a | n/a | ✓ hard cut | ✓ text role | ✓ `tests/fit.rs` | G29 | ✓ |
| Link, ExternalLink | typography/link.rs | ✓ | ✓ | ✓ Enter/Space rules, Tab stop | ✓ click, no focus on press, hover underline | n/a | ✓ | ✓ pressable, label | ✓ `tests/link.rs` | G9 G24 | ✓ |
| NumberText, CurrencyText, PercentText, FileSizeText, DateTimeText | typography/formatted.rs, format.rs | ✓ | n/a | n/a | n/a | n/a | ✓ (signed percent ink) | ✓ text role | ✓ `tests/values.rs` | G20 G25 | ✓ |
| DurationText, PluralText, RelativeTime (Clock, Figures) | typography/formatted.rs, format.rs | ✓ | n/a | n/a | n/a | ✓ 30 s refresh (host clock) | ✓ | ✓ text role | ✓ `tests/values.rs` (matches the Rust port case by case) | G20 G25 G26 | ✓ |
| CopyButton (chapter 5, ported early) | buttons/copy.rs | ✓ | ✓ | ✓ (IconButton) | ✓ | ✓ check turns in, 1.4 s | ✓ | ✓ (IconButton) | ✓ `tests/copy_select.rs` | | ✓ |
| ButtonGroup | buttons/group.rs | ✓ | n/a | ✓ (Button) | ✓ (Button) | n/a | ✓ joined corners | ✓ (Button) | ✓ `tests/toggles.rs` | G47 | ✓ |
| ToggleButton, ToggleGroup (ToggleItem, Toggles) | buttons/group.rs | ✓ | ✓ toggled, selection-changed | ✓ Enter/Space, Tab stop | ✓ hover, compact tooltip | n/a | ✓ | ✓ button, checked | ✓ `tests/toggles.rs`, `src/toggles.rs` | G48 | ✓ |
| FloatingActionButton | buttons/fab.rs | ✓ | ✓ | ✓ Enter/Space, Tab stop | ✓ hover lift, tooltip | ✓ 320 ms rise | ✓ | ✓ button named | ✓ `tests/fab.rs` | G49 | ✓ |
| SegmentedControl (Segment) | buttons/segmented.rs | ✓ | ✓ selection-changed | ✓ Enter/Space, every segment a Tab stop, focus scrolls into view | ✓ compact tooltip, sideways scroll | ✓ 320 ms spring thumb | ✓ | ✓ radio button, checked | ✓ `tests/segmented.rs` | | ✓ |
| ConfirmButton (ConfirmMode) | buttons/confirm.rs | ✓ | ✓ confirmed | ✓ keys press twice | ✓ hold 1.2 s, release cancels | ✓ hold fill, check turns in | ✓ | ✓ button named by its label | ✓ `tests/confirm.rs` | G26 G36 | ✓ |
| CopyableText | typography/copy.rs | ✓ | n/a | ✓ (CopyButton) | ✓ | ✓ | ✓ | ✓ | ✓ `tests/copy_select.rs` | G20 | ✓ |
| SelectableText | typography/select.rs | ✓ | n/a | ✓ copy, select all | ✓ press, shift, drag | n/a | ✓ selection fill | ✓ text node | ✓ `tests/copy_select.rs` | G27 | ✓ |
| Emoji | typography/emoji.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/copy_select.rs` | | ✓ |
| Typewriter | typography/effects.rs | ✓ | n/a | n/a | n/a | ✓ per grapheme, caret breath, reduced motion | ✓ | ✓ text node | ✓ `tests/effects.rs` | | ✓ |
| GradientText, ShimmerText | typography/effects.rs | ✓ | n/a | n/a | n/a | ✓ 1.8 s sweep, reduced motion | ✓ | ✓ text node | ✓ `tests/effects.rs` | G28 | ✓ |
| AnimatedNumber | typography/rolling.rs | ✓ | n/a | n/a | n/a | ✓ roll and count, 640 ms ease-out | ✓ | ✓ reads its target | ✓ `tests/effects.rs` | G20 G25 | ✓ |
| HStack, VStack (Stack) | layout/stack.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/layout.rs` | | ✓ |
| ZStack, ZLayer | layout/stack.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/layout.rs` | | ✓ |
| AspectRatio, Container | layout/stack.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/layout.rs` | | ✓ |
| SimpleGrid, Masonry | layout/grid.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/layout.rs`, `src/layout.rs` | | ✓ |
| Scrollbar | layout/scroll.rs | ✓ | n/a | n/a | ✓ thumb drag, track blocks presses | ✓ 900 ms idle, 320 ms fade | ✓ | ✓ (no node) | ✓ `tests/scroll.rs` | | ✓ |
| ScrollArea | layout/scroll.rs | ✓ | n/a | n/a | ✓ wheel, hover shows bars | ✓ (Scrollbar) | ✓ | ✓ (no node) | ✓ `tests/scroll.rs` | G31 | ✓ |
| ScrollShadow, ScrollToTop, StickyHeader | layout/scroll_aids.rs | ✓ | ✓ | ✓ (IconButton) | ✓ | ✓ 320 ms ease back | ✓ | ✓ button label | ✓ `tests/scroll.rs` | G10 | ✓ |
| Card, CardHeader, CardBody, CardFooter | layout/surface.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/surface.rs` | | ✓ |
| Panel, PanelHeader | layout/surface.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/surface.rs` | | ✓ |
| Section, SectionHeader | layout/surface.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/surface.rs` | | ✓ |
| Fieldset (Group), Frame, Well | layout/surface.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/surface.rs` | | ✓ |
| Collapsible | layout/collapse.rs | ✓ | n/a | n/a | n/a | ✓ 200 ms ease-in-out, restarts from the far end | ✓ | ✓ (no node) | ✓ `tests/collapse.rs` | | ✓ |
| Accordion, AccordionItem | layout/collapse.rs | ✓ | ✓ | ✓ Enter/Space, Tab stop | ✓ | ✓ (Collapsible, Disclosure) | ✓ | ✓ button, expanded | ✓ `tests/collapse.rs` | G21 G32 | ✓ |
| SplitPane, SplitCell (ResizablePanel) | layout/split.rs | ✓ | ✓ resized | n/a | ✓ drag from first move, floor | n/a | ✓ | ✓ (no node) | ✓ `tests/split.rs`, `src/layout.rs` | | ✓ |
| ResizeHandle (resize_handle, resize_edge) | layout/split.rs | ✓ | ✓ | n/a | ✓ hover lights, resize cursor | n/a | ✓ | ✓ (no node) | ✓ `tests/split.rs` | | ✓ |
| Sidebar | layout/sidebar.rs | ✓ | n/a | n/a | n/a | ✓ 200 ms ease-in-out, restarts | ✓ | ✓ (no node) | ✓ `tests/shells.rs` | | ✓ |
| Sheet | layout/sheet.rs | ✓ | ✓ close | ✓ (IconButton) | ✓ scrim and close button | ✓ 200 ms slide in | ✓ | ✓ close button | ✓ `tests/shells.rs` | G10 G12 | ✓ |
| Drawer | layout/sheet.rs | ✓ | ✓ close | n/a | ✓ pull past 30 %, scrim | ✓ rise, spring back | ✓ | n/a | ✓ `tests/shells.rs` | G12 | ✓ |
| Page, PageHeader, PageContent | layout/page.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/shells.rs` | | ✓ |
| AppShell, AppShellBar, AppShellMain, AppShellContent | layout/page.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/shells.rs` | | ✓ |
| MasterDetail | layout/page.rs | ✓ | ✓ (SplitPane) | n/a | ✓ (SplitPane) | n/a | ✓ | ✓ (no node) | ✓ `tests/shells.rs` | | ✓ |
| Viewport (Transform) | layout/viewport.rs | ✓ | n/a | ✓ ⌘/Ctrl zoom | ✓ drag pan, wheel pan, zoom about pointer | n/a | ✓ | ✓ (no node) | ✓ `tests/viewport.rs` | G31 | ✓ |
| FloatingPanel | layout/floating.rs | ✓ | ✓ moved | n/a | ✓ bar drag, kept in window | n/a | ✓ | ✓ text role | ✓ `tests/viewport.rs` | G12 | ✓ |
| DragTouch | div.rs `DRAG_THRESHOLD` (gpui) | ✓ | ✓ started, moved, ended, tapped | n/a | ✓ 2 px threshold | n/a | n/a | n/a | ✓ `tests/split.rs`, `tests/shells.rs` | | ✓ |
| Dock / DockPanel / DockZone | layout/dock/ | ✓ | ✓ activate, move, float, resize | n/a | ✓ tab click and drag, zones, handle | n/a | ✓ | ✓ tab role, selected | ✓ `tests/dock_panes.rs`, `src/dock.rs` | G12 G33 G34 | ✓ |
| Pane / PaneGroup | layout/panes.rs | ✓ | ✓ split, close, focus, resize | n/a | ✓ press focus, divider drag | n/a | ✓ | ✓ buttons | ✓ `tests/dock_panes.rs`, `src/panes.rs` | G33 G35 | ✓ |
| Workspace | layout/workspace.rs | ✓ | n/a | n/a | n/a | n/a | n/a | n/a | ✓ `src/workspace.rs`, `tests/dock_panes.rs` | | ✓ |
| WindowChrome, window host (`wire_window!`, `guard_close`) | shell/chrome.rs, frame.rs, closing.rs | ✓ | ✓ move, resize, minimize, zoom, close, zoom % | n/a | n/a | n/a | n/a | n/a | ✓ `tests/title_bar.rs` | G37 G38 G39 G40 | ✓ |
| WindowDragRegion (WindowDragArea) | shell/chrome.rs `drag_region` | ✓ | ✓ start-move, double click | n/a | ✓ moves on the first move of a press | n/a | n/a | n/a | ✓ `tests/title_bar.rs` | G38 | ✓ |
| WindowControls / TrafficLights | shell/chrome.rs | ✓ | ✓ close-requested | n/a | ✓ hover glyphs, occludes | n/a | ✓ | ✓ buttons named | ✓ `tests/title_bar.rs` | G37 | ✓ |
| TitleBar (TitleBarTitle) | shell/chrome.rs | ✓ | ✓ close-requested, routed system close | n/a | ✓ drag, slots take presses | n/a | ✓ | ✓ title text | ✓ `tests/title_bar.rs` | G39 G41 | ✓ |
| WindowResizeBorder (ResizeBorder) | shell/frame.rs | ✓ | ✓ start-resize | n/a | ✓ edge cursors | n/a | ✓ | n/a | ✓ `tests/title_bar.rs` | G37 | ✓ |
| Vibrancy | shell/vibrancy.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | n/a | n/a (a fill) | | ✓ |
| ColumnShell / ShellColumn (ShellColumnHeader, ShellColumnTitle, ShellColumnBody) | shell/columns.rs | ✓ | ✓ resized, close-requested | n/a | ✓ border drags from the drawn width, header drags the window | n/a | ✓ | ✓ title text | ✓ `tests/column_shell.rs`, `src/columns.rs` | G39 G42 | ✓ |
| Hosted window (HostedWindow, `open_hosted`, `open_splash`) | shell/windows.rs | ✓ | ✓ Escape closes | ✓ Escape | ✓ strip drags | n/a | ✓ | n/a | ✓ `tests/dialogs.rs` | G44 | ✓ |
| AboutDialog (AboutLink) | shell/dialogs.rs | ✓ | ✓ links open | n/a | n/a | n/a | ✓ | ✓ texts, links | ✓ `tests/dialogs.rs` | G44 | ✓ |
| SplashScreen | shell/dialogs.rs | ✓ | n/a | n/a | n/a | ✓ rises in | ✓ | ✓ progress | ✓ `tests/dialogs.rs` | G43 G44 | ✓ |
| CrashReporter | shell/crash.rs | ✓ | ✓ send, dismiss, copied | n/a | ✓ report opens | ✓ collapse | ✓ | ✓ buttons | ✓ `tests/dialogs.rs` | G44 | ✓ |
| UpdateBanner / UpdateDialog (UpdateState) | shell/update.rs | ✓ | ✓ download, restart, later | n/a | n/a | ✓ progress glide | ✓ | ✓ buttons | ✓ `tests/dialogs.rs` | | ✓ |
| OfflineIndicator (Connectivity, ConnectivityDot) | shell/status.rs | ✓ | ✓ changed-state | n/a | n/a | ✓ enter, hold fade, breathing dot | ✓ | ✓ text | ✓ `tests/status.rs` | | ✓ |
| ZoomControl (ZoomStep, ZoomSteps) | shell/status.rs | ✓ | ✓ zoom (owned), set-zoom (window) | n/a | ✓ reset hover | ✓ rolling percent | ✓ | ✓ buttons, reset named by percent | ✓ `tests/status.rs` | G40 | ✓ |
| WindowManager (Rust, `wire_managed_windows!`) | shell/windows.rs | ✓ | ✓ open, list, focus | n/a | n/a | n/a | n/a | n/a | ✓ `tests/switcher.rs`, `src/windows.rs` | G44 G45 | ✓ |
| WindowSwitcher (ManagedWindows) | shell/switcher.rs | ✓ | ✓ close, focus | ✓ arrows, Tab, Shift+Tab, Enter on release, Escape | ✓ scrim, rows | ✓ rise in | ✓ | ✓ list, selectable items | ✓ `tests/switcher.rs` | G19 G45 | ✓ |
| MiniWindow (Corner, `open_mini`, `wire_mini_window!`) | shell/windows.rs | ✓ | ✓ expand, close | n/a | ✓ bar on hover, title drags | n/a | ✓ | ✓ buttons | ✓ `tests/switcher.rs` | G44 G45 | ✓ |
| TrayIcon (TrayItem, TrayLook) | ely-desktop native.rs, native/linux.rs, platform/ | ✓ | ✓ picks on the event loop | n/a | ✓ left click picks the first | n/a | n/a | system menus | ✓ `tests/desktop.rs` (Linux, private bus); macOS and Windows type-checked | G46 | ✓ |
| Dock badge (`set_dock_badge`, `dock_badge`) | ely-desktop native.rs, platform/mac.rs | ✓ | n/a | n/a | n/a | n/a | n/a | n/a | type-checked (macOS only) | G46 | ✓ |
| Toolbar / ToolbarGroup / ToolbarSeparator | shell/bars.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | n/a | ✓ `tests/bars.rs` | | ✓ |
| StatusBar / StatusBarSide / StatusBarItem | shell/bars.rs, status.rs | ✓ | ✓ clicked | n/a | ✓ hover, tooltip | n/a | ✓ | ✓ clickable items are buttons | ✓ `tests/bars.rs` | | ✓ |
| ActivityBar (ActivityItem) | shell/activity.rs | ✓ | ✓ select | n/a | ✓ hover, tooltip | ✓ mark grows in | ✓ | ✓ tab role, selected | ✓ `tests/bars.rs` | | ✓ |
| TabBar (WindowTab) | shell/tabs.rs | ✓ | ✓ select, close, add, reorder | n/a | ✓ drag to reorder, ghost | ✓ rise in | ✓ | ✓ tab-list, tabs selectable | ✓ `tests/bars.rs` | G12 | ✓ |
| Badge / CountBadge / DotBadge / Tag (Tone) | data_display/badge.rs (chapter 12, early) | ✓ | ✓ remove | n/a | ✓ x hover | ✓ count rolls | ✓ | ✓ text, remove button | ✓ `tests/bars.rs` | | ✓ |
| ProgressBar / ProgressRing | motion/progress.rs (chapter 6, early) | ✓ | n/a | n/a | n/a | ✓ glide, sweep | ✓ | ✓ progress-indicator, value | ✓ `tests/progress.rs` | G43 | ✓ |

Gallery: pages 1–4 (Foundations, Typography, Layout, Window & Shell) mirror the story app; page 4's sidebar rows are a stand-in until NavRow lands with Navigation (chapter 7), and its window buttons open the real windows (about, splash, update, crash, managed, mini, picture in picture; the tray and Dock badge with `--features desktop`); `examples/capture.rs` renders any page headless. Layout regressions found there are pinned in `tests/squeeze.rs`.

Chapter 1 entries that need no component of their own (Slint builtins or aliases) are marked in `INVENTORY.md`.

Not started: everything else in `INVENTORY.md`, in its port order.
