use lazy_static::lazy_static;
use std::env;
use systray;

use once_cell::sync::Lazy;
use std::{
    process,
    mem::{size_of, transmute_copy, MaybeUninit},
    ptr::null_mut,
    sync::Mutex,
    thread::{spawn, sleep},
    time::Duration,
    sync::atomic::{AtomicPtr, Ordering},
};
use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::*},
    um::winuser::*,
};

lazy_static! {
    static ref LEADER_STATE: Mutex<bool> = Mutex::new(false);
}

static KEYBD_HHOOK: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

pub fn create_tray() -> Result<(), systray::Error> {
    // get directory that holds the executable
    let mut current_dir = env::current_exe().unwrap();
    current_dir.pop();
    let mut app;

    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    let icon_path = current_dir.join("caps-caps.ico");
    match app.set_icon_from_file(icon_path.to_str().unwrap()) {
        Ok(_) => {},
        Err(_) => {},    
    }

    app.add_menu_item("Quit", |window| {
        window.quit();
        exit_app();
        Ok::<_, systray::Error>(())
    })?;

    app.wait_for_message().unwrap();
    
    Ok(())
}

pub fn intercept_keybd_events() {
    set_hook(WH_KEYBOARD_LL, &*KEYBD_HHOOK, keybd_proc);
    let mut msg: MSG = unsafe { MaybeUninit::zeroed().assume_init() };
    unsafe { GetMessageW(&mut msg, 0 as HWND, 0, 0) };
}

fn set_hook(
    hook_id: i32,
    hook_ptr: &AtomicPtr<HHOOK__>,
    hook_proc: unsafe extern "system" fn(c_int, WPARAM, LPARAM) -> LRESULT,
) {
    hook_ptr.store(
        unsafe { SetWindowsHookExW(hook_id, Some(hook_proc), 0 as HINSTANCE, 0) },
        Ordering::Relaxed,
    );
}

unsafe extern "system" fn keybd_proc(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let mut l_state = LEADER_STATE.lock().unwrap();
    let key_code = (*(l_param as *const KBDLLHOOKSTRUCT)).vkCode;
    if w_param as u32 == WM_KEYDOWN {
        if key_code == 0x14 {
            *l_state = true;
            return 1;
        } else if *l_state == true {
            *l_state = false;
            spawn(move || {
                // 0x11 virtual key code for ctrl
                // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
                press(0x11);
                press(key_code as u16);
                sleep(Duration::from_millis(50));
                release(0x11);
                release(key_code as u16);
            });
            return 1;
        }
    }

    if w_param as u32 == WM_KEYUP {
        if key_code == 0x14 && *l_state == true {
            *l_state = false;
            spawn(move || {
                press(0x1B);
            });
            return 1;
        }
    }
    CallNextHookEx(null_mut(), code, w_param, l_param)
}

fn press(code: u16) {
    send_keybd_input(KEYEVENTF_SCANCODE, code);
}

fn release(code: u16) {
    send_keybd_input(KEYEVENTF_SCANCODE | KEYEVENTF_KEYUP, code);
}

fn send_keybd_input(flags: u32, code: u16) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe {
            transmute_copy(&KEYBDINPUT {
                wVk: code,
                wScan: MapVirtualKeyW(code.into(), 0) as u16,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            })
        },
    };
    unsafe { SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int) };
}

fn exit_app() {
    process::exit(0x00);
}

