export type ThemeMode = "system" | "light" | "dark";

const STORAGE_KEY = "asusctl_theme";

export const theme = $state<{ mode: ThemeMode; resolved: "light" | "dark" }>({
  mode: "system",
  resolved: "dark",
});

function isThemeMode(value: string | null): value is ThemeMode {
  return value === "system" || value === "light" || value === "dark";
}

function applyTheme(mode: ThemeMode) {
  if (typeof window === "undefined") return;
  const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const isDark = mode === "dark" || (mode === "system" && prefersDark);
  theme.resolved = isDark ? "dark" : "light";
  document.documentElement.classList.toggle("dark", isDark);
}

export function setTheme(mode: ThemeMode) {
  theme.mode = mode;
  if (typeof window !== "undefined") {
    try {
      window.localStorage.setItem(STORAGE_KEY, mode);
    } catch {
      // ignore storage access errors
    }
  }
  applyTheme(mode);
}

export function initTheme(): () => void {
  if (typeof window === "undefined") return () => {};
  let saved: string | null = null;
  try {
    saved = window.localStorage.getItem(STORAGE_KEY);
  } catch {
    // ignore storage access errors
  }
  const initial: ThemeMode = isThemeMode(saved) ? saved : "system";
  theme.mode = initial;
  applyTheme(initial);

  const mq = window.matchMedia("(prefers-color-scheme: dark)");
  const onChange = () => {
    if (theme.mode === "system") applyTheme("system");
  };
  mq.addEventListener("change", onChange);
  return () => mq.removeEventListener("change", onChange);
}
