import { invoke } from "@tauri-apps/api/core";

export type Profile = "Quiet" | "Balanced" | "Performance";
export type ProfileTarget = "ac" | "battery" | "both";
export type Brightness = "off" | "low" | "med" | "high";
export type Speed = "low" | "med" | "high";
export type Direction = "up" | "down" | "left" | "right";
export type Fan = "cpu" | "gpu" | "mid";
export type PowerZone = "keyboard" | "logo" | "lightbar" | "lid" | "rear-glow" | "ally";
export type AuraEffect =
  | "static"
  | "breathe"
  | "rainbow-cycle"
  | "rainbow-wave"
  | "stars"
  | "rain"
  | "highlight"
  | "laser"
  | "ripple"
  | "pulse"
  | "comet"
  | "flash";

export interface Supported {
  interfaces: string[];
  aura_modes: string[];
  fan_curves: boolean;
  slash: boolean;
  screenpad: boolean;
  armoury: boolean;
}
export interface Info {
  raw: string;
  supported_raw: string | null;
  supported: Supported;
}
export interface ThermalMetrics {
  cpu_temp_c: number | null;
  gpu_temp_c: number | null;
  cpu_fan_rpm: number | null;
  gpu_fan_rpm: number | null;
}
export interface ProfileList {
  profiles: string[];
  raw: string;
}
export interface ProfileGet {
  active: string | null;
  ac: string | null;
  battery: string | null;
}
export interface ProfileGetOut {
  profile: ProfileGet;
  raw: string;
}
export interface LedsOut {
  brightness: string | null;
  raw: string;
}
export interface BatteryOut {
  limit: number | null;
  raw: string;
}
export interface ArmouryItem {
  name: string;
  values: string[];
  current: string | null;
}
export interface ArmouryList {
  items: ArmouryItem[];
  raw: string;
}
export interface FanPoint {
  temp: number;
  pct: number;
}
export interface AuraEffectReq {
  effect?: AuraEffect;
  colour?: string;
  colour2?: string;
  speed?: Speed;
  direction?: Direction;
  zone?: string;
  switch?: "next" | "prev";
}
export interface ScreenpadReq {
  brightness?: number;
  gamma?: number;
  sync?: boolean;
}
export interface SlashReq {
  enable?: boolean;
  brightness?: number;
  interval?: number;
  mode?: string;
  list?: boolean;
  show_on_boot?: boolean;
  show_on_shutdown?: boolean;
  show_on_sleep?: boolean;
  show_on_battery?: boolean;
  show_battery_warning?: boolean;
}

const isTauri = () => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

const mockState = {
  active: "Performance" as Profile,
  ac: "Performance" as Profile,
  battery: "Quiet" as Profile,
  brightness: "high" as Brightness,
  limit: 80,
  armoury: [
    { name: "charge_mode", values: ["0", "1", "2"], current: "1" },
    { name: "dgpu_disable", values: ["0", "1"], current: "0" },
    { name: "panel_overdrive", values: ["0", "1"], current: "1" },
  ] as ArmouryItem[],
};

function call<T>(cmd: string, args?: Record<string, unknown>, fallback?: () => T): Promise<T> {
  if (import.meta.env.DEV && !isTauri() && fallback) {
    return Promise.resolve(fallback());
  }
  return invoke<T>(cmd, args);
}

export const getThermals = () =>
  call<ThermalMetrics>("get_thermals", undefined, () => ({
    cpu_temp_c: 54,
    gpu_temp_c: 48,
    cpu_fan_rpm: 2400,
    gpu_fan_rpm: 2100,
  }));

export const getInfo = () =>
  call<Info>("get_info", undefined, () => ({
    raw: "asusctl v6.3.8\n\nSoftware version: 6.3.8\n  Product family: ASUS TUF Gaming F15\n      Board name: FX506HF",
    supported_raw: "Supported Aura Modes: [Static, Breathe, RainbowCycle, RainbowWave, Pulse]",
    supported: {
      interfaces: [
        "xyz.ljones.AsusArmoury",
        "xyz.ljones.Aura",
        "xyz.ljones.Backlight",
        "xyz.ljones.Platform",
      ],
      aura_modes: ["Static", "Breathe", "RainbowCycle", "RainbowWave", "Pulse"],
      fan_curves: false,
      slash: false,
      screenpad: false,
      armoury: true,
    },
  }));

export const profileList = () =>
  call<ProfileList>("profile_list", undefined, () => ({
    profiles: ["Quiet", "Balanced", "Performance"],
    raw: "Quiet\nBalanced\nPerformance",
  }));

export const profileGet = () =>
  call<ProfileGetOut>("profile_get", undefined, () => ({
    profile: { active: mockState.active, ac: mockState.ac, battery: mockState.battery },
    raw: `Active profile: ${mockState.active}\n\nAC profile ${mockState.ac}\nBattery profile ${mockState.battery}`,
  }));

export const profileSet = (profile: Profile, target: ProfileTarget) =>
  call<string>("profile_set", { profile, target }, () => {
    if (target === "both") mockState.active = profile;
    if (target === "ac") mockState.ac = profile;
    if (target === "battery") mockState.battery = profile;
    return `Profile set to ${profile} (${target})`;
  });

export const ledsGet = () =>
  call<LedsOut>("leds_get", undefined, () => ({
    brightness: mockState.brightness,
    raw: `Current keyboard led brightness: ${mockState.brightness}`,
  }));

export const ledsSet = (brightness: Brightness) =>
  call<string>("leds_set", { brightness }, () => {
    mockState.brightness = brightness;
    return `Brightness set to ${brightness}`;
  });

export const ledsNext = () =>
  call<string>("leds_next", undefined, () => {
    const order: Brightness[] = ["off", "low", "med", "high"];
    mockState.brightness = order[(order.indexOf(mockState.brightness) + 1) % order.length];
    return `Brightness: ${mockState.brightness}`;
  });

export const ledsPrev = () =>
  call<string>("leds_prev", undefined, () => {
    const order: Brightness[] = ["off", "low", "med", "high"];
    mockState.brightness = order[(order.indexOf(mockState.brightness) + 3) % order.length];
    return `Brightness: ${mockState.brightness}`;
  });

export const auraEffect = (req: AuraEffectReq) =>
  call<string>(
    "aura_effect",
    { req },
    () => `Aura effect ${req.effect ?? req.switch ?? "updated"}`,
  );

export const auraPower = (
  zone: PowerZone,
  boot: boolean,
  awake: boolean,
  sleep: boolean,
  shutdown: boolean,
) =>
  call<string>(
    "aura_power",
    { zone, boot, awake, sleep, shutdown },
    () => `Power zone ${zone} updated`,
  );

export const batteryInfo = () =>
  call<BatteryOut>("battery_info", undefined, () => ({
    limit: mockState.limit,
    raw: `Current battery charge limit: ${mockState.limit}%`,
  }));

export const batterySetLimit = (percent: number) =>
  call<string>("battery_set_limit", { percent }, () => {
    mockState.limit = percent;
    return `Battery charge limit set to ${percent}%`;
  });

export const batteryOneShot = (percent?: number) =>
  call<string>("battery_one_shot", { percent: percent ?? null }, () => "One-shot charge started");

export const fanCurveGet = (profile?: Profile) =>
  call<string>(
    "fan_curve_get",
    { profile: profile ?? null },
    () => "unsupported: FanCurves not available",
  );

export const fanCurveSet = (profile: Profile, fan: Fan, points: FanPoint[]) =>
  call<string>("fan_curve_set", { profile, fan, points }, () => "Saved");

export const fanCurveEnable = (profile: Profile, fan: Fan | null, enable: boolean) =>
  call<string>("fan_curve_enable", { profile, fan, enable }, () => "Updated");

export const fanCurveReset = () => call<string>("fan_curve_reset", undefined, () => "Reset");

export const armouryList = () =>
  call<ArmouryList>("armoury_list", undefined, () => ({
    items: mockState.armoury.map((i) => ({ ...i })),
    raw: mockState.armoury
      .map(
        (i) =>
          `${i.name}:\n  current: [${i.values.map((v) => (v === i.current ? `(${v})` : v)).join(",")}]`,
      )
      .join("\n\n"),
  }));

export const armouryGet = (name: string) =>
  call<string>(
    "armoury_get",
    { name },
    () => mockState.armoury.find((i) => i.name === name)?.current ?? "",
  );

export const armourySet = (name: string, value: string) =>
  call<string>("armoury_set", { name, value }, () => {
    const item = mockState.armoury.find((i) => i.name === name);
    if (item) item.current = value;
    return `${name} = ${value}`;
  });

export const screenpad = (req: ScreenpadReq) =>
  call<string>("screenpad", { req }, () => "Screenpad updated");
export const slash = (req: SlashReq) => call<string>("slash", { req }, () => "Slash updated");

/** True when an error message indicates the feature is absent on this device. */
export function isUnsupported(err: unknown): boolean {
  const m = (err instanceof Error ? err.message : String(err)).toLowerCase();
  return /not supported|unsupported|not found|no such|doesn't exist|does not exist|unknown|no .*interface/.test(
    m,
  );
}

export const errMsg = (e: unknown) => {
  const m = e instanceof Error ? e.message : String(e);
  if (m.includes("reading 'invoke'") || m.includes("__TAURI_INTERNALS__")) {
    return "Tauri backend unavailable — launch the app with 'bun run tauri dev' to connect to asusctl.";
  }
  return m;
};
