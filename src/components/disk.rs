#![allow(unused_assignments)]

//! # Disk
//! Get information about the hard disk

use std::{
    ffi::CString,
    mem::MaybeUninit,
};

/// Get the total memory of the hard disk
pub fn disk_total(path: &str) -> usize {
    let mut fs = MaybeUninit::uninit();
    let mut disk_size = 0usize;
    unsafe {
        let c_path = CString::new(path).expect("Could not initialize C string");
        let status = libc::statvfs(c_path.as_ptr(), fs.as_mut_ptr());
        if status < 0 {
            panic!("Error at calling libc::statvfs");
        }
        let initialize_fs = fs.assume_init();
        disk_size = (initialize_fs.f_frsize * initialize_fs.f_blocks) as usize
    }

    disk_size * 1024
}

// Get the disk total of disk used
pub fn disk_used(path: &str) -> usize {
    let mut fs = MaybeUninit::uninit();
    let mut disk_size = 0usize;
    unsafe {
        let c_path = CString::new(path).expect("Could not initialize C string");
        let status = libc::statvfs(c_path.as_ptr(), fs.as_mut_ptr());
        if status < 0 {
            panic!("Error at calling libc::statvfs");
        }
        let initialize_fs = fs.assume_init();
        disk_size =
            (initialize_fs.f_frsize * (initialize_fs.f_blocks - initialize_fs.f_bfree)) as usize
    }

    disk_size * 1024
}
