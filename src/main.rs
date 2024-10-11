mod components;
mod config;
mod utils;

use clap::Parser;

use std::{fs::File, io::Write, path::PathBuf, str::FromStr, time::Duration};

use chrono::{Datelike, Timelike};
use components::{
    cpu::cpu_prec,
    datetime::datetime,
    disk::{disk_total, disk_used},
    kernel_release::kernel_release,
    net_speed::{netspeed_rx, netspeed_tx},
    ram::{ram_perc, ram_total, ram_used},
    volume::{volume, volume_icon},
};
use config::TIME_INTERVAL;
use interprocess::os::unix::fifo_file::create_fifo;
use utils::{fmt_human, level_color};

#[derive(Parser)]
struct Arguments {
    /// Pipe location
    #[arg(short, long, default_value=PathBuf::from_str("/tmp/dwl_bar.pipe").expect("Could not decode path").into_os_string())]
    pipe_location: PathBuf,

    /// Print the output to the screen
    #[arg(short, long, action)]
    screen: bool,
}

const NETWORK_INTERFACE: &'static str = "enp42s0";

fn main() {
    let args = Arguments::parse();
    let mut pipe_file = None;

    if !args.screen {
        if !args.pipe_location.exists() {
            create_fifo(&args.pipe_location, 0o600).expect("Could not create FIFO file!");
        }
        pipe_file = Some(
            File::options()
                .write(true)
                .open(args.pipe_location)
                .expect("Could not open FIFO file"),
        );
    }

    let mut status_line = String::new();
    let delimiter = "|";
    loop {
        if let Some((vol, muted)) = volume() {
            if muted {
                status_line.push_str(&format!("^bg(000c11) 󰖁 {:.0}% ^bg()", vol));
            } else {
                status_line.push_str(&format!(
                    "^bg(000c11) {} {} {:.0}% ^fg()^bg()",
                    level_color(vol),
                    volume_icon(vol),
                    vol
                ));
            }
        }
        status_line.push_str(delimiter);

        if let (Some(rx), Some(tx)) = (netspeed_rx(NETWORK_INTERFACE), netspeed_tx(NETWORK_INTERFACE)) {
            status_line.push_str(&format!(
                "^bg(000c11)  {} /  {} ^bg()",
                fmt_human(rx as usize, 1024),
                fmt_human(tx as usize, 1024)
            ));
            status_line.push_str(delimiter);
        }

        let perc = cpu_prec();
        status_line.push_str(&format!(
            "{}^bg(000000) 󰻠  {:02.00}% ^fg()^bg()",
            level_color(perc),
            perc
        ));

        status_line.push_str(delimiter);

        if let (Some(perc), Some(used), Some(total)) = (ram_perc(), ram_used(), ram_total()) {
            status_line.push_str(&format!(
                "{}^bg(09301b) {:.01}% {} / {} ^fg()^bg()",
                level_color(perc),
                perc,
                fmt_human(used, 1024),
                fmt_human(total, 1024)
            ));
        }
        status_line.push_str(delimiter);

        status_line.push_str(&format!(
            "^bg(000000)^fg(009f00) 󰋊  {} / {} ^bg()^fg()",
            fmt_human(disk_used("/"), 1024),
            fmt_human(disk_total("/"), 1024)
        ));
        status_line.push_str(delimiter);

        status_line.push_str(&format!(
            "^fg(ffffff)^bg(ba0b0b)   {} ^bg()^fg()",
            kernel_release()
        ));
        status_line.push_str(delimiter);

        let date = datetime();
        status_line.push_str(&format!(
            "^bg(000000)^fg(46f2e6) ({}) {:02}-{:02}-{:04} {:02}:{:02}:{:02} ^bg()^fg()",
            date.weekday(),
            date.day(),
            date.month(),
            date.year(),
            date.hour(),
            date.minute(),
            date.second()
        ));
        status_line.push_str(delimiter);

        if let Some(mut out) = pipe_file.take() {
            out.write_fmt(format_args!("{}", status_line))
                .expect("Could not write into the FIFO file");
            out.flush().expect("Could not flush the status line");
            pipe_file.replace(out);
        } else {
            print!("\r{}", status_line);
        }

        status_line.clear();
        std::thread::sleep(Duration::from_millis(TIME_INTERVAL as u64));
    }
}
