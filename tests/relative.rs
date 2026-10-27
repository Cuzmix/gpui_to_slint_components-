// RelativeDatePicker, after ely-core/src/forms/date/relative.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { RelativeDatePicker, RelativeRange, RelativeRanges, DateSpan, OptionLayer, CivilDate, IsoWeek, Dates, Choice, Choices, Toggles } from "ui/index.slint";
    export { CivilDate, IsoWeek, Dates, Choice, Choices, Toggles, RelativeRange }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <RelativeRange> range: RelativeRange.last-7-days;
        in-out property <CivilDate> first;
        in-out property <CivilDate> last;
        out property <bool> open: window-pick.open;
        out property <[DateSpan]> spans: [
            RelativeRanges.span(RelativeRange.last-7-days, { year: 2026, month: 9, day: 25 }),
            RelativeRanges.span(RelativeRange.last-month, { year: 2026, month: 9, day: 25 }),
            RelativeRanges.span(RelativeRange.this-quarter, { year: 2026, month: 9, day: 25 }),
            RelativeRanges.span(RelativeRange.this-year, { year: 2026, month: 9, day: 25 }),
            RelativeRanges.span(RelativeRange.yesterday, { year: 2026, month: 3, day: 1 }),
        ];
        public function focus-pick() { window-pick.focus(); }
        window-pick := RelativeDatePicker {
            x: 10px; y: 10px; width: 220px;
            today: { year: 2026, month: 9, day: 25 };
            range <=> root.range;
            range-changed(range, first, last) => { root.first = first; root.last = last; }
        }
        OptionLayer { }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_dates!(h);
    ely_slint::wire_options!(h);
    ely_slint::wire_toggles!(h);
    show(&h);
    h
}

fn ymd(d: CivilDate) -> (i32, i32, i32) {
    (d.year, d.month, d.day)
}

fn spans_count_back_from_today() {
    use slint::Model;
    let h = harness();
    let spans: Vec<_> = h.get_spans().iter().map(|s| (ymd(s.first), ymd(s.last))).collect();
    assert_eq!(spans[0], ((2026, 9, 19), (2026, 9, 25)));
    assert_eq!(spans[1], ((2026, 8, 1), (2026, 8, 31)));
    assert_eq!(spans[2], ((2026, 7, 1), (2026, 9, 30)));
    assert_eq!(spans[3], ((2026, 1, 1), (2026, 12, 31)));
    assert_eq!(spans[4].0, (2026, 2, 28));
}

fn a_pick_reports_the_span() {
    let h = harness();
    let shown = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "Any time").next().unwrap();
    assert_eq!(shown.accessible_value().unwrap_or_default().as_str(), "Last 7 days");
    h.invoke_focus_pick();
    tap(h.window(), " ");
    assert!(h.get_open());
    tap(h.window(), Key::DownArrow);
    tap(h.window(), Key::DownArrow);
    tap(h.window(), Key::Return);
    assert_eq!(h.get_range(), RelativeRange::ThisMonth);
    assert_eq!((ymd(h.get_first()), ymd(h.get_last())), ((2026, 9, 1), (2026, 9, 30)));
    h.set_range(RelativeRange::Today);
    advance(0);
    let shown = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "Any time").next().unwrap();
    assert_eq!(shown.accessible_value().unwrap_or_default().as_str(), "Today");
}

cases!(relative: spans_count_back_from_today, a_pick_reports_the_span);
