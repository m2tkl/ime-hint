use std::{sync::atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering}, thread, time::Duration};

use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, PhysicalPosition, Position};

use crate::ime;

type CGDirectDisplayID = u32;

#[repr(C)]
struct CGPoint {
    x: f64,
    y: f64,
}

#[repr(C)]
struct CGSize {
    width: f64,
    height: f64,
}

#[repr(C)]
struct CGRect {
    origin: CGPoint,
    size: CGSize,
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGMainDisplayID() -> CGDirectDisplayID;
    fn CGDisplayBounds(display: CGDirectDisplayID) -> CGRect;
}

static RUNNING: AtomicBool = AtomicBool::new(false);
static WINDOW_WIDTH: AtomicI32 = AtomicI32::new(260);
static WINDOW_HEIGHT: AtomicI32 = AtomicI32::new(120);
static LAST_STATE: AtomicU8 = AtomicU8::new(255);

const WINDOW_MARGIN: i32 = 24;
const BAR_THICKNESS: i32 = 10;

#[derive(Clone, Copy)]
pub enum DisplayMode {
    Badge,
    BarRight,
    BarLeft,
    BarTop,
    BarBottom,
}

pub fn move_window_to_fixed(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        unsafe {
            let bounds = CGDisplayBounds(CGMainDisplayID());
            let width = WINDOW_WIDTH.load(Ordering::SeqCst);
            let x = bounds.size.width as i32 - width - WINDOW_MARGIN;
            let y = WINDOW_MARGIN;
            let pos = PhysicalPosition::<i32>::new(x.max(0), y.max(0));
            let _ = window.set_position(Position::Physical(pos));
        }
    }
}

fn shrink_window_to_badge(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let width = WINDOW_WIDTH.load(Ordering::SeqCst);
        let height = WINDOW_HEIGHT.load(Ordering::SeqCst);
        let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize::<u32>::new(
            width as u32,
            height as u32,
        )));
    }
}

pub fn apply_display_mode(app: &AppHandle, mode: DisplayMode) {
    if let Some(window) = app.get_webview_window("main") {
        unsafe {
            let bounds = CGDisplayBounds(CGMainDisplayID());
            let screen_w = bounds.size.width;
            let screen_h = bounds.size.height;
            match mode {
                DisplayMode::Badge => {}
                DisplayMode::BarRight => {
                    let _ = window.set_size(tauri::Size::Logical(LogicalSize::new(
                        BAR_THICKNESS as f64,
                        screen_h,
                    )));
                    let _ = window.set_position(Position::Logical(LogicalPosition::new(
                        (screen_w - BAR_THICKNESS as f64).max(0.0),
                        0.0,
                    )));
                }
                DisplayMode::BarLeft => {
                    let _ = window.set_size(tauri::Size::Logical(LogicalSize::new(
                        BAR_THICKNESS as f64,
                        screen_h,
                    )));
                    let _ =
                        window.set_position(Position::Logical(LogicalPosition::new(0.0, 0.0)));
                }
                DisplayMode::BarTop => {
                    let _ = window.set_size(tauri::Size::Logical(LogicalSize::new(
                        screen_w,
                        BAR_THICKNESS as f64,
                    )));
                    let _ =
                        window.set_position(Position::Logical(LogicalPosition::new(0.0, 0.0)));
                }
                DisplayMode::BarBottom => {
                    let _ = window.set_size(tauri::Size::Logical(LogicalSize::new(
                        screen_w,
                        BAR_THICKNESS as f64,
                    )));
                    let _ = window.set_position(Position::Logical(LogicalPosition::new(
                        0.0,
                        (screen_h - BAR_THICKNESS as f64).max(0.0),
                    )));
                }
            }
        }
    }
}

pub fn set_window_size(width: i32, height: i32) {
    WINDOW_WIDTH.store(width, Ordering::SeqCst);
    WINDOW_HEIGHT.store(height, Ordering::SeqCst);
}

pub fn start_macos_input_monitor(app: AppHandle) {
    RUNNING.store(true, Ordering::SeqCst);
    LAST_STATE.store(255, Ordering::SeqCst);

    let app_for_poll = app.clone();
    thread::spawn(move || {
        while RUNNING.load(Ordering::SeqCst) {
            let app_inner = app_for_poll.clone();
            let _ = app_for_poll.run_on_main_thread(move || {
                let state = ime::current_state();
                let code = match state {
                    ime::ImeState::On => 1,
                    ime::ImeState::Off => 0,
                    ime::ImeState::Unknown => 2,
                };
                let prev = LAST_STATE.swap(code, Ordering::SeqCst);
                if prev != code {
                    shrink_window_to_badge(&app_inner);
                    crate::emit_badge(&app_inner, true, state, "ime");
                }
            });
            thread::sleep(Duration::from_millis(300));
        }
    });
}
