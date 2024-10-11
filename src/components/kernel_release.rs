#![allow(unused_assignments)]

//! # Kernel Release
//! Has only the function [`kernel_release`] which will return a [`String`] with the current
//! kernel release version

use std::mem::MaybeUninit;

/// Get the current kernel release version
pub fn kernel_release() -> String {
    let mut result = String::new();

    unsafe {
        let mut uts_name = MaybeUninit::uninit();
        let status = libc::uname(uts_name.as_mut_ptr());
        if status != 0 {
            panic!("Failed when calling uname");
        }

        result = String::from_utf8(
            uts_name
                .assume_init()
                .release
                .iter()
                .map(|x| *x as u8)
                .filter(|x| *x != 10 && *x != 0)
                .collect(),
        )
        .expect("Could not convert received kernel version!");
    }

    result
}
