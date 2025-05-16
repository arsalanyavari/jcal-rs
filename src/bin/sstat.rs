use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Utc};
use clap::Parser;
use scal_lib::*;
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::process;
use std::time::SystemTime;

#[derive(Parser)]
#[command(
    author = "Amir Arsalan Yavari",
    version,
    about = "Shamsi (Jalali) stat displays file or file system status with Shamsi (Jalali) dates",
    name = "sstat"
)]
struct Cli {
    #[arg(required = true, num_args = 1.., help = "File(s) or directory(s) to get status of")]
    paths: Vec<String>,
    #[arg(short = 'p', long, help = "Display Farsi numbers and names")]
    persian_output: bool,
    #[arg(short = 'l', long, help = "Display in ls -lT format")]
    ls_format: bool,
    #[arg(short = 'n', long, help = "Do not print trailing newline")]
    no_newline: bool,
    #[arg(short = 'r', long, help = "Display raw numerical information")]
    raw_format: bool,
    #[arg(short = 'x', long, help = "Display verbose information")]
    verbose_format: bool,
    #[arg(
        short = 's',
        long,
        help = "Display in shell-friendly format for `eval`"
    )]
    shell_format: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut overall_exit_code = 0;
    let mut any_output_printed = false;

    let format_flags_count = [
        &cli.ls_format,
        &cli.raw_format,
        &cli.verbose_format,
        &cli.shell_format,
    ]
    .iter()
    .filter(|&&&flag| flag)
    .count();

    if format_flags_count > 1 {
        eprintln!(
            "Error: Options -l, -r, -s, -x are mutually exclusive. Example: sstat -l /path/to/file"
        );
        process::exit(1);
    }

    let num_str = |num: String, persian: bool| -> String {
        if persian {
            to_persian_numerals(&num)
        } else {
            num
        }
    };

    let mode_oct_str = |num: u32, persian: bool| -> String {
        let oct_s = format!("{:o}", num);
        if persian {
            to_persian_numerals(&oct_s)
        } else {
            oct_s
        }
    };

    for path_str in cli.paths {
        let file_path = Path::new(&path_str);

        match fs::metadata(file_path) {
            Ok(metadata) => {
                any_output_printed = true;
                let device_id = metadata.dev();
                let inode = metadata.ino();
                let mode = metadata.mode();
                let nlink = metadata.nlink();
                let uid = metadata.uid();
                let gid = metadata.gid();
                let rdev = metadata.rdev();
                let size = metadata.len();
                let blksize = metadata.blksize();
                let blocks = metadata.blocks();

                // Raw timestamps (seconds since epoch)
                let atime_raw_sec = metadata.atime();
                let mtime_raw_sec = metadata.mtime();
                let ctime_raw_sec = metadata.ctime();
                let btime_raw_sec = metadata
                    .created()
                    .map(|st| {
                        st.duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs() as i64
                    })
                    .unwrap_or(metadata.ctime());

                let mut atime_local: DateTime<Local> =
                    DateTime::from(metadata.accessed().unwrap_or(SystemTime::UNIX_EPOCH));
                let mut mtime_local: DateTime<Local> =
                    DateTime::from(metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH));
                let mut btime_local: DateTime<Local> =
                    DateTime::from(metadata.created().unwrap_or(SystemTime::UNIX_EPOCH));
                let ctime_utc_for_local = Utc
                    .timestamp_opt(ctime_raw_sec, 0)
                    .single()
                    .unwrap_or_default();
                let mut ctime_local: DateTime<Local> = ctime_utc_for_local.with_timezone(&Local);

                if !cli.raw_format && !cli.shell_format {
                    let correction = Duration::minutes(90); // FIX BUG :/
                    atime_local += correction;
                    mtime_local += correction;
                    btime_local += correction;
                    ctime_local += correction;
                }

                let user_name = users::get_user_by_uid(uid)
                    .map(|u| u.name().to_string_lossy().into_owned())
                    .unwrap_or_else(|| uid.to_string());
                let group_name = users::get_group_by_gid(gid)
                    .map(|g| g.name().to_string_lossy().into_owned())
                    .unwrap_or_else(|| gid.to_string());

                let mode_str_permissions = format_mode(mode);

                if cli.shell_format {
                    print!(
                        "st_dev={} st_ino={} st_mode={} st_nlink={} st_uid={} st_gid={} st_rdev={} st_size={} st_atime={} st_mtime={} st_ctime={} st_birthtime={} st_blksize={} st_blocks={} st_flags={}",
                        num_str(device_id.to_string(), cli.persian_output),
                        num_str(inode.to_string(), cli.persian_output),
                        mode_oct_str(mode, cli.persian_output),
                        num_str(nlink.to_string(), cli.persian_output),
                        num_str(uid.to_string(), cli.persian_output),
                        num_str(gid.to_string(), cli.persian_output),
                        num_str(rdev.to_string(), cli.persian_output),
                        num_str(size.to_string(), cli.persian_output),
                        num_str(atime_raw_sec.to_string(), cli.persian_output),
                        num_str(mtime_raw_sec.to_string(), cli.persian_output),
                        num_str(ctime_raw_sec.to_string(), cli.persian_output),
                        num_str(btime_raw_sec.to_string(), cli.persian_output),
                        num_str(blksize.to_string(), cli.persian_output),
                        num_str(blocks.to_string(), cli.persian_output),
                        num_str("0".to_string(), cli.persian_output) // st_flags=0
                    );
                } else if cli.raw_format {
                    print!(
                        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
                        num_str(device_id.to_string(), cli.persian_output),
                        num_str(inode.to_string(), cli.persian_output),
                        mode_oct_str(mode, cli.persian_output),
                        num_str(nlink.to_string(), cli.persian_output),
                        num_str(uid.to_string(), cli.persian_output),
                        num_str(gid.to_string(), cli.persian_output),
                        num_str(rdev.to_string(), cli.persian_output),
                        num_str(size.to_string(), cli.persian_output),
                        num_str(atime_raw_sec.to_string(), cli.persian_output),
                        num_str(mtime_raw_sec.to_string(), cli.persian_output),
                        num_str(ctime_raw_sec.to_string(), cli.persian_output),
                        num_str(btime_raw_sec.to_string(), cli.persian_output),
                        num_str(blksize.to_string(), cli.persian_output),
                        num_str(blocks.to_string(), cli.persian_output),
                        num_str("0".to_string(), cli.persian_output), // flags = 0
                        path_str
                    );
                } else if cli.verbose_format {
                    let file_type = metadata.file_type();
                    let file_type_str = if file_type.is_dir() {
                        "Directory"
                    } else if file_type.is_file() {
                        "Regular File"
                    } else if file_type.is_symlink() {
                        "Symbolic Link"
                    } else if file_type.is_socket() {
                        "Socket"
                    } else if file_type.is_char_device() {
                        "Character Device"
                    } else if file_type.is_block_device() {
                        "Block Device"
                    } else if file_type.is_fifo() {
                        "FIFO"
                    } else {
                        "Unknown"
                    };

                    let time_format_str = "%a %b %e %H:%M:%S %Y";

                    print!(
                        "  File: \"{}\"\n  Size: {:<12} FileType: {}\n  Mode: ({:04o}/{})     Uid: ({:>5}/ {:>8})  Gid: ({:>5}/ {:>8})\nDevice: {:<7} Inode: {:<10} Links: {}\nAccess: {}\nModify: {}\nChange: {}\n Birth: {}",
                        path_str,
                        size,
                        file_type_str,
                        mode & 0o7777,
                        mode_str_permissions,
                        uid,
                        user_name,
                        gid,
                        group_name,
                        format!("{},{}", device_id >> 8, device_id & 0xFF),
                        inode,
                        nlink,
                        atime_local.format(time_format_str),
                        mtime_local.format(time_format_str),
                        ctime_local.format(time_format_str),
                        btime_local.format(time_format_str)
                    );
                } else if cli.ls_format {
                    let ls_time_format = mtime_local.format("%b %e %H:%M:%S %Y").to_string();
                    print!(
                        "{} {:>3} {} {} {:>7} {} {}",
                        mode_str_permissions,
                        num_str(nlink.to_string(), cli.persian_output),
                        user_name,
                        group_name,
                        num_str(size.to_string(), cli.persian_output),
                        ls_time_format,
                        path_str
                    );
                } else {
                    // Default Shamsi (Jalali) format
                    let (jy_atime, jm_atime, jd_atime) = gregorian_to_jalali(
                        atime_local.year(),
                        atime_local.month(),
                        atime_local.day(),
                    );
                    let (jy_mtime, jm_mtime, jd_mtime) = gregorian_to_jalali(
                        mtime_local.year(),
                        mtime_local.month(),
                        mtime_local.day(),
                    );
                    let (jy_ctime, jm_ctime, jd_ctime) = gregorian_to_jalali(
                        ctime_local.year(),
                        ctime_local.month(),
                        ctime_local.day(),
                    );
                    let (jy_btime, jm_btime, jd_btime) = gregorian_to_jalali(
                        btime_local.year(),
                        btime_local.month(),
                        btime_local.day(),
                    );

                    let format_jalali_datetime = |jy: i32,
                                                  jm: u8,
                                                  jd: u8,
                                                  dt_local: &DateTime<Local>,
                                                  persian_output: bool|
                     -> String {
                        let month_name = if persian_output {
                            PERSIAN_MONTH_NAMES[(jm - 1) as usize]
                        } else {
                            MONTH_NAMES[(jm - 1) as usize]
                        };
                        let year_str = if persian_output {
                            to_persian_numerals(&jy.to_string())
                        } else {
                            jy.to_string()
                        };
                        let day_str = if persian_output {
                            to_persian_numerals(&jd.to_string())
                        } else {
                            jd.to_string()
                        };
                        let time_val = dt_local.time();
                        let time_str = time_val.format("%H:%M:%S").to_string();
                        let time_str_final = if persian_output {
                            to_persian_numerals(&time_str)
                        } else {
                            time_str
                        };
                        format!("{} {} {} {}", month_name, day_str, time_str_final, year_str)
                    };

                    let atime_jalali_str = format_jalali_datetime(
                        jy_atime,
                        jm_atime,
                        jd_atime,
                        &atime_local,
                        cli.persian_output,
                    );
                    let mtime_jalali_str = format_jalali_datetime(
                        jy_mtime,
                        jm_mtime,
                        jd_mtime,
                        &mtime_local,
                        cli.persian_output,
                    );
                    let ctime_jalali_str = format_jalali_datetime(
                        jy_ctime,
                        jm_ctime,
                        jd_ctime,
                        &ctime_local,
                        cli.persian_output,
                    );
                    let btime_jalali_str = format_jalali_datetime(
                        jy_btime,
                        jm_btime,
                        jd_btime,
                        &btime_local,
                        cli.persian_output,
                    );

                    print!(
                        "{} {} {} {} {} {} {} {} \"{}\" \"{}\" \"{}\" \"{}\" {} {} {} {}",
                        num_str(device_id.to_string(), cli.persian_output),
                        num_str(inode.to_string(), cli.persian_output),
                        mode_str_permissions,
                        num_str(nlink.to_string(), cli.persian_output),
                        user_name,
                        group_name,
                        num_str(rdev.to_string(), cli.persian_output),
                        num_str(size.to_string(), cli.persian_output),
                        atime_jalali_str,
                        mtime_jalali_str,
                        ctime_jalali_str,
                        btime_jalali_str,
                        num_str(blksize.to_string(), cli.persian_output),
                        num_str(blocks.to_string(), cli.persian_output),
                        num_str("0".to_string(), cli.persian_output),
                        path_str
                    );
                }

                if !cli.no_newline {
                    println!();
                }
            }
            Err(e) => {
                eprintln!(
                    "Error: Cannot stat \'{}\': {}. Example: sstat /path/to/valid/file",
                    path_str, e
                );
                overall_exit_code = 1;
            }
        }
    }

    if any_output_printed && cli.no_newline {
        // Do nothing
    } else if !any_output_printed && !cli.no_newline {
        println!(); // Clean prompt
    }
    // If !cli.no_newline and output was printed

    if overall_exit_code != 0 {
        process::exit(overall_exit_code);
    }
}

fn format_mode(mode: u32) -> String {
    let type_char = match mode & libc::S_IFMT {
        libc::S_IFREG => '-',  // regular file
        libc::S_IFDIR => 'd',  // directory
        libc::S_IFLNK => 'l',  // symbolic link
        libc::S_IFCHR => 'c',  // character device
        libc::S_IFBLK => 'b',  // block device
        libc::S_IFIFO => 'p',  // pipe
        libc::S_IFSOCK => 's', // socket
        _ => '?',              // unknown
    };

    let mut perms = String::new();
    perms.push(if (mode & libc::S_IRUSR) != 0 {
        'r'
    } else {
        '-'
    });
    perms.push(if (mode & libc::S_IWUSR) != 0 {
        'w'
    } else {
        '-'
    });
    perms.push(match (mode & libc::S_IXUSR, mode & libc::S_ISUID) {
        (0, 0) => '-',
        (_, 0) => 'x',
        (0, _) => 'S',
        (_, _) => 's',
    });

    perms.push(if (mode & libc::S_IRGRP) != 0 {
        'r'
    } else {
        '-'
    });
    perms.push(if (mode & libc::S_IWGRP) != 0 {
        'w'
    } else {
        '-'
    });
    perms.push(match (mode & libc::S_IXGRP, mode & libc::S_ISGID) {
        (0, 0) => '-',
        (_, 0) => 'x',
        (0, _) => 'S',
        (_, _) => 's',
    });

    perms.push(if (mode & libc::S_IROTH) != 0 {
        'r'
    } else {
        '-'
    });
    perms.push(if (mode & libc::S_IWOTH) != 0 {
        'w'
    } else {
        '-'
    });
    perms.push(match (mode & libc::S_IXOTH, mode & libc::S_ISVTX) {
        (0, 0) => '-',
        (_, 0) => 'x',
        (0, _) => 'T',
        (_, _) => 't',
    });

    format!("{}{}", type_char, perms)
}

mod libc {
    #![allow(dead_code)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    pub type mode_t = u32;
    pub const S_IFMT: mode_t = 0o0170000;
    pub const S_IFSOCK: mode_t = 0o140000;
    pub const S_IFLNK: mode_t = 0o120000;
    pub const S_IFREG: mode_t = 0o100000;
    pub const S_IFBLK: mode_t = 0o060000;
    pub const S_IFDIR: mode_t = 0o040000;
    pub const S_IFCHR: mode_t = 0o020000;
    pub const S_IFIFO: mode_t = 0o010000;

    pub const S_ISUID: mode_t = 0o4000;
    pub const S_ISGID: mode_t = 0o2000;
    pub const S_ISVTX: mode_t = 0o1000;

    pub const S_IRWXU: mode_t = 0o0700;
    pub const S_IRUSR: mode_t = 0o0400;
    pub const S_IWUSR: mode_t = 0o0200;
    pub const S_IXUSR: mode_t = 0o0100;

    pub const S_IRWXG: mode_t = 0o0070;
    pub const S_IRGRP: mode_t = 0o0040;
    pub const S_IWGRP: mode_t = 0o0020;
    pub const S_IXGRP: mode_t = 0o0010;

    pub const S_IRWXO: mode_t = 0o0007;
    pub const S_IROTH: mode_t = 0o0004;
    pub const S_IWOTH: mode_t = 0o0002;
    pub const S_IXOTH: mode_t = 0o0001;
}
