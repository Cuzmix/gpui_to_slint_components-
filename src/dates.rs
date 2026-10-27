//! What the date pickers cannot do in `.slint`: walk the calendar. Ported from ely-core
//! `forms/date` (`month_grid`, `week_start`, jiff's saturating spans). Days cross into Slint as
//! `CivilDate { year, month, day }`; a year of 0 is no day.

use jiff::{ToSpan, civil::Date};

/// A day as Slint holds it: year, month (1–12) and day (1–31).
pub type Civil = (i32, i32, i32);

/// The day `(year, month, day)` names, or `None` for no day or one that does not exist.
pub fn date(civil: Civil) -> Option<Date> {
    let (year, month, day) = civil;
    if year == 0 {
        return None;
    }
    Date::new(i16::try_from(year).ok()?, i8::try_from(month).ok()?, i8::try_from(day).ok()?).ok()
}

/// `date` as Slint holds it.
pub fn civil(date: Date) -> Civil {
    (i32::from(date.year()), i32::from(date.month()), i32::from(date.day()))
}

/// Today in the system's zone (UTC when that is unknown), or the frozen capture time.
pub fn today() -> Date {
    crate::format::now().to_zoned(crate::format::system_zone("date pickers")).date()
}

/// Monday of `date`'s week.
pub fn week_start(date: Date) -> Date {
    date.saturating_sub(i64::from(date.weekday().to_monday_zero_offset()).days())
}

/// The 42 days shown for `month`: six weeks, Monday first.
pub fn month_grid(month: Date) -> Vec<Date> {
    let start = week_start(month.first_of_month());
    (0..42).map(|day| start.saturating_add(day.days())).collect()
}

/// `date` moved by `months`, then by `days`; a month too short keeps its last day.
pub fn shift(date: Date, days: i32, months: i32) -> Date {
    date.saturating_add(months.months()).saturating_add(days.days())
}

/// The Monday of ISO week `week` of ISO year `year`, or `None` when that week does not exist.
pub fn monday_of(year: i32, week: i32) -> Option<Date> {
    let (year, week) = (i16::try_from(year).ok()?, i8::try_from(week).ok()?);
    jiff::civil::ISOWeekDate::new(year, week, jiff::civil::Weekday::Monday).ok().map(|week| week.date())
}

/// The ISO year and week holding `date`.
pub fn iso_week(date: Date) -> (i32, i32) {
    let week = date.iso_week_date();
    (i32::from(week.year()), i32::from(week.week()))
}

thread_local! {
    static CLOCKS: std::cell::RefCell<Vec<slint::Timer>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Keeps `tick` running each minute for the rest of the thread, so `Dates.today` turns over.
#[doc(hidden)]
pub fn keep_ticking(tick: impl Fn() + 'static) {
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(60), tick);
    CLOCKS.with(|clocks| clocks.borrow_mut().push(timer));
}

/// Wires the `Dates` global, which every calendar and date picker asks, and keeps
/// `Dates.today` current. The host's root `.slint` file exports `Dates`, `CivilDate` and `IsoWeek`.
///
/// ```ignore
/// ely_slint::wire_dates!(app);
/// ```
#[macro_export]
macro_rules! wire_dates {
    ($app:expr) => {{
        use ::slint::ComponentHandle;
        fn out(date: ::jiff::civil::Date) -> CivilDate {
            let (year, month, day) = $crate::dates::civil(date);
            CivilDate { year, month, day }
        }
        fn take(date: &CivilDate) -> Option<::jiff::civil::Date> {
            $crate::dates::date((date.year, date.month, date.day))
        }
        let dates = $app.global::<Dates>();
        dates.set_today(out($crate::dates::today()));
        let weak = $app.as_weak();
        $crate::dates::keep_ticking(move || {
            if let Some(app) = weak.upgrade() {
                let today = out($crate::dates::today());
                if app.global::<Dates>().get_today() != today {
                    app.global::<Dates>().set_today(today);
                }
            }
        });
        dates.on_grid(|month| {
            let days: Vec<CivilDate> = take(&month).map(|month| $crate::dates::month_grid(month).into_iter().map(out).collect()).unwrap_or_default();
            ::slint::ModelRc::new(::slint::VecModel::from(days))
        });
        dates.on_shift(|date, days, months| take(&date).map(|date| out($crate::dates::shift(date, days, months))).unwrap_or(date));
        dates.on_week_start(|date| take(&date).map(|date| out($crate::dates::week_start(date))).unwrap_or(date));
        dates.on_monday_of(|year, week| $crate::dates::monday_of(year, week).map(out).unwrap_or_default());
        dates.on_iso_week(|date| {
            let (year, week) = take(&date).map($crate::dates::iso_week).unwrap_or_default();
            IsoWeek { year, week }
        });
    }};
}

#[cfg(test)]
mod tests {
    use jiff::civil::date as day;

    use super::*;

    #[test]
    fn a_month_shows_six_weeks_from_a_monday() {
        let days = month_grid(day(2026, 9, 17));
        assert_eq!(days.len(), 42);
        assert_eq!(days[0], day(2026, 8, 31));
        assert_eq!(days[1], day(2026, 9, 1));
        assert_eq!(days[41], day(2026, 10, 11));
        assert_eq!(week_start(day(2026, 9, 27)), day(2026, 9, 21));
    }

    #[test]
    fn shifts_keep_a_short_months_last_day() {
        assert_eq!(shift(day(2026, 1, 31), 0, 1), day(2026, 2, 28));
        assert_eq!(shift(day(2026, 3, 1), -1, 0), day(2026, 2, 28));
        assert_eq!(shift(day(2026, 9, 5), 7, -1), day(2026, 8, 12));
    }

    #[test]
    fn iso_weeks_start_on_their_monday() {
        assert_eq!(monday_of(2026, 39), Some(day(2026, 9, 21)));
        assert_eq!(monday_of(2027, 1), Some(day(2027, 1, 4)));
        assert_eq!(monday_of(2026, 60), None);
        assert_eq!(iso_week(day(2027, 1, 3)), (2026, 53));
    }

    #[test]
    fn civil_days_round_trip_and_year_zero_is_none() {
        assert_eq!(date(civil(day(2026, 10, 23))), Some(day(2026, 10, 23)));
        assert_eq!(date((0, 1, 1)), None);
        assert_eq!(date((2026, 2, 30)), None);
    }
}
