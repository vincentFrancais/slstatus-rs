use std::{
    fs::File,
    io::{self, Read},
};

use super::Block;

static mut PREV_CPU_TIMES: [f64; 7] = [0.0; 7];

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

fn cpu_perc() -> Option<f64> {
    let a = read_cpu_times().unwrap();

    // TODO: remove this unsafe
    let usage = unsafe {
        let b = PREV_CPU_TIMES;
        if b[0] == 0.0 {
            PREV_CPU_TIMES = a; // Store first measurement
            return None;
        }

        let sum: f64 = (b.iter().sum::<f64>()) - (a.iter().sum::<f64>());
        if sum == 0.0 {
            return None;
        }

        let used: f64 =
            ((b[0] + b[1] + b[2] + b[5] + b[6]) - (a[0] + a[1] + a[2] + a[5] + a[6])) / sum;

        PREV_CPU_TIMES = a; // Update previous values

        used * 100.0
    };

    Some(usage)
}

pub fn cpu_perc_block() -> Block {
    Block {
        func: Box::new(|| {
            let perc = cpu_perc().unwrap_or(0.0).round() as u16;
            perc.to_string()
        }),
    }
}
