// Persian‑style `cal` replacement.

use chrono::{Datelike, Local};
use clap::Parser;
use colored::*;
use jcal_lib::*;

pub const CALENDAR_WIDTH: usize = 21;
pub const DAY_NAME_WIDTH: usize = 3;
pub const FRIDAY_INDEX: usize = 6;
pub const MONTHS_PER_ROW: usize = 3;
pub const COLUMN_SPACING: &str = "  ";
pub const LINE_BUFFER_CAPACITY: usize = 30;
pub const YEAR_HEADER_WIDTH: usize =
    CALENDAR_WIDTH * MONTHS_PER_ROW + COLUMN_SPACING.len() * (MONTHS_PER_ROW - 1);

#[derive(Parser)]
#[command(
    author = "Amir Arsalan Yavari",
    version,
    about = "Jalali calandar like cal command",
    name = "jcal"
)]
struct Cli {
    year: Option<i32>,
}

fn main() {
    let cli = Cli::parse();

    let (cur_jy, cur_jm, cur_jd) = {
        let now = Local::now().naive_local().date();
        gregorian_to_jalali(now.year(), now.month(), now.day())
    };

    match cli.year {
        Some(jy) => print_year(jy, cur_jy, cur_jm, cur_jd),
        None => print_month(cur_jy, cur_jm, cur_jy, cur_jm, cur_jd),
    }
}

fn print_month(jy: i32, jm: u8, cur_jy: i32, cur_jm: u8, cur_jd: u8) {
    let title = format!("{} {}", MONTH_NAMES[(jm - 1) as usize], jy);
    println!("{:^width$}", title, width = CALENDAR_WIDTH);

    for (i, &day_name) in WEEK_DAYS_AB.iter().enumerate() {
        if i == FRIDAY_INDEX {
            print!("{:>width$}", day_name.red(), width = DAY_NAME_WIDTH);
        } else {
            print!("{:>width$}", day_name, width = DAY_NAME_WIDTH);
        }
    }
    println!();

    let first_col = match first_weekday(jy, jm) {
        Some(col) => col,
        None => std::process::exit(1),
    };

    let dim = days_in_month(jy, jm);

    for _ in 0..first_col {
        print!("{}", " ".repeat(DAY_NAME_WIDTH));
    }

    let mut col = first_col;
    for day in 1..=dim {
        let day_num_str = day.to_string();
        let padding_len = DAY_NAME_WIDTH.saturating_sub(day_num_str.chars().count());
        let padding = " ".repeat(padding_len);

        let is_today = jy == cur_jy && jm == cur_jm && day == cur_jd;
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

fn print_year(jy: i32, cur_jy: i32, cur_jm: u8, cur_jd: u8) {
    let mut month_lines: Vec<Vec<String>> = vec![vec![]; 12];

    for m_idx in 0..12 {
        let jm = (m_idx + 1) as u8;
        let month_name = MONTH_NAMES[m_idx];

        month_lines[m_idx].push(format!("{:^width$}", month_name, width = CALENDAR_WIDTH));

        let mut day_names_line = String::with_capacity(LINE_BUFFER_CAPACITY);
        for (i, &day_name) in WEEK_DAYS_AB.iter().enumerate() {
            if i == FRIDAY_INDEX {
                day_names_line.push_str(&format!(
                    "{:>width$}",
                    day_name.red(),
                    width = DAY_NAME_WIDTH
                ));
            } else {
                day_names_line.push_str(&format!("{:>width$}", day_name, width = DAY_NAME_WIDTH));
            }
        }
        month_lines[m_idx].push(day_names_line);

        let first_col = match first_weekday(jy, jm) {
            Some(col) => col,
            None => std::process::exit(1),
        };
        let dim = days_in_month(jy, jm);
        let mut current_line = String::with_capacity(LINE_BUFFER_CAPACITY);

        for _ in 0..first_col {
            current_line.push_str(&" ".repeat(DAY_NAME_WIDTH));
        }

        let mut col = first_col;
        for day in 1..=dim {
            let day_num_str = day.to_string();
            let padding_len = DAY_NAME_WIDTH.saturating_sub(day_num_str.chars().count());
            let padding = " ".repeat(padding_len);

            let is_today = jy == cur_jy && jm == cur_jm && day == cur_jd;
            let is_friday = col as usize == FRIDAY_INDEX;

            let formatted_day_part = if is_today {
                day_num_str.reversed().to_string()
            } else if is_friday {
                day_num_str.red().to_string()
            } else {
                day_num_str
            };
            current_line.push_str(&padding);
            current_line.push_str(&formatted_day_part);

            col += 1;
            if col == WEEK_DAYS_TOTAL {
                month_lines[m_idx].push(current_line);
                current_line = String::with_capacity(LINE_BUFFER_CAPACITY);
                col = 0;
            }
        }
        if !current_line.is_empty() {
            month_lines[m_idx].push(current_line);
        }
    }

    println!("{:^width$}", jy, width = YEAR_HEADER_WIDTH);
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
                print!("{:<width$}", line, width = CALENDAR_WIDTH);
                if idx < MONTHS_PER_ROW - 1 {
                    print!("{COLUMN_SPACING}");
                }
            }
            println!();
        }
        println!();
    }
}
