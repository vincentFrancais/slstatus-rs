use std::{
    fs::File,
    io::{self, Read},
};

use super::{Block, BlockComponent};

fn read_cpu_times() -> io::Result<[f64; 7]> {
    let mut file = File::open("/proc/stat")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if let Some(line) = contents.lines().next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 8 {
            return Ok([
                parts[1].parse().unwrap_or(0.0), // user
                parts[2].parse().unwrap_or(0.0), // nice
                parts[3].parse().unwrap_or(0.0), // system
                parts[4].parse().unwrap_or(0.0), // idle
                parts[5].parse().unwrap_or(0.0), // iowait
                parts[6].parse().unwrap_or(0.0), // irq
                parts[7].parse().unwrap_or(0.0), // softirq
            ]);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Failed to parse /proc/stat",
    ))
}

// pub fn cpu_perc_block() -> Block {
//     Block {
//         func: Box::new(|| {
//             let perc = cpu_perc().unwrap_or(0.0).round() as u16;
//             perc.to_string()
//         }),
//     }
// }

#[derive(Default)]
struct CpuPercentage {
    prev: [f64; 7],
}

impl CpuPercentage {
    fn new() -> Self {
        let stats = read_cpu_times().unwrap();

        Self { prev: stats }
    }

    fn get(&mut self) -> f64 {
        let a = read_cpu_times().unwrap();
        let b = self.prev;

        let sum: f64 = (b.iter().sum::<f64>()) - (a.iter().sum::<f64>());

        let used: f64 =
            ((b[0] + b[1] + b[2] + b[5] + b[6]) - (a[0] + a[1] + a[2] + a[5] + a[6])) / sum;

        self.prev = a; // Update previous values

        used * 100.0
    }
}

impl BlockComponent for CpuPercentage {
    fn call(&mut self) -> String {
        (self.get().round() as u64).to_string()
    }
}

pub fn cpu_perc_block() -> Block {
    Block::new(CpuPercentage::new())
}
