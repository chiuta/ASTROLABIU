//! ASTROLABIU core: sky-event primitives.
//!
//! Basis (public textbook mathematics): J. Meeus, *Astronomical Algorithms*, 2nd ed., chapter 7.

/// Julian Day of the epoch J2000.0 (2000-01-01 12:00 TT).
pub const J2000: f64 = 2_451_545.0;

/// Julian Day for a calendar date; `day` may carry a fraction of the day.
///
/// Dates before 1582-10-15 use the Julian calendar, later ones the Gregorian calendar
/// (Meeus ch. 7). Negative (astronomical) years are allowed: year 0 = 1 BC.
pub fn julian_day(year: i32, month: u32, day: f64) -> f64 {
    let (mut y, mut m) = (i64::from(year), i64::from(month));
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    let gregorian = (year, month, day) >= (1582, 10, 15.0);
    let a = (y as f64 / 100.0).floor();
    let b = if gregorian {
        2.0 - a + (a / 4.0).floor()
    } else {
        0.0
    };
    (365.25 * (y as f64 + 4716.0)).floor() + (30.6001 * (m as f64 + 1.0)).floor() + day + b - 1524.5
}

/// Calendar date `(year, month, day-with-fraction)` for a Julian Day (inverse of [`julian_day`]).
pub fn calendar_date(jd: f64) -> (i32, u32, f64) {
    let z = (jd + 0.5).floor();
    let f = jd + 0.5 - z;
    let a = if z < 2_299_161.0 {
        z
    } else {
        let alpha = ((z - 1_867_216.25) / 36_524.25).floor();
        z + 1.0 + alpha - (alpha / 4.0).floor()
    };
    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let d = (365.25 * c).floor();
    let e = ((b - d) / 30.6001).floor();
    let day = b - d - (30.6001 * e).floor() + f;
    let month = if e < 14.0 { e - 1.0 } else { e - 13.0 };
    let year = if month > 2.0 { c - 4716.0 } else { c - 4715.0 };
    (year as i32, month as u32, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn meeus_examples() {
        assert!(close(julian_day(2000, 1, 1.5), J2000));
        assert!(close(julian_day(1987, 1, 27.0), 2_446_822.5));
        assert!(close(julian_day(1957, 10, 4.81), 2_436_116.31));
        assert!(close(julian_day(333, 1, 27.5), 1_842_713.0));
        assert!(close(julian_day(-4712, 1, 1.5), 0.0));
    }

    #[test]
    fn round_trip() {
        for &(y, m, d) in &[
            (2027, 8, 2.5),
            (1582, 10, 4.0),
            (1582, 10, 15.0),
            (-100, 3, 1.25),
            (2000, 2, 29.0),
        ] {
            let (yy, mm, dd) = calendar_date(julian_day(y, m, d));
            assert_eq!((yy, mm), (y, m));
            assert!(close(dd, d), "{y}-{m}-{d}");
        }
    }
}
