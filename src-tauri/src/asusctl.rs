//! asusctl wrapper: pure arg builders + parsers (testable without a daemon) and Tauri commands.
//! Never uses a shell; every argument is built from validated enums/ranges/tokens.

use serde::{Deserialize, Serialize};
use std::process::Command;

// ---------------------------------------------------------------- enums

macro_rules! str_enum {
    ($name:ident { $($var:ident => $s:expr),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
        pub enum $name { $( #[serde(rename = $s)] $var ),+ }
        impl $name {
            pub fn as_str(&self) -> &'static str { match self { $( Self::$var => $s ),+ } }
        }
    };
}

str_enum!(Profile { Quiet => "Quiet", Balanced => "Balanced", Performance => "Performance" });
str_enum!(Brightness { Off => "off", Low => "low", Med => "med", High => "high" });
str_enum!(Speed { Low => "low", Med => "med", High => "high" });
str_enum!(Direction { Up => "up", Down => "down", Left => "left", Right => "right" });
str_enum!(Fan { Cpu => "cpu", Gpu => "gpu", Mid => "mid" });
str_enum!(PowerZone {
    Keyboard => "keyboard", Logo => "logo", Lightbar => "lightbar",
    Lid => "lid", RearGlow => "rear-glow", Ally => "ally"
});
str_enum!(AuraEffect {
    Static => "static", Breathe => "breathe", RainbowCycle => "rainbow-cycle",
    RainbowWave => "rainbow-wave", Stars => "stars", Rain => "rain",
    Highlight => "highlight", Laser => "laser", Ripple => "ripple",
    Pulse => "pulse", Comet => "comet", Flash => "flash"
});
str_enum!(ProfileTarget { Ac => "ac", Battery => "battery", Both => "both" });

// ---------------------------------------------------------------- validators

fn is_token(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
}

/// armoury attribute name: ^[a-z0-9_]+$
pub fn valid_armoury_name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// 6 hex digits, optional leading '#', returned lowercase without '#'.
pub fn norm_colour(s: &str) -> Result<String, String> {
    let h = s.strip_prefix('#').unwrap_or(s);
    if h.len() == 6 && h.chars().all(|c| c.is_ascii_hexdigit()) {
        Ok(h.to_ascii_lowercase())
    } else {
        Err(format!("invalid colour '{s}', expected 6 hex digits"))
    }
}

/// zone: 0-9 or a simple token (e.g. one, logo, lightbar-left)
fn check_zone(z: &str) -> Result<(), String> {
    if is_token(z) {
        Ok(())
    } else {
        Err(format!("invalid zone '{z}'"))
    }
}

fn v(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|s| s.to_string()).collect()
}

// ---------------------------------------------------------------- arg builders

pub fn args_profile_set(profile: Profile, target: ProfileTarget) -> Vec<Vec<String>> {
    let mk = |flag: Option<&str>| {
        let mut a = v(&["profile", "set"]);
        if let Some(f) = flag {
            a.push(f.into());
        }
        a.push("--".into());
        a.push(profile.as_str().into());
        a
    };
    match target {
        ProfileTarget::Ac => vec![mk(Some("--ac"))],
        ProfileTarget::Battery => vec![mk(Some("--battery"))],
        ProfileTarget::Both => vec![mk(Some("--ac")), mk(Some("--battery"))],
    }
}

pub fn args_leds_set(b: Brightness) -> Vec<String> {
    v(&["leds", "set", b.as_str()])
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AuraEffectReq {
    pub effect: Option<AuraEffect>,
    pub colour: Option<String>,
    pub colour2: Option<String>,
    pub speed: Option<Speed>,
    pub direction: Option<Direction>,
    pub zone: Option<String>,
    /// "next" / "prev" mode switch instead of an effect
    pub switch: Option<String>,
}

pub fn args_aura_effect(r: &AuraEffectReq) -> Result<Vec<String>, String> {
    let mut a = v(&["aura", "effect"]);
    if let Some(sw) = &r.switch {
        match sw.as_str() {
            "next" => a.push("--next-mode".into()),
            "prev" => a.push("--prev-mode".into()),
            _ => return Err(format!("invalid switch '{sw}'")),
        }
        return Ok(a);
    }
    let e = r.effect.ok_or("effect required")?;
    a.push(e.as_str().into());
    use AuraEffect::*;
    let (c1, c2, spd, dir) = match e {
        Static | Pulse | Comet | Flash => (true, false, false, false),
        Breathe | Stars => (true, true, true, false),
        RainbowCycle | Rain => (false, false, true, false),
        RainbowWave => (false, false, true, true),
        Highlight | Laser | Ripple => (true, false, true, false),
    };
    if c1 {
        a.push("--colour".into());
        a.push(norm_colour(r.colour.as_deref().ok_or("colour required")?)?);
    }
    if c2 {
        a.push("--colour2".into());
        a.push(norm_colour(
            r.colour2.as_deref().ok_or("colour2 required")?,
        )?);
    }
    if spd {
        a.push("--speed".into());
        a.push(r.speed.ok_or("speed required")?.as_str().into());
    }
    if dir {
        a.push("--direction".into());
        a.push(r.direction.ok_or("direction required")?.as_str().into());
    }
    if let Some(z) = &r.zone {
        check_zone(z)?;
        a.push("--zone".into());
        a.push(z.clone());
    }
    Ok(a)
}

pub fn args_aura_power(
    zone: PowerZone,
    boot: bool,
    awake: bool,
    sleep: bool,
    shutdown: bool,
) -> Vec<String> {
    let mut a = v(&["aura", "power", zone.as_str()]);
    for (on, f) in [
        (boot, "--boot"),
        (awake, "--awake"),
        (sleep, "--sleep"),
        (shutdown, "--shutdown"),
    ] {
        if on {
            a.push(f.into());
        }
    }
    a
}

pub fn args_battery_limit(p: u8) -> Result<Vec<String>, String> {
    if !(20..=100).contains(&p) {
        return Err(format!("battery limit {p} out of range 20-100"));
    }
    Ok(vec!["battery".into(), "limit".into(), p.to_string()])
}

pub fn args_battery_oneshot(p: Option<u8>) -> Result<Vec<String>, String> {
    let mut a = v(&["battery", "oneshot"]);
    if let Some(p) = p {
        if !(20..=100).contains(&p) {
            return Err(format!("percent {p} out of range 20-100"));
        }
        a.push("--".into());
        a.push(p.to_string());
    }
    Ok(a)
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct FanPoint {
    pub temp: u8,
    pub pct: u8,
}

/// "30c:1%,49c:2%,..."
pub fn fan_data_string(points: &[FanPoint]) -> Result<String, String> {
    if points.is_empty() || points.len() > 8 {
        return Err("fan curve needs 1-8 points".into());
    }
    let mut last = 0u8;
    let mut out = Vec::new();
    for p in points {
        if !(20..=110).contains(&p.temp) {
            return Err(format!("temperature {} out of range 20-110", p.temp));
        }
        if p.pct > 100 {
            return Err(format!("fan percent {} out of range 0-100", p.pct));
        }
        if p.temp < last {
            return Err("temperatures must be non-decreasing".into());
        }
        last = p.temp;
        out.push(format!("{}c:{}%", p.temp, p.pct));
    }
    Ok(out.join(","))
}

pub fn args_fan_get(profile: Option<Profile>) -> Vec<String> {
    match profile {
        Some(p) => v(&["fan-curve", "--mod-profile", p.as_str()]),
        None => v(&["fan-curve", "--get-enabled"]),
    }
}

pub fn args_fan_set(
    profile: Profile,
    fan: Fan,
    points: &[FanPoint],
) -> Result<Vec<String>, String> {
    let data = fan_data_string(points)?;
    Ok(vec![
        "fan-curve".into(),
        "--mod-profile".into(),
        profile.as_str().into(),
        "--fan".into(),
        fan.as_str().into(),
        "--data".into(),
        data,
    ])
}

pub fn args_fan_enable(profile: Profile, fan: Option<Fan>, enable: bool) -> Vec<String> {
    let b = if enable { "true" } else { "false" };
    let mut a = v(&["fan-curve", "--mod-profile", profile.as_str()]);
    match fan {
        Some(f) => a.extend(v(&["--fan", f.as_str(), "--enable-fan-curve", b])),
        None => a.extend(v(&["--enable-fan-curves", b])),
    }
    a
}

pub fn args_fan_reset() -> Vec<String> {
    v(&["fan-curve", "--default"])
}

pub fn args_armoury_get(name: &str) -> Result<Vec<String>, String> {
    if !valid_armoury_name(name) {
        return Err(format!("invalid attribute name '{name}'"));
    }
    Ok(v(&["armoury", "get", name]))
}

pub fn args_armoury_set(name: &str, value: &str) -> Result<Vec<String>, String> {
    if !valid_armoury_name(name) {
        return Err(format!("invalid attribute name '{name}'"));
    }
    if !is_token(value) {
        return Err(format!("invalid value '{value}'"));
    }
    Ok(v(&["armoury", "set", "--", name, value]))
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ScreenpadReq {
    pub brightness: Option<u8>,
    pub gamma: Option<f32>,
    pub sync: Option<bool>,
}

pub fn args_screenpad(r: &ScreenpadReq) -> Result<Vec<String>, String> {
    let mut a = v(&["backlight"]);
    if let Some(b) = r.brightness {
        if b > 100 {
            return Err("brightness out of range 0-100".into());
        }
        a.extend(v(&["--screenpad-brightness", &b.to_string()]));
    }
    if let Some(g) = r.gamma {
        if !(0.5..=2.2).contains(&g) {
            return Err("gamma out of range 0.5-2.2".into());
        }
        a.extend(v(&["--screenpad-gamma", &format!("{g:.2}")]));
    }
    if let Some(s) = r.sync {
        a.extend(v(&[
            "--sync-screenpad-brightness",
            if s { "true" } else { "false" },
        ]));
    }
    Ok(a)
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SlashReq {
    pub enable: Option<bool>,
    pub brightness: Option<u8>,
    pub interval: Option<u8>,
    pub mode: Option<String>,
    pub list: Option<bool>,
    pub show_on_boot: Option<bool>,
    pub show_on_shutdown: Option<bool>,
    pub show_on_sleep: Option<bool>,
    pub show_on_battery: Option<bool>,
    pub show_battery_warning: Option<bool>,
}

pub fn args_slash(r: &SlashReq) -> Result<Vec<String>, String> {
    let mut a = v(&["slash"]);
    match r.enable {
        Some(true) => a.push("--enable".into()),
        Some(false) => a.push("--disable".into()),
        None => {}
    }
    if r.list == Some(true) {
        a.push("--list".into());
    }
    if let Some(b) = r.brightness {
        a.extend(v(&["--brightness", &b.to_string()]));
    }
    if let Some(i) = r.interval {
        if i > 5 {
            return Err("interval out of range 0-5".into());
        }
        a.extend(v(&["--interval", &i.to_string()]));
    }
    if let Some(m) = &r.mode {
        if !is_token(m) {
            return Err(format!("invalid mode '{m}'"));
        }
        a.extend(v(&["--mode", m]));
    }
    for (val, flag) in [
        (r.show_on_boot, "--show-on-boot"),
        (r.show_on_shutdown, "--show-on-shutdown"),
        (r.show_on_sleep, "--show-on-sleep"),
        (r.show_on_battery, "--show-on-battery"),
        (r.show_battery_warning, "--show-battery-warning"),
    ] {
        if let Some(b) = val {
            a.extend(v(&[flag, if b { "true" } else { "false" }]));
        }
    }
    Ok(a)
}

// ---------------------------------------------------------------- parsers

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct ProfileGet {
    pub active: Option<String>,
    pub ac: Option<String>,
    pub battery: Option<String>,
}

pub fn parse_profile_list(out: &str) -> Vec<String> {
    out.lines()
        .map(|l| l.trim().trim_start_matches(['*', '-']).trim())
        .filter(|l| {
            !l.is_empty()
                && l.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        })
        .map(String::from)
        .collect()
}

pub fn parse_profile_get(out: &str) -> ProfileGet {
    let mut r = ProfileGet::default();
    for l in out.lines() {
        let Some((k, val)) = l.split_once(':') else {
            continue;
        };
        let (k, val) = (k.to_ascii_lowercase(), val.trim().to_string());
        if val.is_empty() {
            continue;
        }
        if k.contains("battery") {
            r.battery = Some(val);
        } else if k.contains("ac") && !k.contains("active") {
            r.ac = Some(val);
        } else if k.contains("active") {
            r.active = Some(val);
        }
    }
    r
}

/// "Current keyboard led brightness: Med" -> "med"
pub fn parse_leds_get(out: &str) -> Option<String> {
    for l in out.lines() {
        if let Some((k, val)) = l.rsplit_once(':')
            && k.to_ascii_lowercase().contains("bright")
        {
            return Some(val.trim().to_ascii_lowercase());
        }
    }
    None
}

/// First "NN%" or number following "limit" in the text.
pub fn parse_battery_info(out: &str) -> Option<u8> {
    for l in out.lines() {
        if !l.to_ascii_lowercase().contains("limit") {
            continue;
        }
        let digits: String = l
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if let Ok(n) = digits.parse::<u8>() {
            return Some(n);
        }
    }
    None
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct ArmouryItem {
    pub name: String,
    pub values: Vec<String>,
    pub current: Option<String>,
}

/// Lines like `name: [a, (b), c]` or `name: a (b) c`; parentheses mark the current value.
pub fn parse_armoury_list(out: &str) -> Vec<ArmouryItem> {
    let mut items = Vec::new();
    for l in out.lines() {
        let l = l.trim();
        if l.contains("Multiple asusd interfaces") {
            continue;
        }
        let Some((name, rest)) = l.split_once(':') else {
            continue;
        };
        let name = name.trim();
        if !valid_armoury_name(name) {
            continue;
        }
        let mut it = ArmouryItem {
            name: name.into(),
            ..Default::default()
        };
        for tok in rest.split(|c: char| c.is_whitespace() || c == ',' || c == '[' || c == ']') {
            if tok.is_empty() {
                continue;
            }
            if tok.starts_with('(') && tok.ends_with(')') && tok.len() > 2 {
                let t = tok[1..tok.len() - 1].to_string();
                it.current = Some(t.clone());
                it.values.push(t);
            } else {
                it.values.push(tok.to_string());
            }
        }
        items.push(it);
    }
    items
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct Supported {
    pub interfaces: Vec<String>,
    pub aura_modes: Vec<String>,
    pub fan_curves: bool,
    pub slash: bool,
    pub screenpad: bool,
    pub armoury: bool,
}

fn bracket_list(s: &str) -> Vec<String> {
    let (Some(a), Some(b)) = (s.find('['), s.rfind(']')) else {
        return vec![];
    };
    if b <= a {
        return vec![];
    }
    s[a + 1..b]
        .split(',')
        .map(|x| x.trim().trim_matches('"').to_string())
        .filter(|x| !x.is_empty())
        .collect()
}

pub fn parse_supported(out: &str) -> Supported {
    let mut s = Supported::default();
    for l in out.lines() {
        let low = l.to_ascii_lowercase();
        if low.contains("interface") {
            s.interfaces.extend(bracket_list(l));
        } else if low.contains("aura") && low.contains("mode") || low.contains("basic modes") {
            s.aura_modes.extend(bracket_list(l));
        }
    }
    let has = |n: &str| {
        s.interfaces
            .iter()
            .any(|i| i.to_ascii_lowercase().contains(n))
    };
    s.fan_curves = has("fan") || has("platform");
    s.slash = has("slash");
    s.screenpad = has("screenpad") || has("backlight");
    s.armoury = has("armoury");
    s
}

// ---------------------------------------------------------------- runner

const ERR_MARKERS: [&str; 6] = [
    "error",
    "failed",
    "panicked",
    "unrecognized",
    "invalid",
    "not found",
];

/// Classify combined output. Err("unsupported: ...") for missing D-Bus interfaces.
pub fn classify(success: bool, out: &str) -> Result<String, String> {
    let low = out.to_ascii_lowercase();
    if low.contains("unknowninterface")
        || low.contains("unknown interface")
        || low.contains("unknownmethod")
    {
        return Err(format!("unsupported: {}", out.trim()));
    }
    let err_line = out.lines().any(|l| {
        let l = l.trim().to_ascii_lowercase();
        ERR_MARKERS.iter().any(|m| l.starts_with(m))
    });
    if !success || err_line {
        let t = out.trim();
        return Err(if t.is_empty() {
            "asusctl failed".into()
        } else {
            t.into()
        });
    }
    Ok(out.to_string())
}

pub fn run_blocking(args: &[String]) -> Result<String, String> {
    let o = Command::new("asusctl")
        .args(args)
        .current_dir("/tmp")
        .output()
        .map_err(|e| format!("cannot run asusctl: {e}"))?;
    let mut text = String::from_utf8_lossy(&o.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&o.stderr));
    classify(o.status.success(), &text)
}

async fn run(args: Vec<String>) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || run_blocking(&args))
        .await
        .map_err(|e| e.to_string())?
}

// ---------------------------------------------------------------- commands

#[derive(Serialize)]
pub struct ThermalMetrics {
    pub cpu_temp_c: Option<f32>,
    pub gpu_temp_c: Option<f32>,
    pub cpu_fan_rpm: Option<u32>,
    pub gpu_fan_rpm: Option<u32>,
}

fn read_hwmon_thermals() -> ThermalMetrics {
    let mut cpu_temp_c = None;
    let mut gpu_temp_c = None;
    let mut cpu_fan_rpm = None;
    let mut gpu_fan_rpm = None;

    let Ok(entries) = std::fs::read_dir("/sys/class/hwmon") else {
        return ThermalMetrics {
            cpu_temp_c,
            gpu_temp_c,
            cpu_fan_rpm,
            gpu_fan_rpm,
        };
    };

    for entry in entries.flatten() {
        let dir = entry.path();
        let name = std::fs::read_to_string(dir.join("name"))
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase();

        let is_cpu = name.contains("coretemp")
            || name.contains("k10temp")
            || name.contains("zenpower")
            || name.contains("acpitz");
        let is_gpu = name.contains("amdgpu") || name.contains("nouveau") || name.contains("nvidia");

        if is_cpu
            && cpu_temp_c.is_none()
            && let Ok(raw) = std::fs::read_to_string(dir.join("temp1_input"))
            && let Ok(milli) = raw.trim().parse::<f32>()
        {
            cpu_temp_c = Some((milli / 1000.0).round());
        }
        if is_gpu
            && gpu_temp_c.is_none()
            && let Ok(raw) = std::fs::read_to_string(dir.join("temp1_input"))
            && let Ok(milli) = raw.trim().parse::<f32>()
        {
            gpu_temp_c = Some((milli / 1000.0).round());
        }
        if name.contains("asus") {
            if cpu_fan_rpm.is_none()
                && let Ok(raw) = std::fs::read_to_string(dir.join("fan1_input"))
            {
                cpu_fan_rpm = raw.trim().parse::<u32>().ok();
            }
            if gpu_fan_rpm.is_none()
                && let Ok(raw) = std::fs::read_to_string(dir.join("fan2_input"))
            {
                gpu_fan_rpm = raw.trim().parse::<u32>().ok();
            }
        }
    }

    ThermalMetrics {
        cpu_temp_c,
        gpu_temp_c,
        cpu_fan_rpm,
        gpu_fan_rpm,
    }
}

#[tauri::command]
pub async fn get_thermals() -> Result<ThermalMetrics, String> {
    tauri::async_runtime::spawn_blocking(read_hwmon_thermals)
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct Info {
    pub raw: String,
    pub supported_raw: Option<String>,
    pub supported: Supported,
}

#[tauri::command]
pub async fn get_info() -> Result<Info, String> {
    let raw = run(v(&["info"])).await?;
    let supported_raw = run(v(&["info", "--show-supported"])).await.ok();
    let supported = supported_raw
        .as_deref()
        .map(parse_supported)
        .unwrap_or_default();
    Ok(Info {
        raw,
        supported_raw,
        supported,
    })
}

#[derive(Serialize)]
pub struct ProfileList {
    pub profiles: Vec<String>,
    pub raw: String,
}

#[tauri::command]
pub async fn profile_list() -> Result<ProfileList, String> {
    let raw = run(v(&["profile", "list"])).await?;
    Ok(ProfileList {
        profiles: parse_profile_list(&raw),
        raw,
    })
}

#[derive(Serialize)]
pub struct ProfileGetOut {
    pub profile: ProfileGet,
    pub raw: String,
}

#[tauri::command]
pub async fn profile_get() -> Result<ProfileGetOut, String> {
    let raw = run(v(&["profile", "get"])).await?;
    Ok(ProfileGetOut {
        profile: parse_profile_get(&raw),
        raw,
    })
}

#[tauri::command]
pub async fn profile_set(profile: Profile, target: ProfileTarget) -> Result<String, String> {
    let mut out = String::new();
    for a in args_profile_set(profile, target) {
        out.push_str(&run(a).await?);
    }
    Ok(out)
}

#[derive(Serialize)]
pub struct LedsOut {
    pub brightness: Option<String>,
    pub raw: String,
}

#[tauri::command]
pub async fn leds_get() -> Result<LedsOut, String> {
    let raw = run(v(&["leds", "get"])).await?;
    Ok(LedsOut {
        brightness: parse_leds_get(&raw),
        raw,
    })
}

#[tauri::command]
pub async fn leds_set(brightness: Brightness) -> Result<String, String> {
    run(args_leds_set(brightness)).await
}

#[tauri::command]
pub async fn leds_next() -> Result<String, String> {
    run(v(&["leds", "next"])).await
}

#[tauri::command]
pub async fn leds_prev() -> Result<String, String> {
    run(v(&["leds", "prev"])).await
}

#[tauri::command]
pub async fn aura_effect(req: AuraEffectReq) -> Result<String, String> {
    run(args_aura_effect(&req)?).await
}

#[tauri::command]
pub async fn aura_power(
    zone: PowerZone,
    boot: bool,
    awake: bool,
    sleep: bool,
    shutdown: bool,
) -> Result<String, String> {
    run(args_aura_power(zone, boot, awake, sleep, shutdown)).await
}

#[derive(Serialize)]
pub struct BatteryOut {
    pub limit: Option<u8>,
    pub raw: String,
}

#[tauri::command]
pub async fn battery_info() -> Result<BatteryOut, String> {
    let raw = run(v(&["battery", "info"])).await?;
    Ok(BatteryOut {
        limit: parse_battery_info(&raw),
        raw,
    })
}

#[tauri::command]
pub async fn battery_set_limit(percent: u8) -> Result<String, String> {
    run(args_battery_limit(percent)?).await
}

#[tauri::command]
pub async fn battery_one_shot(percent: Option<u8>) -> Result<String, String> {
    run(args_battery_oneshot(percent)?).await
}

#[tauri::command]
pub async fn fan_curve_get(profile: Option<Profile>) -> Result<String, String> {
    run(args_fan_get(profile)).await
}

#[tauri::command]
pub async fn fan_curve_set(
    profile: Profile,
    fan: Fan,
    points: Vec<FanPoint>,
) -> Result<String, String> {
    run(args_fan_set(profile, fan, &points)?).await
}

#[tauri::command]
pub async fn fan_curve_enable(
    profile: Profile,
    fan: Option<Fan>,
    enable: bool,
) -> Result<String, String> {
    run(args_fan_enable(profile, fan, enable)).await
}

#[tauri::command]
pub async fn fan_curve_reset() -> Result<String, String> {
    run(args_fan_reset()).await
}

#[derive(Serialize)]
pub struct ArmouryList {
    pub items: Vec<ArmouryItem>,
    pub raw: String,
}

#[tauri::command]
pub async fn armoury_list() -> Result<ArmouryList, String> {
    let raw = run(v(&["armoury", "list"])).await?;
    Ok(ArmouryList {
        items: parse_armoury_list(&raw),
        raw,
    })
}

#[tauri::command]
pub async fn armoury_get(name: String) -> Result<String, String> {
    run(args_armoury_get(&name)?).await
}

#[tauri::command]
pub async fn armoury_set(name: String, value: String) -> Result<String, String> {
    run(args_armoury_set(&name, &value)?).await
}

#[tauri::command]
pub async fn screenpad(req: ScreenpadReq) -> Result<String, String> {
    run(args_screenpad(&req)?).await
}

#[tauri::command]
pub async fn slash(req: SlashReq) -> Result<String, String> {
    run(args_slash(&req)?).await
}

// ---------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_list() {
        assert_eq!(
            parse_profile_list("Quiet\nBalanced\nPerformance\n"),
            ["Quiet", "Balanced", "Performance"]
        );
    }

    #[test]
    fn profile_get() {
        let r = parse_profile_get(
            "Active profile: Balanced\nProfile on AC: Performance\nProfile on Battery: Quiet\n",
        );
        assert_eq!(r.active.as_deref(), Some("Balanced"));
        assert_eq!(r.ac.as_deref(), Some("Performance"));
        assert_eq!(r.battery.as_deref(), Some("Quiet"));
    }

    #[test]
    fn leds() {
        assert_eq!(
            parse_leds_get("Current keyboard led brightness: Med\n").as_deref(),
            Some("med")
        );
        assert_eq!(parse_leds_get("nothing"), None);
    }

    #[test]
    fn battery() {
        assert_eq!(
            parse_battery_info("Current battery charge limit: 80%\n"),
            Some(80)
        );
        assert_eq!(parse_battery_info("garbage"), None);
    }

    #[test]
    fn armoury() {
        let out = "Multiple asusd interfaces devices found\npanel_od: [0, (1)]\nppt_pl1_spl: (45) 5 80\nBad Name: x\n";
        let r = parse_armoury_list(out);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].name, "panel_od");
        assert_eq!(r[0].current.as_deref(), Some("1"));
        assert_eq!(r[0].values, ["0", "1"]);
        assert_eq!(r[1].current.as_deref(), Some("45"));
        assert_eq!(r[1].values.len(), 3);
    }

    #[test]
    fn supported() {
        let out = "Supported Interfaces: [Aura, FanCurves, Slash]\nSupported Aura Modes: [Static, Breathe]\n";
        let s = parse_supported(out);
        assert_eq!(s.interfaces, ["Aura", "FanCurves", "Slash"]);
        assert_eq!(s.aura_modes, ["Static", "Breathe"]);
        assert!(s.fan_curves && s.slash && !s.screenpad);
    }

    #[test]
    fn profile_set_args() {
        let a = args_profile_set(Profile::Quiet, ProfileTarget::Both);
        assert_eq!(a.len(), 2);
        assert_eq!(a[0], ["profile", "set", "--ac", "--", "Quiet"]);
        assert_eq!(a[1][2], "--battery");
    }

    #[test]
    fn battery_range() {
        assert!(args_battery_limit(19).is_err());
        assert!(args_battery_limit(101).is_err());
        assert_eq!(args_battery_limit(80).unwrap(), ["battery", "limit", "80"]);
        assert!(args_battery_oneshot(Some(5)).is_err());
        assert_eq!(args_battery_oneshot(None).unwrap(), ["battery", "oneshot"]);
    }

    #[test]
    fn colour() {
        assert_eq!(norm_colour("#FF00aa").unwrap(), "ff00aa");
        assert!(norm_colour("ff00").is_err());
        assert!(norm_colour("gg0000").is_err());
        assert!(norm_colour("ff00ff; rm").is_err());
    }

    #[test]
    fn aura_effects() {
        let mut r = AuraEffectReq {
            effect: Some(AuraEffect::Breathe),
            colour: Some("ff0000".into()),
            ..Default::default()
        };
        assert!(args_aura_effect(&r).is_err());
        r.colour2 = Some("00ff00".into());
        r.speed = Some(Speed::Low);
        r.zone = Some("logo".into());
        assert_eq!(
            args_aura_effect(&r).unwrap(),
            [
                "aura",
                "effect",
                "breathe",
                "--colour",
                "ff0000",
                "--colour2",
                "00ff00",
                "--speed",
                "low",
                "--zone",
                "logo"
            ]
        );
        r.zone = Some("a b".into());
        assert!(args_aura_effect(&r).is_err());
        let w = AuraEffectReq {
            effect: Some(AuraEffect::RainbowWave),
            speed: Some(Speed::High),
            direction: Some(Direction::Up),
            ..Default::default()
        };
        assert_eq!(args_aura_effect(&w).unwrap().len(), 7);
        let n = AuraEffectReq {
            switch: Some("next".into()),
            ..Default::default()
        };
        assert_eq!(
            args_aura_effect(&n).unwrap(),
            ["aura", "effect", "--next-mode"]
        );
    }

    #[test]
    fn aura_power() {
        assert_eq!(
            args_aura_power(PowerZone::RearGlow, true, false, false, true),
            ["aura", "power", "rear-glow", "--boot", "--shutdown"]
        );
    }

    #[test]
    fn fan_curve() {
        let p = |t, c| FanPoint { temp: t, pct: c };
        assert_eq!(
            fan_data_string(&[p(30, 1), p(49, 2)]).unwrap(),
            "30c:1%,49c:2%"
        );
        assert!(fan_data_string(&[]).is_err());
        assert!(fan_data_string(&[p(10, 1)]).is_err());
        assert!(fan_data_string(&[p(30, 101)]).is_err());
        assert!(fan_data_string(&[p(50, 1), p(40, 2)]).is_err());
        let a = args_fan_set(Profile::Balanced, Fan::Cpu, &[p(30, 1)]).unwrap();
        assert_eq!(a.last().unwrap(), "30c:1%");
        assert_eq!(
            args_fan_enable(Profile::Quiet, None, true).last().unwrap(),
            "true"
        );
        assert_eq!(args_fan_reset(), ["fan-curve", "--default"]);
    }

    #[test]
    fn armoury_args() {
        assert!(args_armoury_get("panel_od").is_ok());
        assert!(args_armoury_get("Panel").is_err());
        assert!(args_armoury_get("a b").is_err());
        assert_eq!(
            args_armoury_set("panel_od", "1").unwrap(),
            ["armoury", "set", "--", "panel_od", "1"]
        );
        assert!(args_armoury_set("panel_od", "1; ls").is_err());
        assert!(args_armoury_set("panel_od", "").is_err());
    }

    #[test]
    fn screenpad_slash() {
        assert!(
            args_screenpad(&ScreenpadReq {
                brightness: Some(101),
                ..Default::default()
            })
            .is_err()
        );
        assert!(
            args_screenpad(&ScreenpadReq {
                gamma: Some(2.3),
                ..Default::default()
            })
            .is_err()
        );
        assert!(
            args_screenpad(&ScreenpadReq {
                gamma: Some(0.4),
                ..Default::default()
            })
            .is_err()
        );
        assert_eq!(
            args_screenpad(&ScreenpadReq {
                brightness: Some(50),
                gamma: Some(1.0),
                sync: None
            })
            .unwrap(),
            [
                "backlight",
                "--screenpad-brightness",
                "50",
                "--screenpad-gamma",
                "1.00"
            ]
        );
        assert!(
            args_slash(&SlashReq {
                interval: Some(6),
                ..Default::default()
            })
            .is_err()
        );
        assert!(
            args_slash(&SlashReq {
                mode: Some("x;y".into()),
                ..Default::default()
            })
            .is_err()
        );
        assert_eq!(
            args_slash(&SlashReq {
                enable: Some(true),
                brightness: Some(255),
                ..Default::default()
            })
            .unwrap(),
            ["slash", "--enable", "--brightness", "255"]
        );
    }

    #[test]
    fn classify_outcomes() {
        assert!(classify(true, "ok\n").is_ok());
        assert!(classify(false, "").is_err());
        assert!(classify(true, "Error: something bad\n").is_err());
        let e = classify(true, "Error: org.freedesktop.DBus.Error.UnknownInterface").unwrap_err();
        assert!(e.starts_with("unsupported:"));
    }
}
