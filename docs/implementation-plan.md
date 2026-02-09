# Implementation Plan v2

## Current State
- macOS working with TIS IME state detection.
- Low-frequency polling to refresh IME state.
- Always-on-top floating window with transparency.
- Settings window is separate and applies changes immediately.

## Architecture
- Rust (Tauri core) handles IME state and input detection.
- Frontend (HTML/CSS/JS) handles display and settings UI.
- Settings persisted in localStorage; updates pushed to main window.

## Main Components
- `src-tauri/src/ime/macos.rs`: IME state from TIS
- `src-tauri/src/hooks/macos.rs`: polling loop + window control
- `src-tauri/src/lib.rs`: window control + commands
- `src/settings.html` / `src/settings.js`: settings window

## Next Steps
- Windows implementation (IME + input hooks)
- Auto-launch support
- Optional: preset positions and export/import settings
