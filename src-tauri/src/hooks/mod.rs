#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::start_macos_input_monitor;

#[cfg(target_os = "macos")]
pub use macos::move_window_to_fixed;

#[cfg(target_os = "macos")]
pub use macos::{apply_display_mode, DisplayMode};

#[cfg(target_os = "macos")]
pub use macos::set_window_size;
