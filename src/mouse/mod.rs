pub use self::platform::{click, send_button};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[cfg(target_os = "windows")]
#[path = "platform/windows.rs"]
mod platform;
