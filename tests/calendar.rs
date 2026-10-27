// Calendar, after ely-core/src/forms/date/calendar.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Calendar, CivilDate, IsoWeek, Dates, DateWords } from "ui/index.slint";
    export { CivilDate, IsoWeek, Dates }
    export component Harness inherits Window {
        width: 300px;
        height: 320px;
        in-out property <CivilDate> chosen: { year: 2026, month: 9, day: 17 };
        in-out property <CivilDate> least;
        in-out property <int> picks;
        in-out property <bool> week;
        out property <CivilDate> month: cal.month;
        out property <CivilDate> cursor: cal.cursor;
        out property <CivilDate> hovered: cal.hovered;
        out property <bool> focused: cal.has-focus;
        out property <string> heading: DateWords.month(cal.month);
        out property <string> day: DateWords.day({ year: 2026, month: 9, day: 5 });
        out property <string> span-month: DateWords.span({ year: 2026, month: 9, day: 3 }, { year: 2026, month: 9, day: 17 });
        out property <string> span-year: DateWords.span({ year: 2026, month: 9, day: 28 }, { year: 2026, month: 10, day: 2 });
        out property <string> span-years: DateWords.span({ year: 2026, month: 12, day: 30 }, { year: 2027, month: 1, day: 2 });
        out property <length> cal-width: cal.width;
        out property <length> cal-height: cal.height;
        public function focus-cal() { cal.focus(); }
        cal := Calendar {
            x: 10px;
            y: 10px;
            selected: root.chosen;
            minimum: root.least;
            week: root.week;
            today: { year: 2026, month: 9, day: 20 };
            picked(date) => { root.chosen = date; root.picks += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_dates!(h);
    show(&h);
    h
}

fn ymd(d: CivilDate) -> (i32, i32, i32) {
    (d.year, d.month, d.day)
}

// The middle of grid cell `ix` (0..42): 28 px cells 2 px apart from (18, y0).
fn cell(h: &Harness, ix: usize) -> (f32, f32) {
    let days = i_slint_backend_testing::ElementHandle::find_by_accessible_label(h, "Aug 31, 2026").next().expect("the first day");
    let (at, _) = (days.absolute_position(), days.size());
    (at.x + 14.0 + (ix % 7) as f32 * 30.0, at.y + 14.0 + (ix / 7) as f32 * 30.0)
}

fn words_read_short() {
    let h = harness();
    assert_eq!(h.get_day(), "Sep 5, 2026");
    assert_eq!(h.get_heading(), "September 2026");
    assert_eq!(h.get_span_month(), "Sep 3 \u{2013} 17, 2026");
    assert_eq!(h.get_span_year(), "Sep 28 \u{2013} Oct 2, 2026");
    assert_eq!(h.get_span_years(), "Dec 30, 2026 \u{2013} Jan 2, 2027");
}

fn it_opens_on_the_chosen_month() {
    let h = harness();
    assert_eq!(ymd(h.get_month()), (2026, 9, 1));
    assert_eq!(ymd(h.get_cursor()), (2026, 9, 17));
    assert_eq!(h.get_cal_width(), 224.0);
    // Six 28 px rows 2 px apart under the header and weekdays.
    assert!(h.get_cal_height() > 6.0 * 28.0 + 10.0 + 16.0);
}

fn keys_move_the_cursor_and_turn_months() {
    let h = harness();
    h.invoke_focus_cal();
    tap(h.window(), Key::RightArrow);
    assert_eq!(ymd(h.get_cursor()), (2026, 9, 18));
    tap(h.window(), Key::DownArrow);
    tap(h.window(), Key::DownArrow);
    assert_eq!(ymd(h.get_cursor()), (2026, 10, 2));
    assert_eq!(ymd(h.get_month()), (2026, 10, 1), "the month follows the cursor");
    tap(h.window(), Key::Home);
    assert_eq!(ymd(h.get_cursor()), (2026, 9, 28));
    assert_eq!(ymd(h.get_month()), (2026, 9, 1));
    tap(h.window(), Key::End);
    assert_eq!(ymd(h.get_cursor()), (2026, 10, 4));
    tap(h.window(), Key::PageUp);
    assert_eq!(ymd(h.get_cursor()), (2026, 9, 4));
    tap(h.window(), Key::Return);
    assert_eq!(ymd(h.get_chosen()), (2026, 9, 4));
    assert_eq!(h.get_picks(), 1);
}

fn a_press_picks_a_day_in_range() {
    let h = harness();
    h.set_least(CivilDate { year: 2026, month: 9, day: 10 });
    let (x, y) = cell(&h, 2); // Sep 2, before the least day.
    click(h.window(), x, y);
    assert_eq!(h.get_picks(), 0);
    let (x, y) = cell(&h, 23); // Sep 23.
    click(h.window(), x, y);
    assert_eq!(ymd(h.get_chosen()), (2026, 9, 23));
    assert!(h.get_focused(), "a press focuses");
}

fn hover_tracks_the_day_and_leaving_clears_it() {
    let h = harness();
    h.set_week(true);
    let (x, y) = cell(&h, 9);
    mouse_move(h.window(), x, y);
    assert_eq!(ymd(h.get_hovered()), (2026, 9, 9));
    mouse_move(h.window(), 290.0, 310.0);
    assert_eq!(h.get_hovered().year, 0);
}

fn the_arrows_turn_the_month() {
    let h = harness();
    click_label(&h, "Next month");
    assert_eq!(ymd(h.get_month()), (2026, 10, 1));
    assert_eq!(ymd(h.get_cursor()), (2026, 10, 17));
    click_label(&h, "Previous month");
    click_label(&h, "Previous month");
    assert_eq!(h.get_heading(), "August 2026");
    advance(300);
}

cases!(calendar: words_read_short, it_opens_on_the_chosen_month, keys_move_the_cursor_and_turn_months, a_press_picks_a_day_in_range, hover_tracks_the_day_and_leaving_clears_it, the_arrows_turn_the_month);
