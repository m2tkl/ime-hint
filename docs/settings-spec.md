# Settings Spec v2

## Behavior
- Settings apply immediately on change.
- Settings are persisted in `localStorage`.

## Controls
- Display mode: Badge / Bar (radio)
- Bar position: Top / Right / Left / Bottom (select; shown only for Bar)
- Badge width / height (shown only for Badge)
- On / Off / Unknown colors
- Background opacity (slider)

## UI Sections
- Display mode
- Mode-specific settings (Badge/Bar)
- Common settings

## Storage Shape (Current)
```json
{
  "displayMode": "badge",
  "barPosition": "top",
  "width": 260,
  "height": 120,
  "colorOn": "#ef4444",
  "colorOff": "#22c55e",
  "colorUnknown": "#6b7280",
  "bgOpacity": 0.2
}
```
