use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ImeState {
    On,
    Off,
    Unknown,
}

pub fn current_state() -> ImeState {
    #[cfg(target_os = "macos")]
    return macos::current_state();

    #[cfg(target_os = "windows")]
    return windows::current_state();

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    ImeState::Unknown
}

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;
