pub mod open_source_license;
pub mod proprietary_license;

use chrono::{ DateTime, Local };

pub fn get_time() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%H:%M:%S %Z").to_string()
}

pub fn get_date() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%d-%m-%Y %H:%M:%S %Z").to_string()
}

pub fn get_year() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y").to_string()
}
