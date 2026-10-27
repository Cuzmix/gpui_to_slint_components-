# Spec: Window & Shell (chapter 4)

Source: `ely-core/src/shell/` (`chrome.rs`, `frame.rs`, `closing.rs`, `columns.rs`, `bars.rs`, `activity.rs`, `tabs.rs`, `status.rs`, `dialogs.rs`, `crash.rs`, `update.rs`, `switcher.rs`, `windows.rs`, `vibrancy.rs`) and their tests (`shell/tests.rs`, `tests/title.rs`, `tests/border.rs`); `ely-desktop` (`native.rs`, `platform/`) for the tray and the Dock badge; stories in `ely-gallery` `pages/shell/`. English strings from `i18n/builtin/en.rs`.

## Window host (Rust, `ely_slint::window`)
- Every window-level act goes through the `WindowChrome` global, one per window, which `wire_window!` connects to the window: `start-move` (winit `drag_window`), `start-resize(edge)` (`drag_resize_window`), `minimize`, `zoom` (toggle maximized), `title-double-click` (macOS and Linux zoom; Windows nothing in gpui, see G38), `close` (asks the close check, then hides), `set-zoom(percent)`.
- The host keeps `active`, `maximized`, `fullscreen`, `client-decorated`, `offers-minimize`, `offers-maximize`, `system-lights` (the system draws macOS's lights) and `zoom-percent` current.
- `guard_close(window, check)`: the check runs before any close (system or drawn) removes the window; false keeps it. Logged when it keeps the window.
- Routing: a bar with `routes-close` (gpui: an `on_close` and no forced platform) takes system closes while it is shown: the host bumps `close-asked` and keeps the window; the bar runs `close-requested`.
- Resize edges: `resize-edge(pos, inset, size, tiling)` picks the untiled corner first, then the edge, within `inset` (10 px); none in the middle or on a tiled edge. Cursors: up-down, left-right, and the two diagonals.

## TitleBar / WindowControls / WindowDragArea
- Drag area: a left press arms it (unless a control inside took the press); the first move while armed starts the window move and disarms; release disarms. A double click runs `title-double-click`.
- WindowControls by platform style:
  - Mac: close, minimize, maximize lights, 12 px round, 8 px apart, `danger`/`warning`/`success` while the window is active, `border_strong` otherwise, 1 px `shadow` at 12 % rim; glyphs ×, −, + in xs bold, hidden until the group is hovered, then `shadow` at 60 %.
  - Windows: minimize, maximize (copy icon when maximized, else square), close; each 46 px wide and the title bar's height less 1 px; hover `hover` (close: `danger`, glyph `on_accent`); icons sm `fg_muted`, `fg` on hover.
  - Linux: minimize and maximize when offered, close always; 24 px round on `hover`, `active` when hovered; icons xs `fg`; 8 px apart.
  - Each runs minimize, zoom, or close (the owner's `close-requested` when it handles close). Logged. The controls take presses so nothing under them sees one.
- TitleBar: 38 px, `bg`, `border` hairline below. On macOS with no forced platform the system draws the lights (Mac style, none drawn); else the style's controls are drawn. Mac controls lead, others trail. Centered layout (all but Windows): start and end take equal flexible room, the title in between; start pads 78 px when the system draws the lights and the window is not full screen, else 12 px; end pads 8 px right. Windows: 12 px left, start and end at their own width, the title fills. Title: sm, medium, `fg_muted`, ellipsized. Slots (leading children and actions) are rows 8 px apart that take their own presses.

## ResizeBorder / Vibrancy
- With client decorations, a `window-inset` margin on each untiled side to resize from, with the edge's cursor; inside, a clipped pane on `bg` (none with `glass`); floating (untiled), radius lg, 1 px `border_strong`, modal shadow unless glass. Without client decorations it is a plain box.
- Vibrancy: a box on `glass`.

## ColumnShell / ShellColumn
- Columns side by side, full height, ground `bg` (or `glass`). Set widths are flex bases, a column without one fills. Each border lies toward the filling column; a column after it borders its left, others their right (the last none). A resizable column draws its border with a 6 px handle; others a 1 px `border` line.
- Header: the title bar's height, 8 px gap; the first header pads 78 px for the system's lights (not full screen), else 12 px; 12 px right unless Windows controls end it flush. Drawn controls lead the first (Mac) or end the last (others). The header items (leading, title, actions) are at least 32 px wide and scroll sideways when short; the title (sm, medium, `fg_muted`, ellipsized) or a spacer drags the window.
- Resize: a press on the handle remembers x; the first sideways move sets off from max(drawn width, owner's width) and the owner's width; each move clamps to [min, max] (towards the filling column grows); a move back to the start gives the owner's width; `resized(width)` only when it differs. `min > max` panics in gpui (logged and ignored here).

## Toolbar / StatusBar / ActivityBar / TabBar
- Toolbar: 32 px row, 4 px sides, 4 px gap. Group: 2 px gap. Separator: 4 px margin, 16 px tall 1 px `border` line.
- StatusBar: 24 px, 4 px sides, `bg`, `border` top hairline, xs `fg_muted`; left and right sides 2 px gap, shrink, clip, 8 px apart, right end-aligned. Item: 4 px gap, 6 px sides, radius sm; leading, icon xs `fg_muted`, label ellipsized; a tooltip; clickable ones are buttons named by label, tooltip or icon, `hover` on hover, pointer cursor.
- ActivityBar: 48 px wide, full height, 4 px top and bottom, `surface`, `border` right; items 48 × 48, icon lg `fg_subtle` (`fg` selected or hovered), tooltip label; the selected one has a 2 px `fg` mark on the left that grows to 20 px over `BASE` with ease-out-cubic; a badge (CountBadge) 4 px from the top right. Main items top, footer bottom.
- TabBar: tabs 4 px apart; each flexes between 96 and 200 px, 28 px tall, 12 px left 4 px right, radius md, 1 px border, sm text, 8 px gap. Selected: `surface`, `border`, `fg`, medium; others transparent border, `fg_muted`, `hover` on hover. Icon sm (`fg` selected, else `fg_subtle`). Close (sm IconButton x) shown on the selected tab, else on hover. New tab: sm IconButton plus. Tabs rise in (opacity, 4 px) over `BASE`. Drag to reorder: the ghost carries title and icon; a tab under the drag outlines `focus`; a drop on another tab reports `(from, to)`.

## Dialog contents
- Mark: 64 px tile, radius xl, `sunken`, 1 px `border`; xxl initial (`fg`) or xxl icon.
- AboutDialog: column centered, 8 px gap, 32 px sides, 16 px top, 32 px bottom: mark; Title (12 px above); CopyableText "Version {v}"; description Paragraph centered (8 px above); links (ExternalLink) 16 px apart (8 px above); copyright Caption (16 px above). Window 360 × 400.
- SplashScreen: centered column, 40 px sides; block (mark, Title 8 px above, tagline Caption) rises in over `SLOW` (8 px); then 60 % wide, 32 px above: ProgressBar (value or indeterminate), status xs `fg_subtle`. Progress outside 0..1 panics in gpui (clamped and logged here). Window 480 × 300, no title bar, not movable or resizable.
- CrashReporter: column, 16 px gap, 32 px sides, 24 px bottom: TriangleAlert xl `fg_muted`; Title "{name} quit unexpectedly"; Paragraph summary; row: link Button "Show report"/"Hide report" with chevron down/up, ghost "Copy report" with copy icon (copies, logged); Collapsible report: 160 px tall, radius md, 1 px `border`, `sunken`, mono xs `fg_muted`, scrolling, 12 px in; a spacer; end row 8 px apart: ghost "Don't send", primary "Send report" (each only with a handler). Window 400 × 480 titled "{name} crashed".
- UpdateBanner: 8 px top and bottom, 16 px left, 8 px right, 12 px gap, `surface`, `border` bottom, sm: Download icon sm `fg_muted`; message (fills): "Version {version} is available.", "Downloading {version}… {percent}%", "Version {version} is ready."; Download (Available) or "Restart to update" (Ready), sm primary; an x dismiss when a later handler is set; the progress bar along its foot while downloading.
- UpdateDialog: column, 16 px gap, 32 px sides, 24 px bottom: Title "{name} {version} is available" (or "is ready"), Caption "You have {current}."; notes box (fills, radius lg, `border`, `sunken`, scrolling, 16 px in, 8 px apart, "·" and the line in Labels); the bar while downloading; end row: ghost "Later", primary Download / "Restart to update". Window 400 × 480 titled "Software Update".
- Hosted window: a drag strip the title bar's height, then the content on `bg`; Escape closes it; not resizable or minimizable.

## OfflineIndicator / ZoomControl
- Pill: 12 px sides, 4 px top and bottom, round, `surface`, 1 px `border`, raised shadow, sm `fg_muted`, 8 px gap: dot (6 px: `success` online, `warning` reconnecting, breathing between 35 % and 100 % over `SLOW` × 3 with ease-in-out-cubic unless reduced motion, `fg_subtle` offline), icon sm (wifi-off offline, wifi otherwise), label "Offline · changes stay on this device", "Reconnecting…", "Back online". Shown while offline or reconnecting, and for 2.4 s once back online. Enters over `BASE` (opacity, 4 px); back online it fades in over the first 10 % and out over the last 15 % of 2.4 s (reduced motion: the plain entrance). Changes logged.
- ZoomControl: steps 50, 67, 75, 80, 90, 100, 110, 125, 150, 175, 200; the next step up or down, none at the ends. Zoom out (sm IconButton), the percent (AnimatedNumber sm and "%") on a 48 px-wide, 24 px-tall round button that resets to 100, zoom in (ghost). Window mode scales the window (disabled at the ends); owned mode shows the owner's percent and hands Out, In, Reset.

## WindowSwitcher / WindowManager / MiniWindow
- WindowManager (Rust): windows opened through it, oldest first, with their titles; each opens one title bar (38 px) down and right of the newest, the first centered; closed ones drop from the list; `focus` brings one forward. Logged.
- WindowSwitcher: over a Backdrop (centered); a 380 px panel, 8 px in, 2 px apart, radius xl, `overlay`, `border`, modal shadow, list box named "Windows", rising in over `BASE`. Rows 32 px, 12 px sides, radius md, base text, app-window icon sm `fg_muted`; the selected one on `hover`. Down, Tab step on; Up, Shift+Tab step back (wrapping); Escape or a scrim click closes; Enter switches on release (so the key does not reach the opener). "No managed windows are open." when there are none. Focus is taken while open and given back on close.
- MiniWindow (Rust + Slint chrome): a small always-on-top window at a display corner (24 px margin, 48 px from the top) or centered; a bar over its top shown on hover: 32 px, 8 px sides, `overlay` at 92 %, `border` below, xs `fg_muted`; the title drags; expand (maximize-2) closes it and activates the opener; close (x).

## Desktop (Rust, `ely_slint::desktop`, feature `desktop`)
- TrayIcon: macOS NSStatusItem with an SF Symbol and a menu, Windows Shell_NotifyIcon with the PNG, Linux StatusNotifierItem with a dbusmenu; picks hand the index among all items (a left click picks the first) to the app on the event loop. Dropping removes it.
- Dock badge: macOS only; elsewhere an error.

## Slint
- Platform calls happen in the host; components ask `WindowChrome` (G37, G38). Notes in PORTING_GAPS.md: G37 (window acts through winit), G38 (no native caption hit-test on Windows), G39 (closing routes by flag), G40 (zoom through the scale factor).
- Conservative readings: gpui panics on bad input in code (splash progress, column limits backwards, a resizing column with no border toward a filling one); here the value is clamped or the resize is off, and it is logged.
