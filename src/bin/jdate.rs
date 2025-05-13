use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use clap::Parser;
use jcal_lib::*;
use std::process;

// Constants for magic numbers
const MAX_MONTH: u32 = 12;
const MAX_DAY: u32 = 31;
const WEEKDAYS_GREGORIAN: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const WEEKDAYS_JALALI: [&str; 7] = ["Sha", "Yek", "Dos", "Ses", "Cha", "Pan", "Jom"];
const MONTH_ABBRS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

#[derive(Debug)]
enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

#[derive(Debug)]
enum AdjustmentType {
    Add(i64),
    Set,
}

#[derive(Debug)]
struct TimeAdjustment {
    value: i64,
    unit: TimeUnit,
    adjustment_type: AdjustmentType,
}

impl TimeAdjustment {
    fn parse(input: &str) -> Result<Self, &'static str> {
        if input.is_empty() {
            return Err("Empty adjustment string");
        }

        let (adjustment_type, rest) = match input.chars().next().unwrap() {
            '+' => (AdjustmentType::Add(1), &input[1..]),
            '-' => (AdjustmentType::Add(-1), &input[1..]),
            _ => (AdjustmentType::Set, input),
        };

        let mut value_str = String::new();
        let mut unit_char = None;

        for c in rest.chars() {
            if c.is_ascii_digit() {
                value_str.push(c);
            } else {
                unit_char = Some(c);
                break;
            }
        }

        let value = value_str.parse::<i64>().map_err(|_| "Invalid number")?;
        let unit = match unit_char {
            Some('y') => TimeUnit::Year,
            Some('m') => TimeUnit::Month,
            Some('w') => TimeUnit::Week,
            Some('d') => TimeUnit::Day,
            Some('H') => TimeUnit::Hour,
            Some('M') => TimeUnit::Minute,
            Some('S') => TimeUnit::Second,
            _ => return Err("Invalid time unit. Use y, m, w, d, H, M, or S"),
        };

        let value = match adjustment_type {
            AdjustmentType::Add(sign) => value * sign,
            AdjustmentType::Set => value,
        };

        Ok(TimeAdjustment {
            value,
            unit,
            adjustment_type,
        })
    }

    fn apply(&self, dt: NaiveDateTime) -> NaiveDateTime {
        match self.adjustment_type {
            AdjustmentType::Add(_) => self.apply_add(dt),
            AdjustmentType::Set => self.apply_set(dt),
        }
    }

    fn apply_add(&self, dt: NaiveDateTime) -> NaiveDateTime {
        match self.unit {
            TimeUnit::Year => {
                let mut new_year = dt.year() + self.value as i32;
                let mut new_month = dt.month();
                let mut new_day = dt.day();

                // Handle month/day overflow
                while new_month > 12 {
                    new_year += 1;
                    new_month -= 12;
                }
                while new_month < 1 {
                    new_year -= 1;
                    new_month += 12;
                }

                // Adjust day if it's invalid for the new month
                let days_in_month = if new_month == 2 {
                    if is_leap(new_year) { 29 } else { 28 }
                } else if [4, 6, 9, 11].contains(&new_month) {
                    30
                } else {
                    31
                };
                new_day = new_day.min(days_in_month);

                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(new_year, new_month, new_day).unwrap(),
                    dt.time(),
                )
            }
            TimeUnit::Month => {
                let mut new_year = dt.year();
                let mut new_month = dt.month() as i32 + self.value as i32;
                let mut new_day = dt.day();

                // Handle month overflow
                while new_month > 12 {
                    new_year += 1;
                    new_month -= 12;
                }
                while new_month < 1 {
                    new_year -= 1;
                    new_month += 12;
                }

                // Adjust day if it's invalid for the new month
                let days_in_month = if new_month == 2 {
                    if is_leap(new_year) { 29 } else { 28 }
                } else if [4, 6, 9, 11].contains(&new_month) {
                    30
                } else {
                    31
                };
                new_day = new_day.min(days_in_month);

                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(new_year, new_month as u32, new_day).unwrap(),
                    dt.time(),
                )
            }
            TimeUnit::Week => dt + Duration::weeks(self.value),
            TimeUnit::Day => dt + Duration::days(self.value),
            TimeUnit::Hour => dt + Duration::hours(self.value),
            TimeUnit::Minute => dt + Duration::minutes(self.value),
            TimeUnit::Second => dt + Duration::seconds(self.value),
        }
    }

    fn apply_set(&self, dt: NaiveDateTime) -> NaiveDateTime {
        let (jy, jm, jd) = gregorian_to_jalali(dt.year(), dt.month(), dt.day());
        let (new_jy, new_jm, new_jd) = match self.unit {
            TimeUnit::Year => {
                let new_jy = self.value as i32;
                (new_jy, jm, jd)
            }
            TimeUnit::Month => {
                let new_jm = self.value as u8;
                if new_jm < 1 || new_jm > 12 {
                    panic!("Invalid Jalali month: {}", new_jm);
                }
                (jy, new_jm, jd)
            }
            TimeUnit::Day => {
                let new_jd = self.value as u8;
                if new_jd < 1 || new_jd > days_in_month(jy, jm) {
                    panic!("Invalid Jalali day: {} for month {}", new_jd, jm);
                }
                (jy, jm, new_jd)
            }
            TimeUnit::Hour => {
                let (gy, gm, gd) = jalali_to_gregorian(jy, jm, jd);
                let new_dt =
                    NaiveDateTime::new(NaiveDate::from_ymd_opt(gy, gm, gd).unwrap(), dt.time())
                        .with_hour(self.value as u32)
                        .unwrap();
                return new_dt;
            }
            TimeUnit::Minute => {
                let (gy, gm, gd) = jalali_to_gregorian(jy, jm, jd);
                let new_dt =
                    NaiveDateTime::new(NaiveDate::from_ymd_opt(gy, gm, gd).unwrap(), dt.time())
                        .with_minute(self.value as u32)
                        .unwrap();
                return new_dt;
            }
            TimeUnit::Second => {
                let (gy, gm, gd) = jalali_to_gregorian(jy, jm, jd);
                let new_dt =
                    NaiveDateTime::new(NaiveDate::from_ymd_opt(gy, gm, gd).unwrap(), dt.time())
                        .with_second(self.value as u32)
                        .unwrap();
                return new_dt;
            }
            TimeUnit::Week => {
                // For week, we'll set to the first day of the specified week
                let week_start = (self.value as i32 - 1) * 7 + 1;
                if week_start < 1 || week_start > 365 {
                    panic!("Invalid week number: {}", self.value);
                }
                let mut day_count = 0;
                let mut new_jm = 1;
                let mut new_jd = 1;

                // Find the month and day for the given week start
                for m in 1..=12 {
                    let days = days_in_month(jy, m) as i32;
                    if day_count + days >= week_start {
                        new_jm = m;
                        new_jd = (week_start - day_count) as u8;
                        break;
                    }
                    day_count += days;
                }
                (jy, new_jm, new_jd)
            }
        };

        let (gy, gm, gd) = jalali_to_gregorian(new_jy, new_jm, new_jd);
        NaiveDateTime::new(NaiveDate::from_ymd_opt(gy, gm, gd).unwrap(), dt.time())
    }
}

#[derive(Parser)]
#[command(
    author = "Amir Arsalan Yavari",
    version,
    about = "Converts between Jalali and Gregorian dates",
    name = "jdate"
)]
struct Cli {
    // Convert Jalali date (YYYY/MM/DD) to Gregorian
    #[arg(short = 'g', long, value_name = "YYYY/MM/DD")]
    jalali_to_gregorian: Option<String>,

    // Convert Gregorian date (YYYY/MM/DD) to Jalali
    #[arg(short = 'j', long, value_name = "YYYY/MM/DD")]
    gregorian_to_jalali: Option<String>,

    // Show UTC time instead of local time
    #[arg(short = 'u', long)]
    utc: bool,

    // Set timezone
    #[arg(short = 'z', long, value_name = "TIMEZONE")]
    timezone: Option<String>,

    // RFC 2822 format
    #[arg(short = 'R', long)]
    rfc2822: bool,

    // ISO 8601 format
    #[arg(short = 'I', long, value_name = "PRECISION", require_equals = true)]
    iso8601: Option<Option<String>>,

    // Adjust time by value[unit]
    #[arg(
        short = 'v',
        long,
        value_name = "[+|-]val[y|m|w|d|H|M|S]",
        allow_hyphen_values = true
    )]
    adjustments: Vec<String>,
}

// Parse YYYY/MM/DD or YYYY-MM-DD
fn parse_date(date_str: &str) -> Result<(i32, u32, u32), &'static str> {
    let parts: Vec<&str> = date_str.split(['/', '-']).collect();
    if parts.len() != 3 {
        return Err("Invalid date format. Use YYYY/MM/DD or YYYY-MM-DD.");
    }

    let year = parts[0].parse::<i32>().map_err(|_| "Invalid year")?;
    let month = parts[1].parse::<u32>().map_err(|_| "Invalid month")?;
    let day = parts[2].parse::<u32>().map_err(|_| "Invalid day")?;

    if month == 0 || month > MAX_MONTH || day == 0 || day > MAX_DAY {
        return Err("Invalid month or day value.");
    }

    Ok((year, month, day))
}

fn format_rfc2822(dt: NaiveDateTime, jy: i32, jm: u8, jd: u8, offset_str: &str) -> String {
    let weekday_idx = (dt.weekday().num_days_from_sunday() + 1) % 7;
    let jalali_weekday_abbr = WEEKDAYS_JALALI[weekday_idx as usize];
    let month_name = MONTH_NAMES[(jm - 1) as usize];
    format!(
        "{}, {} {} {} {:02}:{:02}:{:02} {}",
        jalali_weekday_abbr,
        jd,
        month_name,
        jy,
        dt.hour(),
        dt.minute(),
        dt.second(),
        offset_str
    )
}

fn format_iso8601(dt: NaiveDateTime, jy: i32, jm: u8, jd: u8, precision: Option<&str>) -> String {
    match precision {
        Some("hours") => {
            let offset = if dt.hour() >= 24 {
                format!("+{:02}", dt.hour() / 24)
            } else {
                format!("{:02}", dt.hour())
            };
            format!("{:04}-{:02}-{:02}T{}", jy, jm, jd, offset)
        }
        Some("minutes") => {
            let offset = if dt.hour() >= 24 {
                format!("+{:02}:{:02}", dt.hour() / 24, dt.minute())
            } else {
                format!("{:02}:{:02}", dt.hour(), dt.minute())
            };
            format!("{:04}-{:02}-{:02}T{}", jy, jm, jd, offset)
        }
        Some("seconds") => {
            let offset = if dt.hour() >= 24 {
                format!(
                    "+{:02}:{:02}:{:02}",
                    dt.hour() / 24,
                    dt.minute(),
                    dt.second()
                )
            } else {
                format!("{:02}:{:02}:{:02}", dt.hour(), dt.minute(), dt.second())
            };
            format!("{:04}-{:02}-{:02}T{}", jy, jm, jd, offset)
        }
        _ => format!("{:04}-{:02}-{:02}", jy, jm, jd),
    }
}

fn main() {
    let cli = Cli::parse();

    match (cli.jalali_to_gregorian, cli.gregorian_to_jalali) {
        (Some(jdate_str), None) => {
            // Jalali to Gregorian (-g)
            match parse_date(&jdate_str) {
                Ok((jy, jm, jd)) => {
                    if jd > days_in_month(jy, jm as u8) as u32 {
                        eprintln!("Error: Invalid day {} for month {} in year {}", jd, jm, jy);
                        process::exit(1);
                    }
                    let (gy, gm, gd) = jalali_to_gregorian(jy, jm as u8, jd as u8);

                    match NaiveDate::from_ymd_opt(gy, gm, gd) {
                        Some(naive_date) => {
                            let datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
                            // We'll format manually to match.
                            let weekday = WEEKDAYS_GREGORIAN
                                [datetime.weekday().num_days_from_monday() as usize];
                            let month_abbr = MONTH_ABBRS[(gm - 1) as usize];
                            println!(
                                "{} {} {:02} {:02}:{:02}:{:02} UTC {}",
                                weekday,
                                month_abbr,
                                gd,
                                datetime.hour(),
                                datetime.minute(),
                                datetime.second(),
                                gy
                            );
                        }
                        None => {
                            eprintln!(
                                "Error: Calculated Gregorian date ({}-{}-{}) is invalid.",
                                gy, gm, gd
                            );
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing Jalali date: {}", e);
                    process::exit(1);
                }
            }
        }
        (None, Some(gdate_str)) => {
            // Gregorian to Jalali (-j)
            match parse_date(&gdate_str) {
                Ok((gy, gm, gd)) => {
                    if NaiveDate::from_ymd_opt(gy, gm, gd).is_none() {
                        eprintln!("Error: Invalid Gregorian date specified.");
                        process::exit(1);
                    }
                    let (jy, jm, jd) = gregorian_to_jalali(gy, gm, gd);
                    let (g_conv_y, g_conv_m, g_conv_d) = jalali_to_gregorian(jy, jm, jd);

                    match NaiveDate::from_ymd_opt(g_conv_y, g_conv_m, g_conv_d) {
                        Some(naive_date) => {
                            let weekday_idx = (naive_date.weekday().num_days_from_sunday() + 1) % 7;
                            let jalali_weekday_abbr = WEEKDAYS_JALALI[weekday_idx as usize];
                            let month_name = MONTH_NAMES[(jm - 1) as usize];
                            println!(
                                "{} {} {:02} 00:00:00 UTC {}",
                                jalali_weekday_abbr, month_name, jd, jy
                            );
                        }
                        None => {
                            eprintln!("Error: Internal conversion resulted in invalid date.");
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing Gregorian date: {}", e);
                    process::exit(1);
                }
            }
        }
        (None, None) => {
            // No flags: print current date in Jalali
            let (naive_local, offset_str) = if let Some(tz_str) = &cli.timezone {
                match tz_str.parse::<Tz>() {
                    Ok(tz) => {
                        let tz_now = tz.from_utc_datetime(&Utc::now().naive_utc());
                        (tz_now.naive_local(), tz_now.offset().to_string())
                    }
                    Err(_) => {
                        eprintln!("Error: Invalid timezone '{}'", tz_str);
                        process::exit(1);
                    }
                }
            } else if cli.utc {
                let utc_now = Utc::now();
                (utc_now.naive_local(), "UTC".to_string())
            } else {
                let local_now = Local::now();
                (local_now.naive_local(), local_now.offset().to_string())
            };

            // Apply time adjustments if any
            let adjusted_dt = if !cli.adjustments.is_empty() {
                let mut dt = naive_local;
                for adj_str in &cli.adjustments {
                    match TimeAdjustment::parse(adj_str) {
                        Ok(adj) => dt = adj.apply(dt),
                        Err(e) => {
                            eprintln!("Error parsing adjustment '{}': {}", adj_str, e);
                            process::exit(1);
                        }
                    }
                }
                dt
            } else {
                naive_local
            };

            let (jy, jm, jd) =
                gregorian_to_jalali(adjusted_dt.year(), adjusted_dt.month(), adjusted_dt.day());

            if cli.rfc2822 {
                println!("{}", format_rfc2822(adjusted_dt, jy, jm, jd, &offset_str));
            } else if let Some(precision) = &cli.iso8601 {
                println!(
                    "{}",
                    format_iso8601(adjusted_dt, jy, jm, jd, precision.as_deref())
                );
            } else {
                let weekday_idx = (adjusted_dt.weekday().num_days_from_sunday() + 1) % 7;
                let jalali_weekday_abbr = WEEKDAYS_JALALI[weekday_idx as usize];
                let month_name = MONTH_NAMES[(jm - 1) as usize];

                println!(
                    "{} {} {:02} {:02}:{:02}:{:02} {} {}",
                    jalali_weekday_abbr,
                    month_name,
                    jd,
                    adjusted_dt.hour(),
                    adjusted_dt.minute(),
                    adjusted_dt.second(),
                    offset_str,
                    jy
                );
            }
        }
        (Some(_), Some(_)) => {
            eprintln!("Error: Please specify only one of -g or -j.");
            process::exit(1);
        }
    }
}
