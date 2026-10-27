#[macro_use]
mod common;
use slint::ComponentHandle;

slint::slint! {
    import { TextFormat, PlatformStyle, Theme, Clock, Figures, DurationStyle, NumberText, CurrencyText, PercentText, FileSizeText, DurationText, PluralText, RelativeTime, DateTimeText } from "ui/index.slint";
    export { TextFormat, PlatformStyle, Clock, DurationStyle }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <float> value: 1234.5;
        in-out property <int> decimals: 2;
        in-out property <float> amount: 19.99;
        in-out property <string> code: "USD";
        in-out property <float> ratio: 0.082;
        in-out property <bool> signed: true;
        in-out property <float> bytes: 1536000;
        in-out property <bool> binary: false;
        in-out property <int> seconds: 3725;
        in-out property <DurationStyle> style: DurationStyle.clock;
        in-out property <int> count: 3;
        in-out property <int> at: 1000000000;
        in-out property <string> pattern: "%Y-%m-%d %H:%M";
        in-out property <string> zone: "UTC";
        out property <string> number: n.text;
        out property <string> currency: c.text;
        out property <string> percent: p.text;
        out property <bool> percent-gain: p.color == Theme.colors.success;
        out property <bool> percent-loss: p.color == Theme.colors.danger;
        out property <bool> percent-plain: p.color == Theme.colors.fg;
        out property <string> size: f.text;
        out property <string> duration: d.text;
        out property <string> plural: pl.text;
        out property <string> relative: r.text;
        out property <string> datetime: dt.text;
        n := NumberText { value: root.value; decimals: root.decimals; }
        c := CurrencyText { y: 30px; amount: root.amount; code: root.code; }
        p := PercentText { y: 60px; ratio: root.ratio; signed: root.signed; }
        f := FileSizeText { y: 90px; bytes: root.bytes; binary: root.binary; }
        d := DurationText { y: 120px; seconds: root.seconds; style: root.style; }
        pl := PluralText { y: 150px; count: root.count; one: "file"; other: "files"; }
        r := RelativeTime { y: 180px; at: root.at; }
        dt := DateTimeText { y: 210px; at: root.at; pattern: root.pattern; zone: root.zone; }
    }
}

fn wired() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    common::show(&h);
    h
}

fn numbers_group_and_use_a_true_minus() {
    let h = wired();
    assert_eq!(h.get_number(), "1,234.50");
    h.set_value(-3.0);
    h.set_decimals(0);
    assert_eq!(h.get_number(), "\u{2212}3");
    h.set_value(-0.004);
    h.set_decimals(2);
    assert_eq!(h.get_number(), "0.00", "no minus on a zero");
}

fn currencies_take_their_symbol_and_minor_units() {
    let h = wired();
    assert_eq!(h.get_currency(), "$19.99");
    h.set_amount(1200.0);
    h.set_code("JPY".into());
    assert_eq!(h.get_currency(), "¥1,200");
    h.set_amount(5.0);
    h.set_code("CHF".into());
    assert_eq!(h.get_currency(), "CHF 5.00");
}

fn signed_percents_read_in_word_and_color() {
    let h = wired();
    assert_eq!(h.get_percent(), "+8.2%");
    assert!(h.get_percent_gain());
    h.set_ratio(-0.05);
    assert_eq!(h.get_percent(), "\u{2212}5.0%");
    assert!(h.get_percent_loss());
    h.set_signed(false);
    h.set_ratio(0.25);
    assert_eq!(h.get_percent(), "25.0%");
    assert!(h.get_percent_plain());
}

fn file_sizes_pick_their_unit() {
    let h = wired();
    assert_eq!(h.get_size(), "1.5 MB");
    h.set_binary(true);
    assert_eq!(h.get_size(), "1.5 MiB");
    h.set_bytes(999.0);
    assert_eq!(h.get_size(), "999 B");
    h.set_binary(false);
    h.set_bytes(999_950.0);
    assert_eq!(h.get_size(), "1.0 MB", "a rounding to the next unit moves up");
}

fn durations_read_as_a_clock_or_compact() {
    let h = wired();
    let read = |seconds: i32, style: DurationStyle| {
        h.set_seconds(seconds);
        h.set_style(style);
        h.get_duration().to_string()
    };
    let expected = |seconds: u64, style| ely_slint::format::duration(seconds, style).to_string();
    use ely_slint::format::DurationStyle as Rust;
    for seconds in [0, 5, 59, 60, 65, 3599, 3600, 3725, 86_399, 86_400, 90_061, 1_000_000] {
        assert_eq!(read(seconds as i32, DurationStyle::Clock), expected(seconds, Rust::Clock), "{seconds} s");
        assert_eq!(read(seconds as i32, DurationStyle::Compact), expected(seconds, Rust::Compact), "{seconds} s");
    }
}

fn plurals_group_their_count() {
    let h = wired();
    for count in [0u64, 1, 2, 999, 1000, 12_345, 1_000_001, 2_147_483_647] {
        h.set_count(count as i32);
        assert_eq!(h.get_plural().to_string(), ely_slint::format::plural(count, "file", "files"));
    }
}

fn relative_times_round_like_the_source() {
    let h = wired();
    let now = 1_000_000_000i64;
    h.global::<Clock>().set_now(now as i32);
    let ts = |s: i64| jiff::Timestamp::from_second(s).unwrap();
    for ago in [0i64, 10, 44, 45, 60, 89, 90, 600, 2_699, 2_700, 5_399, 5_400, 7_200, 79_199, 79_200, 129_599, 129_600, 86_400 * 3, 86_400 * 25, 86_400 * 26, 86_400 * 44, 86_400 * 45, 86_400 * 100, 86_400 * 319, 86_400 * 320, 86_400 * 400, 86_400 * 547, 86_400 * 548, 86_400 * 2000, -7_200, -86_400 * 3] {
        h.set_at((now - ago) as i32);
        assert_eq!(h.get_relative().to_string(), ely_slint::format::relative(ts(now - ago), ts(now)), "{ago} s ago");
    }
}

fn relative_times_follow_the_clock() {
    let h = wired();
    h.global::<Clock>().set_now(1_000_000_000);
    h.set_at(1_000_000_000);
    assert_eq!(h.get_relative(), "just now");
    h.global::<Clock>().set_now(1_000_000_000 + 180);
    assert_eq!(h.get_relative(), "3 minutes ago");
}

fn dates_read_in_their_zone() {
    let h = wired();
    assert_eq!(h.get_datetime(), "2001-09-09 01:46");
    h.set_zone("Asia/Tokyo".into());
    assert_eq!(h.get_datetime(), "2001-09-09 10:46");
    h.set_pattern("%b %-d, %Y · %H:%M".into());
    assert_eq!(h.get_datetime(), "Sep 9, 2001 · 10:46");
    h.set_zone("Nowhere/Atlantis".into());
    assert_eq!(h.get_datetime(), "", "an unknown zone shows nothing");
}

fn unwired_values_fall_back_to_fixed_digits() {
    let h = Harness::new().unwrap();
    common::show(&h);
    assert_eq!(h.get_number(), "1234.50");
    assert_eq!(h.get_currency(), "USD 19.99");
    assert_eq!(h.get_percent(), "8.2%");
    assert_eq!(h.get_duration(), "01:02:05", "pure Slint needs no host");
}

cases!(values: numbers_group_and_use_a_true_minus, currencies_take_their_symbol_and_minor_units, signed_percents_read_in_word_and_color, file_sizes_pick_their_unit, durations_read_as_a_clock_or_compact, plurals_group_their_count, relative_times_round_like_the_source, relative_times_follow_the_clock, dates_read_in_their_zone, unwired_values_fall_back_to_fixed_digits);
