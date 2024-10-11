
//! # RAM
//! Get information about the current RAM state of used and availability

use std::{fs::read_to_string, sync::LazyLock};

fn get_ram_size(line: &str) -> usize {
    line.split_whitespace()
        .nth(1)
        .expect("There is no element 1 when spliting whitespace")
        .parse::<usize>()
        .expect("Could not parse RAM number")
}

// We know that the total of RAM will not change during execution of the current program, duh..
static RAM_TOTAL: LazyLock<Option<usize>> = LazyLock::new(|| {
    let mut total = None;
    let meminfo_content = read_to_string("/proc/meminfo").expect("Could not read ram total!");
    for line in meminfo_content.lines() {
        if line.starts_with("MemTotal: ") {
            total = Some(get_ram_size(line));
        }
    }

    total
});

/// Get the total RAM used
pub fn ram_used() -> Option<usize> {
    let meminfo_content = read_to_string("/proc/meminfo").expect("Could not read ram total!");
    let mut free = None;
    let mut buffers = None;
    let mut cached = None;

    for line in meminfo_content.lines() {
        if line.starts_with("MemFree:") {
            free = Some(get_ram_size(line));
        }
        if line.starts_with("Buffers") {
            buffers = Some(get_ram_size(line));
        }
        if line.starts_with("Cached") {
            cached = Some(get_ram_size(line));
        }
    }

    if free.is_some() && buffers.is_some() && cached.is_some() {
        Some(
            (RAM_TOTAL.unwrap() - free.unwrap() - buffers.unwrap() - cached.unwrap()) * 1024 * 1024,
        )
    } else {
        None
    }
}

/// Get the total available RAM on the machine
pub fn ram_total() -> Option<usize> {
    (*RAM_TOTAL).map(|val| val * 1024 * 1024)
}

/// Get the percentage of RAM used
pub fn ram_perc() -> Option<f64> {
    Some(((ram_used()? as f64) / (ram_total()? as f64)) * 100.0)
}
