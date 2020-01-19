extern crate winapi;

use self::winapi::shared::minwindef::WORD;
use self::winapi::um::winuser::{
    MapVirtualKeyA, SendInput, VkKeyScanA, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    KEYEVENTF_SCANCODE, KEYEVENTF_UNICODE, MAPVK_VK_TO_VSC,
};
use std::mem::{size_of, transmute_copy};

use super::{Key, Physical};

fn get_scancode(p: Physical) -> u16 {
    use Physical::*;
    match p {
        Return => 0x1c,
        Shift => 0x2a,
        Control => 0x1d,
        Alt => 0x38,
        A => 0x1e,
        B => 0x30,
        C => 0x2e,
        D => 0x20,
        E => 0x12,
        F => 0x21,
        G => 0x22,
        H => 0x23,
        I => 0x17,
        J => 0x24,
        K => 0x25,
        L => 0x26,
        M => 0x32,
        N => 0x31,
        O => 0x18,
        P => 0x19,
        Q => 0x10,
        R => 0x13,
        S => 0x1f,
        T => 0x14,
        U => 0x16,
        V => 0x2f,
        W => 0x11,
        X => 0x2d,
        Y => 0x15,
        Z => 0x2c,
    }
}

// fn unicode_to_inputs(c: char) -> Vec<INPUT> {
//     let mut wide = [0; 2];
//     c.encode_utf16(&mut wide);
//     let mut inputs: Vec<INPUT> = vec![];
//     for i in 0..wide.len()-1 {
//         dbg!(i);
//         dbg!(wide[i]);
//         let input = INPUT {
//             type_: INPUT_KEYBOARD,
//             u: unsafe { transmute_copy(&KEYBDINPUT {
//                 wVk: 0,
//                 wScan: wide[i],
//                 time: 0,
//                 dwFlags: KEYEVENTF_UNICODE,
//                 dwExtraInfo: zeroed()
//             }) }
//         };
//         inputs.push(input);
//     }
//     inputs
// }

fn key_to_lpinput(key: &Key, up: bool) -> INPUT {
    let upflag = if up { KEYEVENTF_KEYUP } else { 0 };

    match *key {
        Key::Physical(p) => {
            INPUT {
                type_: INPUT_KEYBOARD,
                u: unsafe {
                    transmute_copy(&KEYBDINPUT {
                        wVk: 0,
                        wScan: get_scancode(p), // hardware scan code
                        dwFlags: KEYEVENTF_SCANCODE | upflag,
                        time: 0,
                        dwExtraInfo: 0,
                    })
                },
            }
        }
        Key::Unicode(c) => {
            INPUT {
                type_: INPUT_KEYBOARD,
                u: unsafe {
                    transmute_copy(&KEYBDINPUT {
                        wVk: 0,
                        wScan: c as WORD, // a unicode code
                        dwFlags: KEYEVENTF_UNICODE | upflag,
                        time: 0,
                        dwExtraInfo: 0,
                    })
                },
            }
        }
        Key::Emulated(c) => {
            // emulated is slower but more "accurate".
            let long = unsafe { VkKeyScanA(c as i8) };
            let vkey = unsafe { MapVirtualKeyA(long as u32, MAPVK_VK_TO_VSC) };
            INPUT {
                type_: INPUT_KEYBOARD,
                u: unsafe {
                    transmute_copy(&KEYBDINPUT {
                        wVk: long as u16,
                        wScan: vkey as u16,
                        dwFlags: upflag,
                        time: 0,
                        dwExtraInfo: 0,
                    })
                },
            }
        }
    }
}

fn send_input(keys: &[Key], up: bool) {
    //convert all the keys to windows events
    let mut inputs: Vec<INPUT> = keys.iter().map(|k| key_to_lpinput(k, up)).collect();
    unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_mut_ptr(),
            size_of::<INPUT>() as i32,
        );
    }
}

#[inline]
pub fn press_key(k: Key) {
    send_input(&[k], false);
}

#[inline]
pub fn release_key(k: Key) {
    send_input(&[k], true);
}

#[inline]
pub fn send_combo(keys: &[Key]) {
    send_input(keys, false);
    send_input(keys, true);
}

#[inline]
pub fn send_key(k: Key) {
    press_key(k);
    release_key(k);
}

/// Send all unicode characters below 0x10000, silently skipping others.
#[inline]
pub fn send_char(c: char) {
    if (c as u64) < 0x10000 {
        send_key(Key::Unicode(c));
    } else {
        panic!("char {:?} is two points", c as u64);
    }
}

/// Send a string as keyboard events. Unsupported chars are silently ignored.
#[inline]
pub fn send_str(msg: &str) {
    for c in msg.chars() {
        send_char(c);
    }
}
