use chrono::{Datelike, Local, NaiveDate, Timelike};
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
            let now = Local::now();
            let naive_local = now.naive_local();
            let (jy, jm, jd) =
                gregorian_to_jalali(naive_local.year(), naive_local.month(), naive_local.day());

            let weekday_idx = (naive_local.weekday().num_days_from_sunday() + 1) % 7;
            let jalali_weekday_abbr = WEEKDAYS_JALALI[weekday_idx as usize];
            let month_name = MONTH_NAMES[(jm - 1) as usize];
            println!(
                "{} {} {:02} {:02}:{:02}:{:02} UTC {}",
                jalali_weekday_abbr,
                month_name,
                jd,
                naive_local.hour(),
                naive_local.minute(),
                naive_local.second(),
                jy
            );
        }
        (Some(_), Some(_)) => {
            eprintln!("Error: Please specify only one of -g or -j.");
            process::exit(1);
        }
    }
}
