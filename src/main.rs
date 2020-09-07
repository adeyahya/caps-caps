#![windows_subsystem = "windows"]

#[cfg(target_os = "windows" )]
mod windows;

#[cfg(target_os = "windows" )]
use windows::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::*;

use std::{
    thread::spawn,
};

fn main() {
    spawn(|| {
        create_tray().unwrap();
    });
    intercept_keybd_events();
}

