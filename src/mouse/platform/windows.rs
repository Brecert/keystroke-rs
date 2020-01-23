extern crate winapi;

use self::winapi::um::winuser::{
    SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEINPUT,
};

use super::MouseButton;
use std::mem::{size_of, transmute_copy};

fn button_to_flag(button: MouseButton, up: bool) -> u32 {
    match button {
        MouseButton::Left => {
            if up {
                MOUSEEVENTF_LEFTUP
            } else {
                MOUSEEVENTF_LEFTDOWN
            }
        }
        MouseButton::Right => {
            if up {
                MOUSEEVENTF_RIGHTUP
            } else {
                MOUSEEVENTF_RIGHTDOWN
            }
        }
        MouseButton::Middle => {
            if up {
                MOUSEEVENTF_MIDDLEUP
            } else {
                MOUSEEVENTF_MIDDLEDOWN
            }
        }
    }
}

pub fn send_button(button: MouseButton, up: bool) {
    let input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe {
            transmute_copy(&MOUSEINPUT {
                dx: 0,
                dy: 0,
                dwFlags: button_to_flag(button, up),
                time: 0,
                dwExtraInfo: 0,
                mouseData: 0,
            })
        },
    };
    let mut inputs = vec![input];
    unsafe {
        SendInput(1, inputs.as_mut_ptr(), size_of::<INPUT>() as i32);
    }
}

#[inline]
pub fn click(button: MouseButton) {
    // todo: create more optimized syscall with vec of both
    send_button(button, false);
    send_button(button, true);
}
