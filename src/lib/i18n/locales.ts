export const SUPPORTED_LOCALES = ["en", "tr", "de", "fr", "es", "ru", "zh", "ja"] as const;

export type Locale = (typeof SUPPORTED_LOCALES)[number];

export const DEFAULT_LOCALE: Locale = "en";

export const LOCALE_OPTIONS: ReadonlyArray<{ value: Locale; label: string }> = [
  { value: "en", label: "English" },
  { value: "tr", label: "Türkçe" },
  { value: "de", label: "Deutsch" },
  { value: "fr", label: "Français" },
  { value: "es", label: "Español" },
  { value: "ru", label: "Русский" },
  { value: "zh", label: "中文" },
  { value: "ja", label: "日本語" },
];

const STORAGE_KEY = "asusctl_locale";

export function isLocale(value: string | undefined | null): value is Locale {
  return typeof value === "string" && (SUPPORTED_LOCALES as readonly string[]).includes(value);
}

export function detectPreferredLocale(): Locale {
  if (typeof window === "undefined") return DEFAULT_LOCALE;
  try {
    const saved = window.localStorage.getItem(STORAGE_KEY);
    if (isLocale(saved)) return saved;
  } catch {
    // ignore storage access errors
  }

  const langs = window.navigator.languages?.length
    ? window.navigator.languages
    : [window.navigator.language];
  for (const lang of langs) {
    const code = lang?.split("-")[0]?.toLowerCase();
    if (isLocale(code)) return code;
  }
  return DEFAULT_LOCALE;
}

export function persistLocale(locale: Locale): void {
  if (typeof window === "undefined") return;
  try {
    window.localStorage.setItem(STORAGE_KEY, locale);
  } catch {
    // ignore storage access errors
  }
}

export function localizedPath(locale: Locale, pathname: string): string {
  const segments = pathname.split("/").filter(Boolean);
  if (segments.length > 0 && isLocale(segments[0])) {
    segments.shift();
  }
  return segments.length > 0 ? `/${locale}/${segments.join("/")}` : `/${locale}`;
}
