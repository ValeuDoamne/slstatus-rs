
//! # Net Speed
//! Get information about the current network speed

use std::{cell::Cell, fs::read_to_string};

use crate::config::TIME_INTERVAL;

thread_local! {
    static OLD_RX_BYTES: Cell<Option<usize>> = const { Cell::new(None) };
    static OLD_TX_BYTES: Cell<Option<usize>> = const { Cell::new(None) };
}

/// Get the current network download speed
pub fn netspeed_rx(interface: &str) -> Option<f64> {
    let rx_stats = read_to_string(format!("/sys/class/net/{}/statistics/rx_bytes", interface));
    let rx_stats = match rx_stats {
        Ok(rx) => rx,
        Err(_) => { return None; },
    };

    let rxbytes = rx_stats
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Could not parse RX Bytes!");

    if let Some(oldrxbytes) = OLD_RX_BYTES.take() {
        OLD_RX_BYTES.set(Some(rxbytes));
        Some(((rxbytes - oldrxbytes) as f64) * 1000.0 / TIME_INTERVAL as f64)
    } else {
        OLD_RX_BYTES.set(Some(rxbytes));
        Some(0.0)
    }
}

/// Get the current network upload speed
pub fn netspeed_tx(interface: &str) -> Option<f64> {
    let tx_stats = read_to_string(format!("/sys/class/net/{}/statistics/tx_bytes", interface));
    let tx_stats = match tx_stats {
        Ok(tx) => tx,
        Err(_) => { return None; },
    };

    let txbytes = tx_stats
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Could not parse RX Bytes!");

    if let Some(oldtxbytes) = OLD_TX_BYTES.take() {
        OLD_TX_BYTES.set(Some(txbytes));
        Some(((txbytes - oldtxbytes) as f64) * 1000.0 / TIME_INTERVAL as f64)
    } else {
        OLD_TX_BYTES.set(Some(txbytes));
        Some(0.0)
    }
}
