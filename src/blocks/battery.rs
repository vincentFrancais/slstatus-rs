use std::fs;

use super::Block;

fn battery_perc(bat: &str) -> String {
    let p = format!("/sys/class/power_supply/{}/capacity", bat);

    let mut perc = fs::read_to_string(p).unwrap();
    perc.truncate(perc.trim_end().len()); // remove the new line char at the end

    perc
}

pub fn battery_perc_block(bat: &str) -> Block {
    let b = bat.to_string();
    Block {
        func: Box::new(move || battery_perc(&b.clone())),
    }
}

fn battery_status(bat: &str) -> String {
    let p = format!("/sys/class/power_supply/{}/status", bat);

    let mut status = fs::read_to_string(p).unwrap();
    status.truncate(status.trim_end().len());

    match status.as_str() {
        "Charging" => "+".into(),
        "Not charging" => "o".into(),
        "Discharging" => "-".into(),
        "Full" => "o".into(),
        &_ => "?".into(),
    }
}

pub fn battery_status_block(bat: &str) -> Block {
    let b = bat.to_string();
    Block {
        func: Box::new(move || battery_status(&b.clone())),
    }
}
