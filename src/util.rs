use std::{ffi::OsStr, path::Path};

use chrono::{Datelike, Local, TimeZone, Timelike, Utc};
use toml::value::{Date, Datetime, Time};

pub const LISTEN_DEFAULT_ADDRESS: &str = "0.0.0.0";
pub const LISTEN_DEFAULT_PORT: u16 = 12345;

pub fn is_ipv4_address(address: &str) -> bool {
    address.parse::<std::net::Ipv4Addr>().is_ok()
}

pub fn basename<'path>(path: &'path str) -> &'path str {
    path.rfind("/").map(|pos| &path[pos + 1..]).unwrap_or(path)
}

/// Ensures that the given folder does exist.
pub fn ensure_folder(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(path);

    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }

    Ok(())
}

/// Returns a filename that doesn't collide with the existing files.
pub fn non_colliding_filename(path: &str) -> String {
    let mut i = 1;
    let path = Path::new(path);

    if !path.exists() {
        return path.to_str().unwrap().to_string();
    }

    let filename = path.file_stem().unwrap_or(OsStr::new("")).to_str().unwrap();
    let extension = path.extension().unwrap_or(OsStr::new("")).to_str().unwrap();
    let source = path.parent().unwrap_or(Path::new(".")).to_str().unwrap();

    let mut path = format!("{}/{}{}", source, filename, extension);

    while Path::new(&path).exists() {
        path = format!("{}/{}_{}.{}", source, filename, i, extension);
        i += 1;
    }

    path
}

pub fn chrono_to_toml_date(date: chrono::NaiveDate) -> Date {
    Date {
        year: date.year() as u16,
        month: date.month() as u8,
        day: date.day() as u8,
    }
}

pub fn chrono_to_toml_datetime(date: chrono::DateTime<Utc>) -> Datetime {
    let tdate = chrono_to_toml_date(date.date_naive());
    let time = Time {
        hour: date.hour() as u8,
        minute: date.minute() as u8,
        second: date.second() as u8,
        nanosecond: 0,
    };

    Datetime {
        date: Some(tdate),
        time: Some(time),
        offset: None,
    }
}

pub fn toml_to_chrono_datetime(date: Datetime) -> chrono::NaiveDateTime {
    let time = date.time.unwrap();
    let date = date.date.unwrap();

    let date =
        chrono::NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32);
    let time =
        chrono::NaiveTime::from_hms_opt(time.hour as u32, time.minute as u32, time.second as u32);

    match (date, time) {
        (Some(date), Some(time)) => date.and_time(time),
        _ => panic!("Invalid date or time"),
    }
}

pub fn fmt_datetime(date: chrono::NaiveDateTime) -> String {
    let date = Utc.from_utc_datetime(&date);
    let date = date.with_timezone(&Local);

    date.format("%Y/%m/%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_name_collision() {
        // Some example files
        let files = vec![
            "/tmp/file.txt",
            "/tmp/file_1.txt",
            "/tmp/file.txt.gz",
            "/tmp/file_3.txt",
        ];

        files.iter().for_each(|file| {
            if !std::path::Path::new(file).exists() {
                std::fs::File::create(file).expect("Failed to create file");
            }
        });

        let non_colliding = files
            .iter()
            .map(|file| super::non_colliding_filename(file))
            .collect::<Vec<String>>();

        non_colliding.iter().for_each(|file| {
            assert!(
                !files.contains(&file.as_str()),
                "The file `{}` is colliding with the original files",
                file
            );
            assert!(
                !std::path::Path::new(&file).exists(),
                "The file `{}` souldn't exist",
                file
            );
        });
    }
}
