//! # Volume
//! Using the ALSA API gets the volume of the Master card

use alsa::{
    mixer::{Selem, SelemChannelId, SelemId},
    Mixer,
};
use std::ffi::CStr;

/// Retrieve the volume level and if the volume was muted or not, `true` for mute and `false`
/// for unmuted
pub fn volume() -> Option<(f64, bool)> {
    // Create the object only at the first call and reuse it later on
    let mut master_mixer = Mixer::open(false).expect("Could not open mixer");

    master_mixer
        .attach(CStr::from_bytes_with_nul(b"default\0").expect("Could not convert to CString"))
        .expect("Could not attach to the default mixer");

    Selem::register(&mut master_mixer).expect("Could not register Selem");

    master_mixer.load().expect("Could not load mixer");

    if let Some(master_selem) = master_mixer.find_selem(&SelemId::new("Master", 0)) {
        let is_muted = master_selem
            .get_playback_switch(SelemChannelId::mono())
            .expect("Could not retrieve if it's muted!");
        let (pmin, pmax) = master_selem.get_playback_volume_range();
        let pvol = master_selem
            .get_playback_volume(SelemChannelId::mono())
            .expect("Could not retrieve volume from Master Mixer");
        let volume = ((pvol as f64) / ((pmax - pmin) as f64)) * 100.0;

        return Some((volume, is_muted == 0));
    }

    None
}

/// Get a volume icon based on the percentage of sound
pub fn volume_icon(percentage: f64) -> &'static str {
    if percentage <= 25.0 {
        "󰕿"
    } else if percentage <= 75.0 {
        return "󰖀";
    } else {
        return "󰕾";
    }
}
