# Technical Notes v2

## macOS

### IME State
- Use TIS APIs to read current input source and infer ASCII-capable (Off) vs non-ASCII (On).

### Input Detection
- Not used in the current build (indicator is always visible and driven by IME state polling).

### Display
- Tauri window is transparent with semi-transparent UI.
- Window is always-on-top.
- Dragging is supported via Tauri `start_dragging` command.

### Settings
- Settings are stored in `localStorage` on the frontend.
- Settings window is a separate Webview window.

## Windows (Planned)

### IME State
- `ImmGetContext` + `ImmGetOpenStatus`.

### Input Detection
- `SetWindowsHookEx(WH_KEYBOARD_LL)`.

### Focus/Position
- `SetWinEventHook` and/or `GetGUIThreadInfo` for caret if needed (optional).

## Performance Strategy
- IME state is polled on a low-frequency loop (approx 300 ms) to keep CPU usage low.
