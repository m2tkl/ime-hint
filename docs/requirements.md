# IME Hint Requirements v2

## Goal
Provide a lightweight floating indicator that shows IME On/Off state so users can avoid mistyping when resuming typing.

## Target OS
- macOS (current)
- Windows (planned)

## Core Behavior (Current Implementation)
- IME state is displayed as a floating indicator.
- Indicator is always visible.
- Indicator is always-on-top and draggable.
- Position is persisted.

## Display Modes
- Badge (default)
- Bar (top/left/right/bottom)

## Settings (Current)
- Display mode (Badge / Bar)
- Bar position (Top/Right/Left/Bottom)
- Badge width / height
- On / Off / Unknown colors
- Background opacity

## Non-Functional Requirements
- Low overhead
- Reasonable responsiveness without aggressive polling
- Non-intrusive, semi-transparent appearance

## Constraints
- Tauri-based application
- No Accessibility permission required in the current macOS build

## Out of Scope (Now)
- IME detailed modes (hiragana/roman, etc.)
- IME switching operations
- Taskbar/menu bar-only solution

## Risks / Limitations
- IME state detection depends on OS APIs; accuracy can vary by input source
- Global key events require Accessibility permission on macOS
