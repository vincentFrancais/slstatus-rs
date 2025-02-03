use std::fs;

use super::{Block, BlockComponent};

fn battery_perc(bat: &str) -> String {
    let p = format!("/sys/class/power_supply/{}/capacity", bat);

    let mut perc = fs::read_to_string(p).unwrap();
    perc.truncate(perc.trim_end().len()); // remove the new line char at the end

    perc
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

struct BatteryStatus {
    bat: String,
}
struct BatteryPercentage {
    bat: String,
}

impl BlockComponent for BatteryPercentage {
    fn call(&mut self) -> String {
        battery_perc(&self.bat)
    }
}

impl BlockComponent for BatteryStatus {
    fn call(&mut self) -> String {
        battery_status(&self.bat)
    }
}

pub fn battery_status_block(bat: &str) -> Block {
    Block::new(BatteryStatus {
        bat: bat.to_string(),
    })
}

pub fn battery_perc_block(bat: &str) -> Block {
    Block::new(BatteryPercentage {
        bat: bat.to_string(),
    })
}
