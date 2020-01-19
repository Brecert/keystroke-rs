//! Send a string, character, or keystroke event to the system.

pub use platform::{press_key, release_key};
pub use platform::{send_char, send_str};
pub use platform::{send_combo, send_key};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Physical {
    Return,
    Control,
    Alt,
    Shift,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Key {
    Physical(Physical),
    Unicode(char),
    Emulated(char),
    Auto(char)
}

#[cfg(target_os = "windows")]
#[path = "platform/windows.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "platform/linux.rs"]
mod platform;

#[cfg(test)]
mod tests {
    use super::{send_key, send_str, Key};

    #[test]
    fn test_lowercase_str() {
        send_str("echo 'test'\n");
    }

    #[test]
    fn test_emulated_char() {
        send_key(Key::Emulated('a'));
    }

    #[test]
    fn test_auto_char() {
        send_key(Key::Auto('@'));
    }
}
