use std::{
    fs,
    io::{self, BufRead},
};

use super::{Block, BlockComponent};

fn ram_perc() -> i32 {
    let file = fs::File::open("/proc/meminfo").unwrap();
    let reader = io::BufReader::new(file);

    let mut total = 0;
    let mut free = 0;
    let mut buffers = 0;
    let mut cached = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "MemTotal:" => total = parts[1].parse().unwrap_or(0),
            "MemFree:" => free = parts[1].parse().unwrap_or(0),
            "Buffers:" => buffers = parts[1].parse().unwrap_or(0),
            "Cached:" => cached = parts[1].parse().unwrap_or(0),
            _ => continue, // Skip irrelevant lines
        }
    }

    let used = (total - free) - (buffers + cached);
    (100 * used) / total
}

struct RamPercentage {}

impl BlockComponent for RamPercentage {
    fn call(&mut self) -> String {
        ram_perc().to_string()
    }
}
pub fn ram_perc_block() -> Block {
    Block::new(RamPercentage {})
}
