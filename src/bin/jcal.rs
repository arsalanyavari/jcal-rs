// Persian‑style `cal` replacement.

use chrono::{Datelike, Local};
use clap::Parser;
use colored::*;
use jcal_lib::*;

pub const BASE_DAY_CELL_WIDTH: usize = 3;
pub const JULIAN_DAY_CELL_WIDTH: usize = 4;

pub const FRIDAY_INDEX: usize = 6;
pub const MONTHS_PER_ROW: usize = 3;
pub const COLUMN_SPACING: &str = "  ";
pub const MAX_DAYS_LINE_WIDTH: usize = JULIAN_DAY_CELL_WIDTH * (jcal_lib::WEEK_DAYS_TOTAL as usize);

#[derive(Parser)]
#[command(
    author = "Amir Arsalan Yavari",
    version,
    about = "Jalali calandar like cal command",
    name = "jcal"
)]
struct Cli {
    year: Option<i32>,
    #[arg(short = 'P', long, help = "Display year based on Pahlavi year")]
    pahlavi: bool,
    #[arg(short = 'p', long, help = "Display Farsi numbers and names")]
    persian_output: bool,
    #[arg(
        short = 'e',
        long,
        help = "Display English weekday names (Sa, Su, ...)"
    )]
    english_days: bool,
    #[arg(short = 'y', long, help = "Display the calendar for the current year")]
    current_year_view: bool,
    #[arg(short = 'j', long, help = "Display Julian dates (day of year)")]
    julian_days: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.persian_output && cli.english_days {
        eprintln!(
            "Error: The -p (Persian output) and -e (English output) options cannot be used together."
        );
        std::process::exit(1);
    }

    if cli.current_year_view && cli.year.is_some() {
        eprintln!("Error: The -y option cannot be used when a specific year is provided.");
        std::process::exit(1);
    }

    let (cur_jy, cur_jm, cur_jd) = {
        let now = Local::now().naive_local().date();
        gregorian_to_jalali(now.year(), now.month(), now.day())
    };

    let pahlavi_active = cli.pahlavi;
    let persian_output_active = cli.persian_output;
    let english_days_active = cli.english_days;
    let current_year_view_active = cli.current_year_view;
    let julian_days_active = cli.julian_days;

    match cli.year {
        Some(y) => {
            let display_jy = if pahlavi_active { y + 1180 } else { y };
            let calc_jy = y; // The original year
            print_year(
                display_jy,
                calc_jy,
                cur_jy,
                cur_jm,
                cur_jd,
                pahlavi_active,
                persian_output_active,
                english_days_active,
                julian_days_active,
            );
        }
        None => {
            if current_year_view_active {
                let display_jy = if pahlavi_active {
                    cur_jy + 1180
                } else {
                    cur_jy
                };
                // For current year
                print_year(
                    display_jy,
                    cur_jy,
                    cur_jy,
                    cur_jm,
                    cur_jd,
                    pahlavi_active,
                    persian_output_active,
                    english_days_active,
                    julian_days_active,
                );
            } else {
                let display_jy = if pahlavi_active {
                    cur_jy + 1180
                } else {
                    cur_jy
                };
                let calc_jy = cur_jy;
                print_month(
                    display_jy,
                    calc_jy,
                    cur_jm,
                    cur_jy,
                    cur_jm,
                    cur_jd,
                    pahlavi_active,
                    persian_output_active,
                    english_days_active,
                    julian_days_active,
                );
            }
        }
    }
}

fn print_month(
    display_jy: i32,
    calc_jy: i32,
    jm: u8,
    cur_jy: i32,
    cur_jm: u8,
    cur_jd: u8,
    pahlavi_active: bool,
    persian_output_active: bool,
    english_days_active: bool,
    julian_days_active: bool,
) {
    let day_cell_width = if julian_days_active {
        JULIAN_DAY_CELL_WIDTH
    } else {
        BASE_DAY_CELL_WIDTH
    };
    let current_calendar_width = day_cell_width * (jcal_lib::WEEK_DAYS_TOTAL as usize);

    let month_name_str = if persian_output_active {
        PERSIAN_MONTH_NAMES[(jm - 1) as usize]
    } else {
        MONTH_NAMES[(jm - 1) as usize]
    };
    let display_year_str = if persian_output_active {
        to_persian_numerals(&display_jy.to_string())
    } else {
        display_jy.to_string()
    };

    let mut title_str = format!("{} {}", month_name_str, display_year_str);
    if pahlavi_active {
        if persian_output_active {
            title_str.push_str(" (پهلوی)");
        } else {
            title_str.push_str(" (Pahlavi)");
        }
    }
    println!("{:^width$}", title_str, width = current_calendar_width);

    let week_days_to_use = if persian_output_active {
        PERSIAN_WEEK_DAYS_AB.as_slice()
    } else if english_days_active {
        ENGLISH_WEEK_DAYS_AB.as_slice()
    } else {
        WEEK_DAYS_AB.as_slice()
    };
    for (i, &day_name) in week_days_to_use.iter().enumerate() {
        if i == FRIDAY_INDEX {
            print!("{:>width$}", day_name.red(), width = day_cell_width);
        } else {
            print!("{:>width$}", day_name, width = day_cell_width);
        }
    }
    println!();

    let first_col = match first_weekday(calc_jy, jm) {
        Some(col) => col,
        None => std::process::exit(1),
    };

    let dim = days_in_month(calc_jy, jm);

    for _ in 0..first_col {
        print!("{}", " ".repeat(day_cell_width));
    }

    let mut col = first_col;
    for day in 1..=dim {
        let day_to_display_num = if julian_days_active {
            jalali_day_of_year(calc_jy, jm, day)
        } else {
            day as i32
        };
        let day_num_str_original = day_to_display_num.to_string();
        let day_num_str = if persian_output_active {
            to_persian_numerals(&day_num_str_original)
        } else {
            day_num_str_original
        };

        let padding_len = day_cell_width.saturating_sub(day_num_str.chars().count());
        let padding = " ".repeat(padding_len);

        let is_today = calc_jy == cur_jy && jm == cur_jm && day == cur_jd;
        let is_friday = col as usize == FRIDAY_INDEX;

        if is_today {
            print!("{}{}", padding, day_num_str.reversed());
        } else if is_friday {
            print!("{}{}", padding, day_num_str.red());
        } else {
            print!("{}{}", padding, day_num_str);
        }

        col += 1;
        if col == WEEK_DAYS_TOTAL {
            col = 0;
            println!();
        }
    }
    if col != 0 {
        println!();
    }
}

fn print_year(
    display_jy: i32,
    calc_jy: i32,
    cur_jy: i32,
    cur_jm: u8,
    cur_jd: u8,
    pahlavi_active: bool,
    persian_output_active: bool,
    english_days_active: bool,
    julian_days_active: bool,
) {
    let day_cell_width = if julian_days_active {
        JULIAN_DAY_CELL_WIDTH
    } else {
        BASE_DAY_CELL_WIDTH
    };
    let current_month_sub_calendar_width = day_cell_width * (jcal_lib::WEEK_DAYS_TOTAL as usize);
    let current_year_header_width = current_month_sub_calendar_width * MONTHS_PER_ROW
        + COLUMN_SPACING.len() * (MONTHS_PER_ROW - 1);

    let mut month_lines: Vec<Vec<String>> = vec![vec![]; 12];

    let active_month_names = if persian_output_active {
        PERSIAN_MONTH_NAMES.as_slice()
    } else {
        MONTH_NAMES.as_slice()
    };
    let active_week_days = if persian_output_active {
        PERSIAN_WEEK_DAYS_AB.as_slice()
    } else if english_days_active {
        ENGLISH_WEEK_DAYS_AB.as_slice()
    } else {
        WEEK_DAYS_AB.as_slice()
    };

    for m_idx in 0..12 {
        let jm = (m_idx + 1) as u8;
        let month_name_str = active_month_names[m_idx];

        month_lines[m_idx].push(format!(
            "{:^width$}",
            month_name_str,
            width = current_month_sub_calendar_width
        ));

        let mut day_names_line = String::with_capacity(MAX_DAYS_LINE_WIDTH);
        for (i, &day_name) in active_week_days.iter().enumerate() {
            if i == FRIDAY_INDEX {
                day_names_line.push_str(&format!(
                    "{:>width$}",
                    day_name.red(),
                    width = day_cell_width
                ));
            } else {
                day_names_line.push_str(&format!("{:>width$}", day_name, width = day_cell_width));
            }
        }
        month_lines[m_idx].push(day_names_line);

        let first_col = match first_weekday(calc_jy, jm) {
            Some(col) => col,
            None => std::process::exit(1),
        };
        let dim = days_in_month(calc_jy, jm);
        let mut current_line = String::with_capacity(MAX_DAYS_LINE_WIDTH);

        for _ in 0..first_col {
            current_line.push_str(&" ".repeat(day_cell_width));
        }

        let mut col = first_col;
        for day in 1..=dim {
            let day_to_display_num = if julian_days_active {
                jalali_day_of_year(calc_jy, jm, day)
            } else {
                day as i32
            };
            let day_num_str_original = day_to_display_num.to_string();

            let day_num_str_display = if persian_output_active {
                to_persian_numerals(&day_num_str_original)
            } else {
                day_num_str_original
            };

            let padding_len = day_cell_width.saturating_sub(day_num_str_display.chars().count());
            let padding = " ".repeat(padding_len);

            let is_today = calc_jy == cur_jy && jm == cur_jm && day == cur_jd;
            let is_friday = col as usize == FRIDAY_INDEX;

            let formatted_day_part = if is_today {
                day_num_str_display.as_str().reversed().to_string()
            } else if is_friday {
                day_num_str_display.as_str().red().to_string()
            } else {
                day_num_str_display
            };
            current_line.push_str(&padding);
            current_line.push_str(&formatted_day_part);

            col += 1;
            if col == WEEK_DAYS_TOTAL {
                month_lines[m_idx].push(current_line);
                current_line = String::with_capacity(MAX_DAYS_LINE_WIDTH);
                col = 0;
            }
        }
        if !current_line.is_empty() {
            month_lines[m_idx].push(current_line);
        }
    }

    let year_header_display_str = if persian_output_active {
        to_persian_numerals(&display_jy.to_string())
    } else {
        display_jy.to_string()
    };
    let year_header_str = if pahlavi_active {
        if persian_output_active {
            format!("{} (پهلوی)", year_header_display_str)
        } else {
            format!("{} (Pahlavi)", year_header_display_str)
        }
    } else {
        year_header_display_str
    };
    println!(
        "{:^width$}",
        year_header_str,
        width = current_year_header_width
    );
    println!();

    for row in 0..4 {
        let indices = [row * 3, row * 3 + 1, row * 3 + 2];
        let max_lines = indices
            .iter()
            .map(|&i| month_lines[i].len())
            .max()
            .unwrap_or(0);

        for line_idx in 0..max_lines {
            for (idx, &m_idx) in indices.iter().enumerate() {
                let line = month_lines[m_idx].get(line_idx).map_or("", |s| s.as_str());
                print!("{:<width$}", line, width = current_month_sub_calendar_width);
                if idx < MONTHS_PER_ROW - 1 {
                    print!("{COLUMN_SPACING}");
                }
            }
            println!();
        }
        println!();
    }
}
