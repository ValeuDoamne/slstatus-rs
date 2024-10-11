
//! # CPU
//! Get information about the CPU

use std::{cell::RefCell, fs::read_to_string};

/// Get the current frequency speed the CPU 0 is running on
pub fn cpu_freq() -> usize {
    read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
        .expect("Could not get current CPU frequency")
        .split_whitespace()
        .next()
        .expect("Could not strip whitespace")
        .parse::<usize>()
        .expect("Could not parse frequency as usize")
        * 1024
}

thread_local! {
    static CPU_VALUES: RefCell<Option<Vec<usize>>> = const { RefCell::new(None) } ;
}

/// Get the average percentage of the CPU used by the machine running
pub fn cpu_prec() -> f64 {
    let stat = read_to_string("/proc/stat").expect("Could not read /proc/stat file");

    let mut cpu_mean = vec![0; 7];
    let mut number_of_cpus = 0;
    for line in stat.lines() {
        if line.starts_with("cpu") {
            number_of_cpus += 1;
            /* cpu user nice system idle iowait irq softirq */
            // We want to do a mean on the number values
            for (idx, num) in read_cpu_line(line).iter().enumerate() {
                cpu_mean[idx] += num;
            }
        } else {
            break;
        }
    }

    for i in cpu_mean.iter_mut() {
        *i /= number_of_cpus;
    }

    if let Some(previous_values) = CPU_VALUES.take() {
        let mut percent: f64 = 0.0;

        let mut sum = 0.0;

        for i in previous_values.iter() {
            sum += *i as f64;
        }

        for i in cpu_mean.iter() {
            sum -= *i as f64;
        }

        if sum == 0.0 {
            CPU_VALUES.set(Some(cpu_mean));
            return 0.0;
        }

        for (idx, val) in previous_values.iter().enumerate() {
            if idx == 3 {
                continue;
            }
            percent += *val as f64;
        }

        for (idx, val) in cpu_mean.iter().enumerate() {
            if idx == 3 {
                continue;
            }
            percent -= *val as f64;
        }

        percent = (percent / sum) * 100.0;
        CPU_VALUES.set(Some(cpu_mean));

        percent.abs()
    } else {
        CPU_VALUES.set(Some(cpu_mean));
        0.0
    }
}

fn read_cpu_line(line: &str) -> Vec<usize> {
    let mut result = vec![];

    for value in line.split_whitespace().skip(1).take(7) {
        let number = value.parse::<usize>().expect("Could not parse as number");
        result.push(number);
    }

    result
}
