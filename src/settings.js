const { invoke } = window.__TAURI__.core;
const { getCurrent } = window.__TAURI__.window;

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

function saveSettings(settings) {
  localStorage.setItem("ime-hint-settings", JSON.stringify(settings));
}

async function applySettings(settings) {
  await invoke("set_window_size", { width: settings.width, height: settings.height });
  await invoke("set_display_mode", {
    mode: settings.displayMode,
    barPosition: settings.barPosition,
  });
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

window.addEventListener("DOMContentLoaded", () => {
  const settings = loadSettings();

  const displayModeEls = document.querySelectorAll("input[name=\"display-mode\"]");
  const barPositionEl = document.querySelector("#setting-bar-position");
  const widthEl = document.querySelector("#setting-width");
  const heightEl = document.querySelector("#setting-height");
  const colorOnEl = document.querySelector("#setting-color-on");
  const colorOffEl = document.querySelector("#setting-color-off");
  const colorUnknownEl = document.querySelector("#setting-color-unknown");
  const bgOpacityEl = document.querySelector("#setting-bg-opacity");
  const resetEl = document.querySelector("#settings-reset");

  displayModeEls.forEach((el) => {
    el.checked = el.value === settings.displayMode;
  });
  barPositionEl.value = settings.barPosition;
  widthEl.value = settings.width;
  heightEl.value = settings.height;
  colorOnEl.value = normalizeHex(settings.colorOn, DEFAULT_SETTINGS.colorOn);
  colorOffEl.value = normalizeHex(settings.colorOff, DEFAULT_SETTINGS.colorOff);
  colorUnknownEl.value = normalizeHex(settings.colorUnknown, DEFAULT_SETTINGS.colorUnknown);
  bgOpacityEl.value = settings.bgOpacity ?? DEFAULT_SETTINGS.bgOpacity;

  applySettings(settings);

  resetEl.addEventListener("click", async () => {
    localStorage.removeItem("ime-hint-settings");
    const defaults = { ...DEFAULT_SETTINGS };
    displayModeEls.forEach((el) => {
      el.checked = el.value === defaults.displayMode;
    });
    barPositionEl.value = defaults.barPosition;
    widthEl.value = defaults.width;
    heightEl.value = defaults.height;
    colorOnEl.value = defaults.colorOn;
    colorOffEl.value = defaults.colorOff;
    colorUnknownEl.value = defaults.colorUnknown;
    bgOpacityEl.value = defaults.bgOpacity;
    await applySettings(defaults);
    await invoke("notify_settings", { settings: defaults });
  });

  const controls = [
    ...displayModeEls,
    barPositionEl,
    widthEl,
    heightEl,
    colorOnEl,
    colorOffEl,
    colorUnknownEl,
    bgOpacityEl,
  ];
  controls.forEach((el) => {
    el.addEventListener("input", () => {
      updateAndApply();
    });
    el.addEventListener("change", () => {
      updateAndApply();
    });
  });

  async function updateAndApply() {
    const mode = [...displayModeEls].find((el) => el.checked)?.value || "badge";
    updateFieldVisibility(mode);
    const updated = {
      displayMode: mode,
      barPosition: barPositionEl.value,
      width: Number(widthEl.value),
      height: Number(heightEl.value),
      colorOn: colorOnEl.value,
      colorOff: colorOffEl.value,
      colorUnknown: colorUnknownEl.value,
      bgOpacity: Number(bgOpacityEl.value),
    };
    saveSettings(updated);
    await applySettings(updated);
    await invoke("notify_settings", { settings: updated });
  }

  function updateFieldVisibility(mode) {
    document.querySelectorAll(".field-badge").forEach((el) => {
      el.classList.toggle("hidden", mode !== "badge");
    });
    document.querySelectorAll(".field-bar").forEach((el) => {
      el.classList.toggle("hidden", mode !== "bar");
    });
    const title = document.querySelector("#mode-section-title");
    if (title) {
      title.textContent = mode === "bar" ? "Bar settings" : "Badge settings";
    }
  }

  updateFieldVisibility(settings.displayMode);
});

function normalizeHex(value, fallback) {
  if (typeof value === "string" && value.startsWith("#")) return value;
  return fallback;
}
