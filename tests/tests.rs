use std::process::Command;
use std::str;

fn run_command(program_name: &str, args: &[&str]) -> (String, String, i32) {
    let mut cmd = Command::new(format!("target/debug/{}", program_name));
    cmd.args(args);

    let output = cmd.output().expect("Failed to execute command");

    let stdout = str::from_utf8(&output.stdout)
        .unwrap_or("")
        .trim()
        .to_string();
    let stderr = str::from_utf8(&output.stderr)
        .unwrap_or("")
        .trim()
        .to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    (stdout, stderr, exit_code)
}

#[test]
fn test_jcal_current_month() {
    let (stdout, stderr, exit_code) = run_command("jcal", &[]);

    assert_eq!(
        exit_code, 0,
        "jcal should exit with 0 when run with no arguments. Stderr: {}",
        stderr
    );
    assert!(
        stdout.lines().count() > 2,
        "jcal output should have more than 2 lines for a calendar month. Stdout: {}\\nStderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_jcal_specific_year() {
    // Test `jcal 1400` (Year 1400)
    let (stdout, stderr, exit_code) = run_command("jcal", &["1400"]);
    assert_eq!(
        exit_code, 0,
        "jcal 1400 should exit with 0. Stderr: {}",
        stderr
    );
    // Check for the year and month names separately, as formatting may vary.
    assert!(
        stdout.contains("1400") && stdout.contains("Farvardin"),
        "Year view for 1400 should contain '1400' and 'Farvardin'. Stdout: {}\\nStderr: {}",
        stdout,
        stderr
    );
    assert!(
        stdout.contains("1400") && stdout.contains("Esfand"),
        "Year view for 1400 should contain '1400' and 'Esfand'. Stdout: {}\\nStderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_jdate_current_date() {
    // This test assumes `jdate` with no arguments prints the current Jalali date.
    let (stdout, stderr, exit_code) = run_command("jdate", &[]);

    assert_eq!(exit_code, 0, "jdate should exit with 0. Stderr: {}", stderr);

    // Expected format: Weekday Month Day HH:MM:SS UTC Year
    // e.g., "Pan Ordibehesht 18 15:02:48 UTC 1404"
    let parts: Vec<&str> = stdout.split_whitespace().collect();
    assert_eq!(
        parts.len(),
        6,
        "jdate output should be 6 parts (Weekday Month Day HH:MM:SS UTC Year). Stdout: {}\nStderr: {}",
        stdout,
        stderr
    );

    // Check if the last part (year) is a number
    assert!(
        parts[5].parse::<u32>().is_ok(),
        "Year (last part) should be a number. Stdout: {}\nStderr: {}",
        stdout,
        stderr
    );
    // Check if the third part (day) is a number
    assert!(
        parts[2].parse::<u32>().is_ok(),
        "Day (third part) should be a number. Stdout: {}\nStderr: {}",
        stdout,
        stderr
    );
    // Check for presence of "UTC"
    assert!(
        stdout.contains("UTC"),
        "Output should contain 'UTC'. Stdout: {}\nStderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_jdate_jalali_to_gregorian() {
    // Test `jdate -g 1399/1/1` for Jalali to Gregorian
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1399/1/1"]);

    assert_eq!(
        exit_code, 0,
        "jdate -g 1399/1/1 should exit with 0. Stderr: {}",
        stderr
    );
    // Example output: "Fri Mar 20 00:00:00 UTC 2020"
    assert!(
        stdout.contains("2020") && stdout.contains("Mar") && stdout.contains("20"),
        "Expected Gregorian 2020-Mar-20 for Jalali 1399-1-1. Got: {}. Stderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_jdate_gregorian_to_jalali() {
    // Test `jdate -j 2020/3/20` for Gregorian to Jalali
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "2020/3/20"]);

    assert_eq!(
        exit_code, 0,
        "jdate -j 2020/3/20 should exit with 0. Stderr: {}",
        stderr
    );
    // Example output: "Jom Farvardin 01 00:00:00 UTC 1399"
    assert!(
        stdout.contains("1399") && stdout.contains("Farvardin") && stdout.contains("01"),
        "Expected Jalali 1399-Farvardin-01 for Gregorian 2020-3-20. Got: {}. Stderr: {}",
        stdout,
        stderr
    );
}

#[test]
fn test_g2j_unix_epoch() {
    // Unix epoch: 1970/1/1
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "1970/1/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Pan")
            && stdout.contains("Dey")
            && stdout.contains("11")
            && stdout.contains("1348"),
        "Expected 'Pan Dey 11 ... 1348'. Got: {}",
        stdout
    );
}

#[test]
fn test_g2j_before_unix_epoch() {
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "1969/12/31"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Cha")
            && stdout.contains("Dey")
            && stdout.contains("10")
            && stdout.contains("1348"),
        "Expected 'Cha Dey 10 ... 1348'. Got: {}",
        stdout
    );
}

#[test]
fn test_g2j_after_unix_epoch() {
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "1970/1/2"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Jom")
            && stdout.contains("Dey")
            && stdout.contains("12")
            && stdout.contains("1348"),
        "Expected 'Jom Dey 12 ... 1348'. Got: {}",
        stdout
    );
}

#[test]
fn test_g2j_pre_1900() {
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "1899/12/31"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Yek")
            && stdout.contains("Dey")
            && stdout.contains("10")
            && stdout.contains("1278"),
        "Expected 'Yek Dey 10 ... 1278'. Got: {}",
        stdout
    );

    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "1800/1/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Cha")
            && stdout.contains("Dey")
            && stdout.contains("11")
            && stdout.contains("1178"),
        "Expected 'Cha Dey 11 ... 1178'. Got: {}",
        stdout
    );
}

#[test]
fn test_g2j_leap_years_gregorian() {
    // Century leap year: 2000/2/29
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "2000/2/29"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Ses")
            && stdout.contains("Esf")
            && stdout.contains("10")
            && stdout.contains("1378"),
        "Expected 'Ses Esf 10 ... 1378'. Got: {}",
        stdout
    );

    // Normal leap year: 2004/2/29
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "2004/2/29"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Yek")
            && stdout.contains("Esf")
            && stdout.contains("10")
            && stdout.contains("1382"),
        "Expected 'Yek Esf 10 ... 1382'. Got: {}",
        stdout
    );

    // Century non-leap year: 1900/2/28
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "1900/2/28"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Cha")
            && stdout.contains("Esf")
            && stdout.contains("09")
            && stdout.contains("1278"),
        "Expected 'Cha Esf 09 ... 1278'. Got: {}",
        stdout
    );

    // Day after 1900/2/28
    let (stdout_next, stderr_next, exit_code_next) = run_command("jdate", &["-j", "1900/3/1"]);
    assert_eq!(exit_code_next, 0, "Stderr: {}", stderr_next);
    assert!(
        stdout_next.contains("Pan")
            && stdout_next.contains("Esf")
            && stdout_next.contains("10")
            && stdout_next.contains("1278"),
        "Expected 'Pan Esf 10 ... 1278'. Got: {}",
        stdout_next
    );
}

#[test]
fn test_g2j_month_boundaries_gregorian() {
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "2023/1/31"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("1401") && stdout.contains("Bahman") && stdout.contains("11"),
        "Got: {}",
        stdout
    );
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "2023/2/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("1401") && stdout.contains("Bahman") && stdout.contains("12"),
        "Got: {}",
        stdout
    );

    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "2023/12/31"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("1402") && stdout.contains("Dey") && stdout.contains("10"),
        "Got: {}",
        stdout
    );
    let (stdout, stderr, exit_code) = run_command("jdate", &["-j", "2024/1/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("1402") && stdout.contains("Dey") && stdout.contains("11"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_g2j_invalid_gregorian_dates() {
    // Non-leap year, invalid Feb 29
    let (_, stderr, exit_code) = run_command("jdate", &["-j", "2023/2/29"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid date 2023/2/29"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    // Invalid month
    let (_, stderr, exit_code) = run_command("jdate", &["-j", "2023/13/1"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid month 2023/13/1"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    let (_, stderr, exit_code) = run_command("jdate", &["-j", "2023/0/1"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid month 2023/0/1"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    let (_, stderr, exit_code) = run_command("jdate", &["-j", "2023/-1/1"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid month 2023/-1/1"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    // Invalid day
    let (_, stderr, exit_code) = run_command("jdate", &["-j", "2023/1/32"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid day 2023/1/32"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    let (_, stderr, exit_code) = run_command("jdate", &["-j", "2023/1/0"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid day 2023/1/0"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    let (_, stderr, exit_code) = run_command("jdate", &["-j", "2023/1/-10"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid day 2023/1/-10"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );
}



// --- Jalali to Gregorian Conversion Tests ---

#[test]
fn test_j2g_jalali_leap_years() {
    // End of Jalali leap year 1399 (Esfand has 30 days)
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1399/12/30"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2021") && stdout.contains("Mar") && stdout.contains("20"),
        "Expected Greg 2021-Mar-20 for Jal 1399/12/30. Got: {}",
        stdout
    );

    // Start of next Jalali year (1400/1/1)
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1400/1/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2021") && stdout.contains("Mar") && stdout.contains("21"),
        "Expected Greg 2021-Mar-21 for Jal 1400/1/1. Got: {}",
        stdout
    );

    // End of Jalali non-leap year 1398 (Esfand has 29 days)
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1398/12/29"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2020") && stdout.contains("Mar") && stdout.contains("19"),
        "Expected Greg 2020-Mar-19 for Jal 1398/12/29. Got: {}",
        stdout
    );

    // Invalid: 1398/12/30 (non-leap year, Esfand only 29 days)
    let (_, stderr, exit_code) = run_command("jdate", &["-g", "1398/12/30"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid date 1398/12/30"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    // Jalali leap year 1403 (Esfand has 30 days)
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1403/12/30"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2025") && stdout.contains("Mar") && stdout.contains("20"),
        "Expected Greg 2025-Mar-20 for Jal 1403/12/30. Got: {}",
        stdout
    );
}

#[test]
fn test_j2g_month_transitions_jalali() {
    // Sample year 1402 (non-leap)
    // Farvardin (31) to Ordibehesht (31)
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1402/1/31"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2023") && stdout.contains("Apr") && stdout.contains("20"),
        "Got: {}",
        stdout
    );
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1402/2/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2023") && stdout.contains("Apr") && stdout.contains("21"),
        "Got: {}",
        stdout
    );

    // Shahrivar (31) to Mehr (30)
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1402/6/31"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2023") && stdout.contains("Sep") && stdout.contains("22"),
        "Got: {}",
        stdout
    );
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1402/7/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2023") && stdout.contains("Sep") && stdout.contains("23"),
        "Got: {}",
        stdout
    );

    // Bahman (30) to Esfand (29 in 1402)
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1402/11/30"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2024") && stdout.contains("Feb") && stdout.contains("19"),
        "Got: {}",
        stdout
    );
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1402/12/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2024") && stdout.contains("Feb") && stdout.contains("20"),
        "Got: {}",
        stdout
    );

    // End of non-leap year: 1402/12/29 to 1403/1/1
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1402/12/29"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2024") && stdout.contains("Mar") && stdout.contains("19"),
        "Got: {}",
        stdout
    );
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1403/1/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("2024") && stdout.contains("Mar") && stdout.contains("20"),
        "Got: {}",
        stdout
    );
}

#[test]
fn test_j2g_pre_1300sh() {
    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1299/1/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Sun")
            && stdout.contains("Mar")
            && stdout.contains("21")
            && stdout.contains("1920"),
        "Expected 'Sun Mar 21 ... 1920'. Got: {}",
        stdout
    );

    let (stdout, stderr, exit_code) = run_command("jdate", &["-g", "1200/1/1"]);
    assert_eq!(exit_code, 0, "Stderr: {}", stderr);
    assert!(
        stdout.contains("Wed")
            && stdout.contains("Mar")
            && stdout.contains("21")
            && stdout.contains("1821"),
        "Expected 'Wed Mar 21 ... 1821'. Got: {}",
        stdout
    );
}

#[test]
fn test_j2g_invalid_jalali_dates() {
    // Invalid day for a 31-day month (Farvardin)
    let (_, stderr, exit_code) = run_command("jdate", &["-g", "1402/1/32"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid date 1402/1/32"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    // Invalid day for a 30-day month (Mehr)
    let (_, stderr, exit_code) = run_command("jdate", &["-g", "1402/7/31"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid date 1402/7/31"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    // Invalid month
    let (_, stderr, exit_code) = run_command("jdate", &["-g", "1402/13/1"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid month 1402/13/1"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );

    // Invalid day for Esfand in a non-leap year (1400)
    let (_, stderr, exit_code) = run_command("jdate", &["-g", "1400/12/30"]);
    assert_ne!(
        exit_code, 0,
        "Expected non-zero exit for invalid date 1400/12/30"
    );
    assert!(
        stderr.to_lowercase().contains("invalid"),
        "Stderr: {}",
        stderr
    );
}
