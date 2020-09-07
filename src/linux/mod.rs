use input::{
    AsRaw,
    ffi::{libinput_event_destroy, libinput_event_keyboard_get_base_event as get_base_keyboard},
    Libinput, LibinputInterface,
};

use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::close,
};
use input::{event::keyboard::{KeyboardEvent, KeyboardEventTrait}, Event::*};

use std::{
    os::unix::io::RawFd, path::Path, ptr::null,
    sync::atomic::AtomicPtr,
    thread::sleep, time::Duration,
};

use once_cell::sync::Lazy;

use x11::{xlib::*};

static SEND_DISPLAY: Lazy<AtomicPtr<Display>> = Lazy::new(|| {
    unsafe { XInitThreads() };
    AtomicPtr:: new(unsafe { XOpenDisplay(null()) })
});

struct LibinputInterfaceRaw;

impl LibinputInterfaceRaw {
    fn seat(&self) -> String {
        String::from("seat0")
    }
}

impl LibinputInterface for LibinputInterfaceRaw {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> std::result::Result<RawFd, i32> {
        if let Ok(fd) = open(path, OFlag::from_bits_truncate(flags), Mode::empty()) {
            Ok(fd)
        } else {
            Err(1)
        }
    }

    fn close_restricted(&mut self, fd: RawFd) {
        let _ = close(fd);
    }
}

pub fn create_tray() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn intercept_keybd_events() {
    let mut libinput_context = Libinput::new_with_udev(LibinputInterfaceRaw);
    libinput_context
        .udev_assign_seat(&LibinputInterfaceRaw.seat())
        .unwrap();

    loop {
        libinput_context.dispatch().unwrap();
        while let Some(event) = libinput_context.next() {
            match event {
                Keyboard(keyboard_event) => {
                    let KeyboardEvent::Key(keyboard_event_key) = keyboard_event;
                    let key = keyboard_event_key.key();
                    
                    println!("capslock {:?}", keyboard_event_key);
                    if key == 0x3a {
                        unsafe {
                            let raw_event = get_base_keyboard(keyboard_event_key.as_raw_mut());
                            libinput_event_destroy(raw_event);
                        }
                        continue;
                    }
                },
                _ => {},
            }
        }
    }

}

