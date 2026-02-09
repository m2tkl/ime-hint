const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
let badgeEl;
let overlayEl;
let barEl;
const INVERT_STATE = false;
const DEFAULT_SETTINGS = {
  displayMode: "badge",
  barPosition: "top",
  width: 260,
  height: 120,
  colorOn: "#ef4444",
  colorOff: "#22c55e",
  colorUnknown: "#6b7280",
  bgOpacity: 0.2,
};

function loadSettings() {
  try {
    const raw = localStorage.getItem("ime-hint-settings");
    if (!raw) return { ...DEFAULT_SETTINGS };
    return { ...DEFAULT_SETTINGS, ...JSON.parse(raw) };
  } catch {
    return { ...DEFAULT_SETTINGS };
  }
}

function applySettings(settings) {
  document.body.classList.remove("mode-badge", "mode-bar");
  document.body.classList.add(`mode-${settings.displayMode}`);

  barEl.classList.remove("top", "bottom", "left", "right");
  barEl.classList.add(settings.barPosition);

  const opacity = Number(settings.bgOpacity ?? DEFAULT_SETTINGS.bgOpacity);
  document.documentElement.style.setProperty(
    "--on-color",
    toRgba(settings.colorOn, opacity)
  );
  document.documentElement.style.setProperty(
    "--off-color",
    toRgba(settings.colorOff, opacity * 0.75)
  );
  document.documentElement.style.setProperty(
    "--unknown-color",
    toRgba(settings.colorUnknown, opacity * 0.9)
  );

  invoke("set_window_size", { width: settings.width, height: settings.height });
  invoke("set_display_mode", { mode: settings.displayMode, barPosition: settings.barPosition });
}

function toRgba(hex, alpha) {
  if (typeof hex !== "string") return `rgba(0,0,0,${alpha})`;
  if (hex.startsWith("rgba")) return hex;
  const normalized = hex.replace("#", "");
  const value =
    normalized.length === 3
      ? normalized
          .split("")
          .map((c) => c + c)
          .join("")
      : normalized;
  const r = parseInt(value.slice(0, 2), 16);
  const g = parseInt(value.slice(2, 4), 16);
  const b = parseInt(value.slice(4, 6), 16);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

function labelForState(state) {
  switch (state) {
    case "On":
      return "あ";
    case "Off":
      return "A";
    default:
      return "?";
  }
}

function applyBadge(state, visible, reason) {
  const displayState =
    INVERT_STATE && (state === "On" || state === "Off")
      ? state === "On"
        ? "Off"
        : "On"
      : state;
  const label = labelForState(displayState);
  badgeEl.textContent = label;
  badgeEl.classList.toggle("on", displayState === "On");
  badgeEl.classList.toggle("off", displayState === "Off");
  badgeEl.classList.toggle("unknown", displayState === "Unknown");

  barEl.classList.toggle("off", displayState === "Off");
  barEl.classList.toggle("unknown", displayState === "Unknown");

  document.body.classList.toggle("state-on", displayState === "On");
  document.body.classList.toggle("state-off", displayState === "Off");
  document.body.classList.toggle("state-unknown", displayState === "Unknown");
}

async function refreshImeState() {
  const state = await invoke("get_ime_state");
  applyBadge(state, false, "startup");
}

window.addEventListener("DOMContentLoaded", async () => {
  badgeEl = document.querySelector("#badge");
  barEl = document.querySelector("#bar");
  overlayEl = document.querySelector(".overlay");
  const settings = loadSettings();
  applySettings(settings);

  overlayEl.addEventListener("mousedown", async (event) => {
    if (event.button !== 0) return;
    await invoke("focus_window");
    await invoke("start_window_dragging");
  });

  overlayEl.addEventListener("contextmenu", async (event) => {
    event.preventDefault();
    await invoke("open_settings_window");
  });

  await refreshImeState();

  await listen("ime-hint:badge", (event) => {
    const { state, visible, reason } = event.payload;
    applyBadge(state, visible, reason);
  });

  await listen("settings-updated", (event) => {
    const settings = event.payload;
    applySettings(settings);
  });
});
