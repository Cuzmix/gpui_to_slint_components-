//! What `wire_text_format!` hands the `TextFormat` global. Each function takes and returns Slint
//! values, so the macro is only glue.

use slint::{Color, ModelRc, SharedString, StyledText, VecModel};

use crate::format::{self, Separators};
use crate::keys::{Platform, keystroke, keystroke_labels};
use crate::text;

fn strings(items: Vec<String>) -> ModelRc<SharedString> {
    ModelRc::new(VecModel::from(items.into_iter().map(SharedString::from).collect::<Vec<_>>()))
}

/// The caps of one keystroke; empty if it does not parse.
pub fn key_labels(source: &str, platform: Platform) -> ModelRc<SharedString> {
    match keystroke(source, platform) {
        Ok(stroke) => strings(keystroke_labels(&stroke, platform)),
        Err(error) => {
            log::error!("kbd: {error}");
            strings(Vec::new())
        }
    }
}

/// The caps of each chord; a chord that does not parse shows its source on one cap.
pub fn key_chords(source: &str, platform: Platform) -> ModelRc<ModelRc<SharedString>> {
    let chords: Vec<_> = source
        .split_whitespace()
        .map(|chord| match keystroke(chord, platform) {
            Ok(stroke) => strings(keystroke_labels(&stroke, platform)),
            Err(error) => {
                log::error!("kbd combo: {error}");
                strings(vec![chord.to_string()])
            }
        })
        .collect();
    ModelRc::new(VecModel::from(chords))
}

/// Every end cut, shortest first.
pub fn end_cuts(text: &str) -> ModelRc<SharedString> {
    strings(text::end_cuts(text))
}

/// Every middle cut, shortest first.
pub fn middle_cuts(text: &str) -> ModelRc<SharedString> {
    strings(text::middle_cuts(text))
}

/// Alternating plain and marked runs, first plain.
pub fn highlight_runs(text: &str, query: &str) -> ModelRc<SharedString> {
    strings(text::highlight_runs(text, &text::matches(text, query)))
}

/// Grouped digits at fixed precision.
pub fn number(value: f32, decimals: i32) -> SharedString {
    format::number(f64::from(value), decimals.max(0) as usize, Separators::EN).into()
}

/// An amount in a currency.
pub fn currency(amount: f32, code: &str) -> SharedString {
    format::currency(f64::from(amount), code).into()
}

/// A ratio as percent.
pub fn percent(ratio: f32, decimals: i32, signed: bool) -> SharedString {
    format::percent(f64::from(ratio), decimals.max(0) as usize, signed).into()
}

/// Bytes in decimal or binary units; negative counts read as none.
pub fn file_size(bytes: f32, binary: bool) -> SharedString {
    format::file_size(bytes.max(0.0) as u64, binary).into()
}

/// Unix seconds in a pattern and zone; empty, logged, if either does not read.
pub fn datetime(at: i32, pattern: &str, zone: &str) -> SharedString {
    let zone = if zone.is_empty() {
        format::system_zone("DateTimeText")
    } else {
        match jiff::tz::TimeZone::get(zone) {
            Ok(zone) => zone,
            Err(error) => {
                log::error!("DateTimeText zone {zone:?}: {error}");
                return SharedString::new();
            }
        }
    };
    let at = match jiff::Timestamp::from_second(i64::from(at)) {
        Ok(at) => at,
        Err(error) => {
            log::error!("DateTimeText moment {at}: {error}");
            return SharedString::new();
        }
    };
    match format::datetime(at, &zone, pattern) {
        Ok(text) => text.into(),
        Err(error) => {
            log::error!("DateTimeText pattern {pattern:?}: {error}");
            SharedString::new()
        }
    }
}

/// The emoji a shortcode names; empty if unknown.
pub fn emoji(shortcode: &str) -> SharedString {
    text::emoji(shortcode).unwrap_or_default().into()
}

/// Grapheme clusters in `text`.
pub fn grapheme_count(text: &str) -> i32 {
    i32::try_from(text::graphemes(text).len()).unwrap_or(i32::MAX)
}

/// The first `count` grapheme clusters.
pub fn grapheme_prefix(text: &str, count: i32) -> SharedString {
    text::grapheme_prefix(text, count.max(0) as usize).into()
}

pub(crate) fn rgba(color: Color) -> [u8; 4] {
    [color.red(), color.green(), color.blue(), color.alpha()]
}

pub(crate) fn styled(markdown: &str, text: &str) -> StyledText {
    StyledText::from_markdown(markdown).unwrap_or_else(|error| {
        log::error!("styled text: {error}");
        StyledText::from_plain_text(text)
    })
}

/// A ramp from `from` to `to` across the graphemes.
pub fn gradient(text: &str, from: Color, to: Color) -> StyledText {
    styled(&text::gradient_markdown(text, rgba(from), rgba(to)), text)
}

/// The shimmer's band at loop position `t`.
pub fn shimmer(text: &str, dim: Color, lit: Color, t: f32) -> StyledText {
    styled(&text::shimmer_markdown(text, rgba(dim), rgba(lit), t), text)
}

/// AnimatedNumber's text.
pub fn rolling_text(value: f32, decimals: i32, pad: i32) -> SharedString {
    text::rolling_text(f64::from(value), decimals.max(0) as usize, pad.max(0) as usize).into()
}

/// Two texts aligned by place, as (from, to) pairs.
pub fn roll_pairs(from: &str, to: &str) -> Vec<(SharedString, SharedString)> {
    let (from, to) = text::align(from, to);
    from.into_iter().zip(to).map(|(from, to)| (from.into(), to.into())).collect()
}

/// The current moment in Unix seconds, as `ely_slint::format::now` reads it.
pub fn now() -> i32 {
    i32::try_from(format::now().as_second()).unwrap_or(i32::MAX)
}

/// Wires the `TextFormat` global of `app` to this crate. The host's root `.slint` file exports
/// `TextFormat`, `PlatformStyle` and `RollCell`, and the generated types are in scope where this is called.
///
/// ```ignore
/// let app = App::new()?;
/// ely_slint::wire_text_format!(app);
/// ```
#[macro_export]
macro_rules! wire_text_format {
    ($app:expr) => {{
        fn platform(style: PlatformStyle) -> $crate::Platform {
            match style {
                PlatformStyle::Mac => $crate::Platform::Mac,
                PlatformStyle::Windows => $crate::Platform::Windows,
                PlatformStyle::Linux => $crate::Platform::Linux,
            }
        }
        let format = $app.global::<TextFormat>();
        format.on_key_labels(|source, style| $crate::wire::key_labels(&source, platform(style)));
        format.on_key_chords(|source, style| $crate::wire::key_chords(&source, platform(style)));
        format.on_end_cuts(|text| $crate::wire::end_cuts(&text));
        format.on_middle_cuts(|text| $crate::wire::middle_cuts(&text));
        format.on_highlight_runs(|text, query| $crate::wire::highlight_runs(&text, &query));
        format.on_number(|value, decimals| $crate::wire::number(value, decimals));
        format.on_currency(|amount, code| $crate::wire::currency(amount, &code));
        format.on_percent(|ratio, decimals, signed| $crate::wire::percent(ratio, decimals, signed));
        format.on_file_size(|bytes, binary| $crate::wire::file_size(bytes, binary));
        format.on_datetime(|at, pattern, zone| $crate::wire::datetime(at, &pattern, &zone));
        format.on_emoji(|shortcode| $crate::wire::emoji(&shortcode));
        format.on_grapheme_count(|text| $crate::wire::grapheme_count(&text));
        format.on_grapheme_prefix(|text, count| $crate::wire::grapheme_prefix(&text, count));
        format.on_gradient(|text, from, to| $crate::wire::gradient(&text, from, to));
        format.on_shimmer(|text, dim, lit, t| $crate::wire::shimmer(&text, dim, lit, t));
        format.on_rolling_text(|value, decimals, pad| $crate::wire::rolling_text(value, decimals, pad));
        format.on_roll_cells(|from, to| {
            let cells: Vec<RollCell> = $crate::wire::roll_pairs(&from, &to)
                .into_iter()
                .map(|(from, to)| RollCell { from, to })
                .collect();
            ::slint::ModelRc::new(::slint::VecModel::from(cells))
        });
    }};
}

/// Sets the `Clock` global of `app` to now and moves it on every 30 s, as gpui redraws a
/// relative time. Keep the returned timer: dropping it stops the clock.
///
/// ```ignore
/// let _clock = ely_slint::wire_clock!(app);
/// ```
#[macro_export]
macro_rules! wire_clock {
    ($app:expr) => {{
        $app.global::<Clock>().set_now($crate::wire::now());
        let weak = $app.as_weak();
        let timer = ::slint::Timer::default();
        timer.start(::slint::TimerMode::Repeated, std::time::Duration::from_secs(30), move || {
            if let Some(app) = weak.upgrade() {
                app.global::<Clock>().set_now($crate::wire::now());
            }
        });
        timer
    }};
}

/// Wires the `LayoutMath` global of `app` to [`crate::layout`]. The host exports `LayoutMath`
/// and `MasonrySpot` from its root `.slint` file.
///
/// ```ignore
/// ely_slint::wire_layout!(app);
/// ```
#[macro_export]
macro_rules! wire_layout {
    ($app:expr) => {{
        fn lengths(model: ::slint::ModelRc<f32>) -> Vec<f32> {
            use ::slint::Model;
            model.iter().collect()
        }
        let math = $app.global::<LayoutMath>();
        math.on_masonry(|heights, columns, gap| {
            let spots: Vec<MasonrySpot> = $crate::layout::masonry(&lengths(heights), columns.max(1) as usize, gap)
                .into_iter()
                .map(|(column, y)| MasonrySpot { column: column as i32, y })
                .collect();
            ::slint::ModelRc::new(::slint::VecModel::from(spots))
        });
        math.on_masonry_height(|heights, columns, gap| {
            $crate::layout::masonry_height(&lengths(heights), columns.max(1) as usize, gap)
        });
        math.on_split_shares(|weights, count| {
            let shares = $crate::layout::split_shares(&lengths(weights), count.max(0) as usize);
            ::slint::ModelRc::new(::slint::VecModel::from(shares))
        });
        math.on_split_shift(|start, ix, delta, floor| {
            let next = $crate::layout::split_shift(&lengths(start), ix.max(0) as usize, delta, floor);
            ::slint::ModelRc::new(::slint::VecModel::from(next))
        });
    }};
}

/// Brings `model` to `rows` in place: changed rows are set, the rest added or dropped at the
/// end. A repeater over it keeps its instances, so a press or drag under way survives.
pub fn sync<T: Clone + PartialEq + 'static>(model: &VecModel<T>, rows: Vec<T>) {
    use slint::Model;
    for (ix, row) in rows.iter().enumerate() {
        if ix < model.row_count() {
            if model.row_data(ix).as_ref() != Some(row) {
                model.set_row_data(ix, row.clone());
            }
        } else {
            model.push(row.clone());
        }
    }
    while model.row_count() > rows.len() {
        model.remove(model.row_count() - 1);
    }
}

/// Wires `Docks` and `PaneGroups` to the docks and pane groups in a
/// [`WorkspaceHost`](crate::workspace::WorkspaceHost), which it returns. The host's root
/// `.slint` file exports `Docks`, `PaneGroups`, `DockSide`, `DockTab`, `DockEdge`, `DockFloatSpot`,
/// `PaneSpan`, `PaneCell`, `PaneDivider`, `Icons` and `IconEntry`. A panel's icon is its name in
/// `Icons.all`. The macro names those Slint types bare, so keep `ely_slint::dock::DockSide` out
/// of scope unqualified where it expands.
///
/// ```ignore
/// let host = ely_slint::wire_workspace!(app);
/// host.update(|systems| { systems.groups.insert("editor".into(), ely_slint::panes::PaneGroup::new()); });
/// ```
#[macro_export]
macro_rules! wire_workspace {
    ($app:expr) => {{
        use ::std::{cell::RefCell, collections::HashMap, rc::Rc};
        use $crate::{dock, panes, wire::sync};

        #[derive(Default)]
        struct DockModels {
            edges: Rc<::slint::VecModel<DockEdge>>,
            tabs: [Rc<::slint::VecModel<DockTab>>; 4],
            floating: Rc<::slint::VecModel<DockFloatSpot>>,
        }
        #[derive(Default)]
        struct GroupModels {
            cells: Rc<::slint::VecModel<PaneCell>>,
            dividers: Rc<::slint::VecModel<PaneDivider>>,
            handles: Vec<panes::PaneHandle>,
        }
        #[derive(Default)]
        struct Models {
            docks: HashMap<String, DockModels>,
            groups: HashMap<String, GroupModels>,
        }
        type Icons_ = HashMap<String, ::slint::Image>;

        fn side_of(side: DockSide) -> dock::DockSide {
            match side {
                DockSide::Left => dock::DockSide::Left,
                DockSide::Right => dock::DockSide::Right,
                DockSide::Top => dock::DockSide::Top,
                DockSide::Bottom => dock::DockSide::Bottom,
            }
        }
        fn span(span: panes::Span) -> PaneSpan {
            PaneSpan { scale: span.scale, offset: span.offset }
        }
        fn fill_dock(models: &DockModels, dock: Option<&dock::Dock>, icons: &Icons_) {
            let Some(dock) = dock else {
                sync(&models.edges, Vec::new());
                models.tabs.iter().for_each(|tabs| sync(tabs, Vec::new()));
                sync(&models.floating, Vec::new());
                return;
            };
            let edges = dock::DockSide::ALL
                .iter()
                .map(|side| DockEdge {
                    active: dock.active(*side).map(|panel| panel.id.as_str()).unwrap_or("").into(),
                    size: dock.size(*side),
                })
                .collect();
            sync(&models.edges, edges);
            for (ix, side) in dock::DockSide::ALL.iter().enumerate() {
                let tabs = dock
                    .tabs(*side)
                    .into_iter()
                    .map(|(panel, selected)| DockTab {
                        id: panel.id.as_str().into(),
                        title: panel.title.as_str().into(),
                        icon: icons.get(&panel.icon).cloned().unwrap_or_default(),
                        selected,
                    })
                    .collect();
                sync(&models.tabs[ix], tabs);
            }
            let floating = dock
                .floating()
                .into_iter()
                .map(|(panel, x, y)| DockFloatSpot { id: panel.id.as_str().into(), title: panel.title.as_str().into(), x, y })
                .collect();
            sync(&models.floating, floating);
        }
        fn fill_group(models: &mut GroupModels, group: Option<&panes::PaneGroup>) {
            let Some(group) = group else {
                sync(&models.cells, Vec::new());
                sync(&models.dividers, Vec::new());
                models.handles.clear();
                return;
            };
            let (places, handles) = group.geometry();
            let focused = group.focused();
            let cells = places
                .iter()
                .map(|place| PaneCell {
                    id: place.id as i32,
                    focused: place.id == focused,
                    left: span(place.edges.left),
                    right: span(place.edges.right),
                    top: span(place.edges.top),
                    bottom: span(place.edges.bottom),
                })
                .collect();
            sync(&models.cells, cells);
            let dividers = handles
                .iter()
                .map(|handle| PaneDivider {
                    horizontal: handle.axis == panes::Axis::Horizontal,
                    left: span(handle.edges.left),
                    right: span(handle.edges.right),
                    top: span(handle.edges.top),
                    bottom: span(handle.edges.bottom),
                })
                .collect();
            sync(&models.dividers, dividers);
            models.handles = handles;
        }

        let app = &$app;
        let icons: Rc<Icons_> = Rc::new({
            use ::slint::Model;
            app.global::<Icons>().get_all().iter().map(|entry| (entry.name.to_string(), entry.image)).collect()
        });
        let host = $crate::workspace::WorkspaceHost::new();
        let models = Rc::new(RefCell::new(Models::default()));
        {
            let (models, icons) = (models.clone(), icons.clone());
            host.on_redraw(move |systems| {
                let mut models = models.borrow_mut();
                for name in systems.docks.keys() {
                    models.docks.entry(name.clone()).or_default();
                }
                for (name, dock_models) in &models.docks {
                    fill_dock(dock_models, systems.docks.get(name), &icons);
                }
                for name in systems.groups.keys() {
                    models.groups.entry(name.clone()).or_default();
                }
                for (name, group_models) in models.groups.iter_mut() {
                    fill_group(group_models, systems.groups.get(name));
                }
            });
        }
        // Each getter hands out the same models for a name, filled when first asked for.
        let docks = app.global::<Docks>();
        {
            let (models, host, icons) = (models.clone(), host.clone(), icons.clone());
            docks.on_edges(move |name| {
                let mut models = models.borrow_mut();
                let entry = models.docks.entry(name.to_string()).or_insert_with(|| {
                    let fresh = DockModels::default();
                    host.read(|systems| fill_dock(&fresh, systems.docks.get(name.as_str()), &icons));
                    fresh
                });
                ::slint::ModelRc::from(entry.edges.clone())
            });
        }
        {
            let (models, host, icons) = (models.clone(), host.clone(), icons.clone());
            docks.on_tabs(move |name, side| {
                let mut models = models.borrow_mut();
                let entry = models.docks.entry(name.to_string()).or_insert_with(|| {
                    let fresh = DockModels::default();
                    host.read(|systems| fill_dock(&fresh, systems.docks.get(name.as_str()), &icons));
                    fresh
                });
                let ix = dock::DockSide::ALL.iter().position(|each| *each == side_of(side)).unwrap_or(0);
                ::slint::ModelRc::from(entry.tabs[ix].clone())
            });
        }
        {
            let (models, host, icons) = (models.clone(), host.clone(), icons.clone());
            docks.on_floating(move |name| {
                let mut models = models.borrow_mut();
                let entry = models.docks.entry(name.to_string()).or_insert_with(|| {
                    let fresh = DockModels::default();
                    host.read(|systems| fill_dock(&fresh, systems.docks.get(name.as_str()), &icons));
                    fresh
                });
                ::slint::ModelRc::from(entry.floating.clone())
            });
        }
        // Acts on dock `name` if it is there and holds the panel `id`.
        fn with_panel(host: &$crate::workspace::WorkspaceHost, name: &str, id: &str, act: impl FnOnce(&mut dock::Dock)) {
            host.update(|systems| match systems.docks.get_mut(name) {
                Some(dock) if dock.find(id).is_some() => act(dock),
                Some(_) => $crate::log::error!("dock {name}: no panel {id}"),
                None => $crate::log::error!("no dock {name}"),
            });
        }
        {
            let host = host.clone();
            docks.on_activate(move |name, id| with_panel(&host, &name, &id, |dock| dock.activate(&id)));
        }
        {
            let host = host.clone();
            docks.on_dock_panel(move |name, id, side| {
                with_panel(&host, &name, &id, |dock| dock.move_panel(&id, dock::Spot::Docked(side_of(side))))
            });
        }
        {
            let host = host.clone();
            docks.on_float_panel(move |name, id, x, y| {
                with_panel(&host, &name, &id, |dock| dock.move_panel(&id, dock::Spot::Floating { x, y }))
            });
        }
        {
            let host = host.clone();
            docks.on_floated(move |name, id, x, y| {
                host.quietly(|systems| match systems.docks.get_mut(name.as_str()) {
                    Some(dock) if dock.find(&id).is_some() => dock.floated(&id, x, y),
                    _ => $crate::log::error!("dock {name}: no panel {id}"),
                });
            });
        }
        {
            let host = host.clone();
            docks.on_resize(move |name, side, span, extent| {
                host.update(|systems| match systems.docks.get_mut(name.as_str()) {
                    Some(dock) => dock.resize(side_of(side), span, extent),
                    None => $crate::log::error!("no dock {name}"),
                });
            });
        }

        let groups = app.global::<PaneGroups>();
        {
            let (models, host) = (models.clone(), host.clone());
            groups.on_cells(move |name| {
                let mut models = models.borrow_mut();
                let entry = models.groups.entry(name.to_string()).or_insert_with(|| {
                    let mut fresh = GroupModels::default();
                    host.read(|systems| fill_group(&mut fresh, systems.groups.get(name.as_str())));
                    fresh
                });
                ::slint::ModelRc::from(entry.cells.clone())
            });
        }
        {
            let (models, host) = (models.clone(), host.clone());
            groups.on_dividers(move |name| {
                let mut models = models.borrow_mut();
                let entry = models.groups.entry(name.to_string()).or_insert_with(|| {
                    let mut fresh = GroupModels::default();
                    host.read(|systems| fill_group(&mut fresh, systems.groups.get(name.as_str())));
                    fresh
                });
                ::slint::ModelRc::from(entry.dividers.clone())
            });
        }
        fn with_group(host: &$crate::workspace::WorkspaceHost, name: &str, act: impl FnOnce(&mut panes::PaneGroup)) {
            host.update(|systems| match systems.groups.get_mut(name) {
                Some(group) => act(group),
                None => $crate::log::error!("no pane group {name}"),
            });
        }
        {
            let host = host.clone();
            groups.on_split(move |name, pane, horizontal| {
                let axis = if horizontal { panes::Axis::Horizontal } else { panes::Axis::Vertical };
                with_group(&host, &name, |group| {
                    group.split(pane.max(0) as u64, axis);
                })
            });
        }
        {
            let host = host.clone();
            groups.on_close(move |name, pane| with_group(&host, &name, |group| group.close(pane.max(0) as u64)));
        }
        {
            let host = host.clone();
            groups.on_focus(move |name, pane| with_group(&host, &name, |group| group.focus(pane.max(0) as u64)));
        }
        {
            let (models, host) = (models.clone(), host.clone());
            groups.on_grab(move |name, index, width, height| {
                let handle = models.borrow().groups.get(name.as_str()).and_then(|m| m.handles.get(index.max(0) as usize).cloned());
                match handle {
                    Some(handle) => host.quietly(|systems| {
                        if let Some(group) = systems.groups.get_mut(name.as_str()) {
                            group.grab(&handle, width, height);
                        }
                    }),
                    None => $crate::log::error!("pane group {name}: no divider {index}"),
                }
            });
        }
        {
            let host = host.clone();
            groups.on_drag(move |name, delta| with_group(&host, &name, |group| group.drag(delta)));
        }
        host
    }};
}

/// Wires `ColumnShells` to [`columns::place`](crate::columns::place). The host's root `.slint`
/// file exports `ColumnShells`, `ShellColumnSpec`, `ShellRoom` and `ShellColumnFrame`.
#[macro_export]
macro_rules! wire_column_shell {
    ($app:expr) => {{
        use ::slint::Model;
        use $crate::columns::{self, Column, Room, Side};
        $app.global::<ColumnShells>().on_layout(|specs, widths, width, height, room| {
            let specs: Vec<Column> = specs
                .iter()
                .enumerate()
                .map(|(ix, spec)| Column {
                    width: (!spec.fills).then(|| widths.row_data(ix).unwrap_or(0.0)),
                    resizable: spec.resizable,
                    min: spec.min,
                    max: spec.max,
                })
                .collect();
            let room = Room { first_pad: room.first_pad, lead: room.lead, trail: room.trail, flush: room.flush };
            let frames: Vec<ShellColumnFrame> = columns::place(&specs, width, room)
                .into_iter()
                .map(|placed| ShellColumnFrame {
                    x: placed.x,
                    width: placed.width,
                    height,
                    border_left: placed.side == Some(Side::Left),
                    bordered: placed.side.is_some(),
                    resizes: placed.resizes,
                    items_x: placed.items_x,
                    items_width: placed.items_width,
                })
                .collect();
            ::slint::ModelRc::new(::slint::VecModel::from(frames))
        });
    }};
}
