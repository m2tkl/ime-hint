use std::{
    fs,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use tauri::{Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, WindowEvent};
use serde_json::Value;

mod ime;
mod hooks;

#[derive(Clone, serde::Serialize)]
pub(crate) struct BadgeEvent {
    visible: bool,
    state: ime::ImeState,
    reason: String,
}

struct MockState {
    running: Arc<AtomicBool>,
}

impl MockState {
    fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub(crate) fn emit_badge(app: &tauri::AppHandle, visible: bool, state: ime::ImeState, reason: &str) {
    let payload = BadgeEvent {
        visible,
        state,
        reason: reason.to_string(),
    };
    let _ = app.emit("ime-hint:badge", payload);

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let mode = DISPLAY_MODE.load(Ordering::SeqCst);
        if mode == 0 {
            let width = WINDOW_WIDTH.load(Ordering::SeqCst);
            let height = WINDOW_HEIGHT.load(Ordering::SeqCst);
            let _ = window.set_size(Size::Physical(PhysicalSize::<u32>::new(width, height)));
            if visible && !POSITION_SET.load(Ordering::SeqCst) {
                #[cfg(target_os = "macos")]
                hooks::move_window_to_fixed(app);
            }
        } else {
            #[cfg(target_os = "macos")]
            {
                let bar = BAR_POSITION.load(Ordering::SeqCst);
                let parsed = match bar {
                    1 => hooks::DisplayMode::BarLeft,
                    2 => hooks::DisplayMode::BarTop,
                    3 => hooks::DisplayMode::BarBottom,
                    _ => hooks::DisplayMode::BarRight,
                };
                hooks::apply_display_mode(app, parsed);
            }
        }
    }
}

fn start_macos_monitors(app: &tauri::AppHandle) {
    #[cfg(target_os = "macos")]
    {
        hooks::start_macos_input_monitor(app.clone());
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct WindowPos {
    x: i32,
    y: i32,
}

static POSITION_SET: AtomicBool = AtomicBool::new(false);
static WINDOW_WIDTH: AtomicU32 = AtomicU32::new(260);
static WINDOW_HEIGHT: AtomicU32 = AtomicU32::new(120);
static DISPLAY_MODE: AtomicU8 = AtomicU8::new(0);
static BAR_POSITION: AtomicU8 = AtomicU8::new(0);

fn position_path(app: &tauri::AppHandle) -> Option<PathBuf> {
    let base = app.path().app_config_dir().ok()?;
    Some(base.join("window-position.json"))
}

fn load_position(app: &tauri::AppHandle) -> Option<PhysicalPosition<i32>> {
    let path = position_path(app)?;
    let content = fs::read_to_string(path).ok()?;
    let pos: WindowPos = serde_json::from_str(&content).ok()?;
    Some(PhysicalPosition::new(pos.x, pos.y))
}

fn save_position(app: &tauri::AppHandle, position: PhysicalPosition<i32>) {
    let Some(path) = position_path(app) else { return };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let data = WindowPos {
        x: position.x,
        y: position.y,
    };
    if let Ok(content) = serde_json::to_string(&data) {
        let _ = fs::write(path, content);
    }
}

#[tauri::command]
fn get_ime_state(app: tauri::AppHandle) -> ime::ImeState {
    #[cfg(target_os = "macos")]
    {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        if app.run_on_main_thread(move || {
            let state = ime::current_state();
            let _ = tx.send(state);
        }).is_ok() {
            return rx.recv().unwrap_or(ime::ImeState::Unknown);
        }
        return ime::ImeState::Unknown;
    }

    #[cfg(not(target_os = "macos"))]
    {
        ime::current_state()
    }
}

#[tauri::command]
fn start_mock_events(app: tauri::AppHandle, state: tauri::State<MockState>) {
    if state.running.swap(true, Ordering::SeqCst) {
        return;
    }

    let running = state.running.clone();
    thread::spawn(move || {
        let mut is_on = true;
        while running.load(Ordering::SeqCst) {
            let ime_state = if is_on {
                ime::ImeState::On
            } else {
                ime::ImeState::Off
            };

            let _ = app.run_on_main_thread({
                let app = app.clone();
                move || {
                    emit_badge(&app, true, ime_state, "idle");
                }
            });
            thread::sleep(Duration::from_secs(2));
            let _ = app.run_on_main_thread({
                let app = app.clone();
                move || {
                    emit_badge(&app, false, ime_state, "input");
                }
            });
            thread::sleep(Duration::from_secs(5));

            is_on = !is_on;
        }
    });
}

#[tauri::command]
fn stop_mock_events(state: tauri::State<MockState>) {
    state.running.store(false, Ordering::SeqCst);
}

#[tauri::command]
fn start_window_dragging(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_focus();
        let _ = window.start_dragging();
    }
}

#[tauri::command]
fn focus_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_focus();
    }
}

#[tauri::command]
fn set_window_size(app: tauri::AppHandle, width: u32, height: u32) {
    WINDOW_WIDTH.store(width, Ordering::SeqCst);
    WINDOW_HEIGHT.store(height, Ordering::SeqCst);
    #[cfg(target_os = "macos")]
    hooks::set_window_size(width as i32, height as i32);
    #[cfg(not(target_os = "macos"))]
    let _ = (width, height);
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_size(Size::Physical(PhysicalSize::<u32>::new(width, height)));
    }
}

#[tauri::command]
fn set_display_mode(app: tauri::AppHandle, mode: String, bar_position: String) {
    #[cfg(target_os = "macos")]
    {
        DISPLAY_MODE.store(if mode == "bar" { 1 } else { 0 }, Ordering::SeqCst);
        let bar_code = match bar_position.as_str() {
            "left" => 1,
            "top" => 2,
            "bottom" => 3,
            _ => 0,
        };
        BAR_POSITION.store(bar_code, Ordering::SeqCst);
        let parsed = match (mode.as_str(), bar_position.as_str()) {
            ("bar", "left") => hooks::DisplayMode::BarLeft,
            ("bar", "right") => hooks::DisplayMode::BarRight,
            ("bar", "top") => hooks::DisplayMode::BarTop,
            ("bar", "bottom") => hooks::DisplayMode::BarBottom,
            _ => hooks::DisplayMode::Badge,
        };
        hooks::apply_display_mode(&app, parsed);
    }
    #[cfg(not(target_os = "macos"))]
    let _ = (mode, bar_position, app);
}

#[tauri::command]
fn notify_settings(app: tauri::AppHandle, settings: Value) {
    let _ = app.emit("settings-updated", settings);
}

#[tauri::command]
fn open_settings_window(app: tauri::AppHandle) {
    let label = "settings";
    if app.get_webview_window(label).is_some() {
        return;
    }
    let _ = tauri::WebviewWindowBuilder::new(
        &app,
        label,
        tauri::WebviewUrl::App("settings.html".into()),
    )
    .title("IME Hint Settings")
    .inner_size(520.0, 620.0)
    .resizable(false)
    .build();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(MockState::new())
        .invoke_handler(tauri::generate_handler![
            get_ime_state,
            start_mock_events,
            stop_mock_events,
            start_window_dragging,
            focus_window,
            set_window_size,
            set_display_mode,
            notify_settings,
            open_settings_window
        ])
        .setup(|app| {
            start_macos_monitors(app.handle());
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_always_on_top(true);
                let _ = window.show();
                if let Some(pos) = load_position(app.handle()) {
                    let _ = window.set_position(Position::Physical(pos));
                    POSITION_SET.store(true, Ordering::SeqCst);
                } else {
                    #[cfg(target_os = "macos")]
                    hooks::move_window_to_fixed(app.handle());
                }

                let app_handle = app.handle().clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::Moved(pos) = event {
                        save_position(&app_handle, *pos);
                        POSITION_SET.store(true, Ordering::SeqCst);
                    }
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
