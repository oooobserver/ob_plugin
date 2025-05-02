use crate::{data::Plugin, util};
use std::{error::Error, fs, path::Path, time::UNIX_EPOCH};

pub fn print_recent_file(plugin: &Plugin, path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let path_str = path.display().to_string();

    if path.is_file() {
        return Err("Path should be directory".into());
    } else if path.is_dir() {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() && should_print(&entry_path, plugin.date.as_ref().unwrap())? {
                let stripped_path = entry_path
                    .strip_prefix(&path_str)
                    .expect("entry path and path must have same prefix");
                println!("{}", stripped_path.display());
            }

            if entry_path.is_dir() {
                print_recent_file(plugin, entry_path.to_str().unwrap())?;
            }
        }
    }

    Ok(())
}

// Exclude `imgs` directory
fn should_print(path: &Path, date: &str) -> Result<bool, Box<dyn Error>> {
    if !util::check_path_extention(path.to_str().unwrap()) {
        return Ok(false);
    }

    // Convert YYYY-MM-DD → start and end time
    let start_ts = parse_date_range(date)?;

    let meta = fs::metadata(path)?;
    let modified_ok = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .map(|s| s >= start_ts)
        .unwrap_or(false);

    let created_ok = meta
        .created()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .map(|s| s >= start_ts)
        .unwrap_or(false);

    Ok(modified_ok || created_ok)
}

fn parse_date_range(date: &str) -> Result<u64, Box<dyn Error>> {
    let parts: Vec<u64> = date
        .split('-')
        .map(|s| s.parse::<u64>())
        .collect::<Result<_, _>>()?;

    if parts.len() != 3 {
        return Err("Invalid date format. Expected YYYY-MM-DD".into());
    }

    let (year, month, day) = (parts[0], parts[1], parts[2]);

    let days_since_epoch = days_since_epoch(year, month, day);
    Ok(days_since_epoch * 86400)
}

fn days_since_epoch(year: u64, month: u64, day: u64) -> u64 {
    let y = year as i32;
    let m = month as i32;
    let d = day as i32;

    let (mut y, mut m) = (y, m);
    if m <= 2 {
        y -= 1;
        m += 12;
    }

    let days = 365 * y + y / 4 - y / 100 + y / 400 + (153 * m - 457) / 5 + d - 306;
    let epoch_days = 719468; // days from 0000-03-01 to 1970-01-01
    (days - epoch_days) as u64
}
