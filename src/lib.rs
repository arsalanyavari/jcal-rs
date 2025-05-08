#![allow(clippy::many_single_char_names)]

//! modern Iranian calendar.  Algorithmic outline:
//!   https://en.wikipedia.org/wiki/Solar_Hijri_calendar

use chrono::{Datelike, NaiveDate};

pub const DAYS_IN_LONG_MONTH: u8 = 31;
pub const DAYS_IN_MID_MONTH: u8 = 30;
pub const DAYS_IN_SHORT_MONTH: u8 = 29;
pub const MONTHS_WITH_LONG_DAYS_END: u8 = 6;
pub const MONTHS_WITH_MID_DAYS_END: u8 = 11;
pub const LAST_MONTH_INDEX: u8 = 12;

pub const WEEK_DAYS_TOTAL: u8 = 7;

pub const DAYS_IN_COMMON_YEAR: i64 = 365;
pub const DAYS_IN_LEAP_YEAR: i64 = 366;

pub const GREGORIAN_CE_JDN_OFFSET: i64 = 1_721_425;
pub const UNIX_EPOCH_JDN: i64 = 2_440_588;
pub const JALALI_YEAR_AT_UNIX_EPOCH: i32 = 1_348;
pub const JALALI_YDAY_AT_UNIX_EPOCH: i32 = 286;

pub const LEAP_CYCLE: i32 = 33;
pub const LEAP_REMAINDERS: [i32; 8] = [1, 5, 9, 13, 17, 22, 26, 30];

pub const JDN_MONTH_CORRECTION: i64 = 14;
pub const MONTHS_PER_YEAR: i64 = 12;
pub const JDN_MARCH_ADJUST: i64 = 3;
pub const DAYS_PER_5_MONTH_BLOCK: i64 = 153;
pub const DAYS_PER_5_MONTH_BLOCK_OFFSET: i64 = 2;
pub const JDN_YEAR_SHIFT: i64 = 4800;
pub const JDN_CONSTANT_ADJUST: i64 = 32_045;

pub const MIN_GREGORIAN_YEAR_FOR_JALALI: i32 = 622;
pub const MIN_GREGORIAN_MONTH_FOR_JALALI: u32 = 3;
pub const MIN_GREGORIAN_DAY_FOR_JALALI: u32 = 22;

pub const MONTH_NAMES: [&str; 12] = [
    "Farvardin",
    "Ordibehesht",
    "Khordad",
    "Tir",
    "Mordad",
    "Shahrivar",
    "Mehr",
    "Aban",
    "Azar",
    "Dey",
    "Bahman",
    "Esfand",
];

pub const WEEK_DAYS_AB: [&str; 7] = ["Sh", "Ye", "Do", "Se", "Ch", "Pa", "Jo"];

// Chekc the Jalali year is leap or not
pub fn is_leap(jy: i32) -> bool {
    if jy <= 5 {
        if jy <= 0 {
            return false;
        }
        return jy % 4 == 0;
    }
    let md = jy % LEAP_CYCLE;
    LEAP_REMAINDERS.contains(&md)
}

pub fn days_in_month(jy: i32, jm: u8) -> u8 {
    match jm {
        1..=MONTHS_WITH_LONG_DAYS_END => DAYS_IN_LONG_MONTH,
        7..=MONTHS_WITH_MID_DAYS_END => DAYS_IN_MID_MONTH,
        LAST_MONTH_INDEX => {
            if is_leap(jy) {
                DAYS_IN_MID_MONTH
            } else {
                DAYS_IN_SHORT_MONTH
            }
        }
        _ => panic!("month out of range (1‑12)"),
    }
}

fn jalali_to_jdn_internal(jy: i32, jm: u8, jd: u8) -> i64 {
    let mut input_yday_0_indexed: i32 = 0;
    for m_iter in 1..jm {
        input_yday_0_indexed += days_in_month(jy, m_iter) as i32;
    }
    input_yday_0_indexed += jd as i32 - 1;

    let mut p_days_from_unix_epoch: i64 = 0;

    if jy >= JALALI_YEAR_AT_UNIX_EPOCH {
        for i_y in JALALI_YEAR_AT_UNIX_EPOCH..jy {
            p_days_from_unix_epoch += if is_leap(i_y) {
                DAYS_IN_LEAP_YEAR
            } else {
                DAYS_IN_COMMON_YEAR
            };
        }
        p_days_from_unix_epoch += input_yday_0_indexed as i64 - JALALI_YDAY_AT_UNIX_EPOCH as i64;
    } else {
        for i_y in jy..JALALI_YEAR_AT_UNIX_EPOCH {
            p_days_from_unix_epoch -= if is_leap(i_y) {
                DAYS_IN_LEAP_YEAR
            } else {
                DAYS_IN_COMMON_YEAR
            };
        }
        p_days_from_unix_epoch -= JALALI_YDAY_AT_UNIX_EPOCH as i64 - input_yday_0_indexed as i64;
    }

    p_days_from_unix_epoch + UNIX_EPOCH_JDN
}

pub fn jalali_to_gregorian(jy: i32, jm: u8, jd: u8) -> (i32, u32, u32) {
    if jm == 0 || jm > LAST_MONTH_INDEX || jd == 0 || jd > days_in_month(jy, jm) {
        panic!("Invalid Jalali date: year {}, month {}, day {}", jy, jm, jd);
    }

    let jdn = jalali_to_jdn_internal(jy, jm, jd);

    let days_from_ce = jdn - GREGORIAN_CE_JDN_OFFSET;

    match NaiveDate::from_num_days_from_ce_opt(days_from_ce as i32) {
        Some(date) => (date.year(), date.month(), date.day()),
        None => {
            panic!(
                "Failed to convert JDN {} (days from CE: {}) to Gregorian date.",
                jdn, days_from_ce
            )
        }
    }
}

fn compute_jdn_internal(year: i32, month: u32, day: u32) -> i64 {
    let year_i64 = year as i64;
    let month_i64 = month as i64;
    let day_i64 = day as i64;

    let a = (JDN_MONTH_CORRECTION - month_i64) / MONTHS_PER_YEAR;
    let y = year_i64 + JDN_YEAR_SHIFT - a;
    let m = month_i64 + MONTHS_PER_YEAR * a - JDN_MARCH_ADJUST;

    day_i64
        + (DAYS_PER_5_MONTH_BLOCK * m + DAYS_PER_5_MONTH_BLOCK_OFFSET) / 5
        + DAYS_IN_COMMON_YEAR * y
        + y / 4
        - y / 100
        + y / 400
        - JDN_CONSTANT_ADJUST
}

fn jalali_yday_to_month_day_internal(year: i32, yday: i32) -> (u8, u8) {
    let mut p_day_in_year = yday + 1;
    let mut calculated_month_1_indexed: u8 = 0; // Will store the 1-indexed month. Default to 0 to indicate not found yet.

    // The range 1..LAST_MONTH_INDEX covers 1, 2, ..., 11.
    for current_month_1_indexed_iter in 1..LAST_MONTH_INDEX {
        let days_in_current_month = days_in_month(year, current_month_1_indexed_iter) as i32;
        if p_day_in_year > days_in_current_month {
            p_day_in_year -= days_in_current_month;
        } else {
            // Day falls into this month
            calculated_month_1_indexed = current_month_1_indexed_iter;
            break;
        }
    }

    // calculated_month_1_indexed is still 0, day is in the last month
    if calculated_month_1_indexed == 0 {
        calculated_month_1_indexed = LAST_MONTH_INDEX;
    }

    (calculated_month_1_indexed, p_day_in_year as u8)
}

fn days_offset_to_jalali_internal(days_offset_from_unix_epoch: i64) -> (i32, u8, u8) {
    let mut p_offset = days_offset_from_unix_epoch;
    let mut current_jalali_year = JALALI_YEAR_AT_UNIX_EPOCH;

    p_offset += JALALI_YDAY_AT_UNIX_EPOCH as i64;

    if p_offset >= 0 {
        loop {
            let days_in_current_jalali_year = if is_leap(current_jalali_year) {
                DAYS_IN_LEAP_YEAR
            } else {
                DAYS_IN_COMMON_YEAR
            };
            if p_offset < days_in_current_jalali_year {
                break;
            }
            p_offset -= days_in_current_jalali_year;
            current_jalali_year += 1;
        }
    } else {
        loop {
            current_jalali_year -= 1;
            let days_in_previous_jalali_year = if is_leap(current_jalali_year) {
                DAYS_IN_LEAP_YEAR
            } else {
                DAYS_IN_COMMON_YEAR
            };
            p_offset += days_in_previous_jalali_year;
            if p_offset >= 0 {
                break;
            }
        }
    }

    let final_yday = p_offset as i32;
    let (final_month, final_day) =
        jalali_yday_to_month_day_internal(current_jalali_year, final_yday);
    (current_jalali_year, final_month, final_day)
}

pub fn gregorian_to_jalali(gy: i32, gm: u32, gd: u32) -> (i32, u8, u8) {
    if gy < MIN_GREGORIAN_YEAR_FOR_JALALI
        || (gy == MIN_GREGORIAN_YEAR_FOR_JALALI
            && (gm < MIN_GREGORIAN_MONTH_FOR_JALALI
                || (gm == MIN_GREGORIAN_MONTH_FOR_JALALI && gd < MIN_GREGORIAN_DAY_FOR_JALALI)))
    {
        panic!(
            "Input Gregorian date {}/{}/{} is before 622-03-22 Gregorian (approximate Jalali epoch start) and cannot be converted to Jalali.",
            gy, gm, gd
        );
    }
    let jdn = compute_jdn_internal(gy, gm, gd);

    let days_offset_from_unix_epoch = jdn - UNIX_EPOCH_JDN;

    days_offset_to_jalali_internal(days_offset_from_unix_epoch)
}

// Weekday column (0‑Sat … 6‑Fri) of the first day of a Jalali month.
pub fn first_weekday(jy: i32, jm: u8) -> Option<u8> {
    let (gy, gm, gd) = jalali_to_gregorian(jy, jm, 1);
    match NaiveDate::from_ymd_opt(gy, gm, gd) {
        Some(date) => {
            let sun_based = date.weekday().num_days_from_sunday() as u8;
            Some((sun_based + 1) % WEEK_DAYS_TOTAL)
        }
        None => None,
    }
}
