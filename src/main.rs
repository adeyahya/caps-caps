extern crate winapi;

use once_cell::sync::Lazy;
use std::{
    mem::{size_of, transmute_copy, MaybeUninit},
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
    thread::spawn,
};
use winapi::{
    ctypes::*,
    shared::{minwindef::*, windef::*},
    um::winuser::*,
};

static KEYBD_HHOOK: Lazy<AtomicPtr<HHOOK__>> = Lazy::new(AtomicPtr::default);

fn main() {
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
    if w_param as u32 == WM_KEYDOWN {
        // capslock code
        if (*(l_param as *const KBDLLHOOKSTRUCT)).vkCode == 0x14 {
            let mut input = INPUT {
                type_: INPUT_KEYBOARD,
                u: unsafe {
                    transmute_copy(&KEYBDINPUT {
                        wVk: 0x1B,
                        wScan: MapVirtualKeyW(u64::from(KEYEVENTF_SCANCODE) as u32, 0) as u16,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    })
                },
            };
            spawn(move || unsafe {
                SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int);
            });
            return 1;
        }
    }
    CallNextHookEx(null_mut(), code, w_param, l_param)
}
