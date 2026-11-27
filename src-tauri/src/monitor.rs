//! Real-time Linux system telemetry from `/proc` and `/sys`.

use serde::Serialize;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default, Serialize)]
pub struct SystemMetrics {
    pub cpu_usage_pct: f32,
    pub cpu_per_core_pct: Vec<f32>,
    pub cpu_freq_mhz: Option<f32>,
    pub load_avg: [f32; 3],
    pub uptime_secs: u64,
    pub mem_total_bytes: u64,
    pub mem_used_bytes: u64,
    pub mem_pct: f32,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_pct: f32,
    pub cpu_temp_c: Option<f32>,
    pub gpu_temp_c: Option<f32>,
    pub nvme_temp_c: Option<f32>,
    pub cpu_fan_rpm: Option<u32>,
    pub gpu_fan_rpm: Option<u32>,
    pub battery_pct: Option<u8>,
    pub battery_status: Option<String>,
    pub battery_power_w: Option<f32>,
    pub ac_online: Option<bool>,
    pub net_rx_bytes_per_sec: f64,
    pub net_tx_bytes_per_sec: f64,
    pub disk_read_bytes_per_sec: f64,
    pub disk_write_bytes_per_sec: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CpuTimes {
    pub idle: u64,
    pub total: u64,
}

#[derive(Default)]
struct MonitorState {
    last_sample: Option<Instant>,
    prev_cpu_total: Option<CpuTimes>,
    prev_cpu_cores: Vec<CpuTimes>,
    prev_net: Option<(u64, u64)>,
    prev_disk: Option<(u64, u64)>,
}

static MONITOR_STATE: OnceLock<Mutex<MonitorState>> = OnceLock::new();

pub fn calc_cpu_pct(prev: CpuTimes, curr: CpuTimes) -> f32 {
    let total_delta = curr.total.saturating_sub(prev.total);
    if total_delta == 0 {
        return 0.0;
    }
    let idle_delta = curr.idle.saturating_sub(prev.idle);
    let busy = total_delta.saturating_sub(idle_delta);
    ((busy as f64 / total_delta as f64) * 100.0).clamp(0.0, 100.0) as f32
}

pub fn parse_proc_stat(content: &str) -> (CpuTimes, Vec<CpuTimes>) {
    let mut total_times = CpuTimes::default();
    let mut cores = Vec::new();

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let Some(label) = parts.next() else {
            continue;
        };
        if !label.starts_with("cpu") {
            continue;
        }
        let nums: Vec<u64> = parts.filter_map(|p| p.parse::<u64>().ok()).collect();
        if nums.len() < 4 {
            continue;
        }
        let idle = nums[3].saturating_add(*nums.get(4).unwrap_or(&0));
        let total: u64 = nums.iter().copied().sum();
        let ct = CpuTimes { idle, total };

        if label == "cpu" {
            total_times = ct;
        } else if label[3..].chars().all(|c| c.is_ascii_digit()) {
            cores.push(ct);
        }
    }

    (total_times, cores)
}

pub fn parse_proc_meminfo(content: &str) -> (u64, u64, u64, u64) {
    let mut mem_total_kb = 0u64;
    let mut mem_avail_kb = None;
    let mut mem_free_kb = 0u64;
    let mut swap_total_kb = 0u64;
    let mut swap_free_kb = 0u64;

    for line in content.lines() {
        let Some((key, rest)) = line.split_once(':') else {
            continue;
        };
        let val = rest
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        match key {
            "MemTotal" => mem_total_kb = val,
            "MemAvailable" => mem_avail_kb = Some(val),
            "MemFree" => mem_free_kb = val,
            "SwapTotal" => swap_total_kb = val,
            "SwapFree" => swap_free_kb = val,
            _ => {}
        }
    }

    let mem_avail = mem_avail_kb.unwrap_or(mem_free_kb);
    let mem_used_kb = mem_total_kb.saturating_sub(mem_avail);
    let swap_used_kb = swap_total_kb.saturating_sub(swap_free_kb);

    (
        mem_total_kb * 1024,
        mem_used_kb * 1024,
        swap_total_kb * 1024,
        swap_used_kb * 1024,
    )
}

pub fn parse_proc_loadavg(content: &str) -> [f32; 3] {
    let mut it = content
        .split_whitespace()
        .take(3)
        .filter_map(|s| s.parse::<f32>().ok());
    [
        it.next().unwrap_or(0.0),
        it.next().unwrap_or(0.0),
        it.next().unwrap_or(0.0),
    ]
}

pub fn parse_proc_uptime(content: &str) -> u64 {
    content
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .map(|v| v.max(0.0) as u64)
        .unwrap_or(0)
}

pub fn parse_proc_net_dev(content: &str) -> (u64, u64) {
    let mut rx_sum = 0u64;
    let mut tx_sum = 0u64;

    for line in content.lines().skip(2) {
        let Some((iface, rest)) = line.split_once(':') else {
            continue;
        };
        if iface.trim() == "lo" {
            continue;
        }
        let cols: Vec<u64> = rest
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        if cols.len() >= 9 {
            rx_sum = rx_sum.saturating_add(cols[0]);
            tx_sum = tx_sum.saturating_add(cols[8]);
        }
    }

    (rx_sum, tx_sum)
}

fn is_whole_disk(name: &str) -> bool {
    if name.starts_with("loop")
        || name.starts_with("ram")
        || name.starts_with("dm-")
        || name.starts_with("zram")
        || name.starts_with("sr")
    {
        return false;
    }
    if let Some(rest) = name.strip_prefix("nvme") {
        // nvme0n1 -> whole disk; nvme0n1p1 -> partition
        return !rest.contains('p') && rest.contains('n');
    }
    if let Some(rest) = name.strip_prefix("mmcblk") {
        return !rest.contains('p') && !rest.is_empty();
    }
    if name.starts_with("sd") || name.starts_with("vd") {
        return name.len() >= 3 && !name.chars().last().unwrap_or('0').is_ascii_digit();
    }
    false
}

pub fn parse_proc_diskstats(content: &str) -> (u64, u64) {
    let mut read_bytes = 0u64;
    let mut write_bytes = 0u64;

    for line in content.lines() {
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 10 {
            continue;
        }
        let name = cols[2];
        if !is_whole_disk(name) {
            continue;
        }
        let sectors_read = cols[5].parse::<u64>().unwrap_or(0);
        let sectors_written = cols[9].parse::<u64>().unwrap_or(0);
        read_bytes = read_bytes.saturating_add(sectors_read.saturating_mul(512));
        write_bytes = write_bytes.saturating_add(sectors_written.saturating_mul(512));
    }

    (read_bytes, write_bytes)
}

fn read_cpu_freq_mhz() -> Option<f32> {
    let entries = std::fs::read_dir("/sys/devices/system/cpu").ok()?;
    let mut sum_khz = 0f64;
    let mut count = 0u32;

    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name_str) = name.to_str() else {
            continue;
        };
        if !name_str.starts_with("cpu") || !name_str[3..].chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let freq_path = entry.path().join("cpufreq/scaling_cur_freq");
        if let Ok(raw) = std::fs::read_to_string(freq_path)
            && let Ok(khz) = raw.trim().parse::<f64>()
        {
            sum_khz += khz;
            count += 1;
        }
    }

    if count > 0 {
        Some((sum_khz / count as f64 / 1000.0) as f32)
    } else {
        None
    }
}

fn read_hwmon(metrics: &mut SystemMetrics) {
    let Ok(entries) = std::fs::read_dir("/sys/class/hwmon") else {
        return;
    };

    let mut acpitz_temp: Option<f32> = None;

    for entry in entries.flatten() {
        let dir = entry.path();
        let name = std::fs::read_to_string(dir.join("name"))
            .unwrap_or_default()
            .trim()
            .to_ascii_lowercase();

        let is_cpu_direct =
            name.contains("coretemp") || name.contains("k10temp") || name.contains("zenpower");
        let is_gpu = name.contains("amdgpu") || name.contains("nouveau") || name.contains("nvidia");

        if is_cpu_direct
            && metrics.cpu_temp_c.is_none()
            && let Ok(raw) = std::fs::read_to_string(dir.join("temp1_input"))
            && let Ok(milli) = raw.trim().parse::<f32>()
        {
            metrics.cpu_temp_c = Some((milli / 1000.0).round());
        } else if name.contains("acpitz")
            && acpitz_temp.is_none()
            && let Ok(raw) = std::fs::read_to_string(dir.join("temp1_input"))
            && let Ok(milli) = raw.trim().parse::<f32>()
        {
            acpitz_temp = Some((milli / 1000.0).round());
        }

        if is_gpu
            && metrics.gpu_temp_c.is_none()
            && let Ok(raw) = std::fs::read_to_string(dir.join("temp1_input"))
            && let Ok(milli) = raw.trim().parse::<f32>()
        {
            metrics.gpu_temp_c = Some((milli / 1000.0).round());
        }

        if name.contains("nvme")
            && metrics.nvme_temp_c.is_none()
            && let Ok(raw) = std::fs::read_to_string(dir.join("temp1_input"))
            && let Ok(milli) = raw.trim().parse::<f32>()
        {
            metrics.nvme_temp_c = Some((milli / 1000.0).round());
        }

        if name.contains("asus") {
            if metrics.cpu_fan_rpm.is_none()
                && let Ok(raw) = std::fs::read_to_string(dir.join("fan1_input"))
            {
                metrics.cpu_fan_rpm = raw.trim().parse::<u32>().ok();
            }
            if metrics.gpu_fan_rpm.is_none()
                && let Ok(raw) = std::fs::read_to_string(dir.join("fan2_input"))
            {
                metrics.gpu_fan_rpm = raw.trim().parse::<u32>().ok();
            }
        }
    }

    if metrics.cpu_temp_c.is_none() {
        metrics.cpu_temp_c = acpitz_temp;
    }
}

fn read_power_supply(metrics: &mut SystemMetrics) {
    let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") else {
        return;
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name_str) = name.to_str() else {
            continue;
        };
        let dir = entry.path();

        if name_str.starts_with("BAT") && metrics.battery_pct.is_none() {
            if let Ok(raw) = std::fs::read_to_string(dir.join("capacity")) {
                metrics.battery_pct = raw.trim().parse::<u8>().ok();
            }
            if let Ok(raw) = std::fs::read_to_string(dir.join("status")) {
                let s = raw.trim().to_string();
                if !s.is_empty() {
                    metrics.battery_status = Some(s);
                }
            }
            if let Ok(raw) = std::fs::read_to_string(dir.join("power_now"))
                && let Ok(uw) = raw.trim().parse::<f64>()
            {
                metrics.battery_power_w = Some((uw / 1_000_000.0) as f32);
            } else if let (Ok(c_raw), Ok(v_raw)) = (
                std::fs::read_to_string(dir.join("current_now")),
                std::fs::read_to_string(dir.join("voltage_now")),
            ) && let (Ok(ua), Ok(uv)) =
                (c_raw.trim().parse::<f64>(), v_raw.trim().parse::<f64>())
            {
                metrics.battery_power_w = Some(((ua * uv) / 1e12) as f32);
            }
        } else if (name_str.starts_with("AC") || name_str == "ACAD") && metrics.ac_online.is_none()
            && let Ok(raw) = std::fs::read_to_string(dir.join("online"))
        {
            match raw.trim() {
                "1" => metrics.ac_online = Some(true),
                "0" => metrics.ac_online = Some(false),
                _ => {}
            }
        }
    }
}

fn collect_metrics() -> SystemMetrics {
    let lock = MONITOR_STATE.get_or_init(|| Mutex::new(MonitorState::default()));
    let mut state = lock.lock().unwrap_or_else(|e| e.into_inner());

    // Ensure first sample has a baseline so initial render shows real CPU %
    if state.prev_cpu_total.is_none() {
        if let Ok(stat) = std::fs::read_to_string("/proc/stat") {
            let (total, cores) = parse_proc_stat(&stat);
            state.prev_cpu_total = Some(total);
            state.prev_cpu_cores = cores;
        }
        if let Ok(net) = std::fs::read_to_string("/proc/net/dev") {
            state.prev_net = Some(parse_proc_net_dev(&net));
        }
        if let Ok(disk) = std::fs::read_to_string("/proc/diskstats") {
            state.prev_disk = Some(parse_proc_diskstats(&disk));
        }
        state.last_sample = Some(Instant::now());
        std::thread::sleep(Duration::from_millis(80));
    }

    let now = Instant::now();
    let elapsed_secs = state
        .last_sample
        .map(|t| now.duration_since(t).as_secs_f64().max(0.05))
        .unwrap_or(1.0);

    let mut m = SystemMetrics::default();

    if let Ok(stat) = std::fs::read_to_string("/proc/stat") {
        let (curr_total, curr_cores) = parse_proc_stat(&stat);
        if let Some(prev_total) = state.prev_cpu_total {
            m.cpu_usage_pct = calc_cpu_pct(prev_total, curr_total);
        }
        m.cpu_per_core_pct = curr_cores
            .iter()
            .enumerate()
            .map(|(i, curr)| {
                state
                    .prev_cpu_cores
                    .get(i)
                    .map(|prev| calc_cpu_pct(*prev, *curr))
                    .unwrap_or(0.0)
            })
            .collect();
        state.prev_cpu_total = Some(curr_total);
        state.prev_cpu_cores = curr_cores;
    }

    m.cpu_freq_mhz = read_cpu_freq_mhz();

    if let Ok(load) = std::fs::read_to_string("/proc/loadavg") {
        m.load_avg = parse_proc_loadavg(&load);
    }
    if let Ok(up) = std::fs::read_to_string("/proc/uptime") {
        m.uptime_secs = parse_proc_uptime(&up);
    }

    if let Ok(mem) = std::fs::read_to_string("/proc/meminfo") {
        let (mt, mu, st, su) = parse_proc_meminfo(&mem);
        m.mem_total_bytes = mt;
        m.mem_used_bytes = mu;
        m.mem_pct = if mt > 0 {
            ((mu as f64 / mt as f64) * 100.0) as f32
        } else {
            0.0
        };
        m.swap_total_bytes = st;
        m.swap_used_bytes = su;
        m.swap_pct = if st > 0 {
            ((su as f64 / st as f64) * 100.0) as f32
        } else {
            0.0
        };
    }

    if let Ok(net) = std::fs::read_to_string("/proc/net/dev") {
        let curr = parse_proc_net_dev(&net);
        if let Some(prev) = state.prev_net {
            m.net_rx_bytes_per_sec = curr.0.saturating_sub(prev.0) as f64 / elapsed_secs;
            m.net_tx_bytes_per_sec = curr.1.saturating_sub(prev.1) as f64 / elapsed_secs;
        }
        state.prev_net = Some(curr);
    }

    if let Ok(disk) = std::fs::read_to_string("/proc/diskstats") {
        let curr = parse_proc_diskstats(&disk);
        if let Some(prev) = state.prev_disk {
            m.disk_read_bytes_per_sec = curr.0.saturating_sub(prev.0) as f64 / elapsed_secs;
            m.disk_write_bytes_per_sec = curr.1.saturating_sub(prev.1) as f64 / elapsed_secs;
        }
        state.prev_disk = Some(curr);
    }

    state.last_sample = Some(now);
    drop(state);

    read_hwmon(&mut m);
    read_power_supply(&mut m);

    m
}

#[tauri::command]
pub async fn get_system_metrics() -> Result<SystemMetrics, String> {
    tauri::async_runtime::spawn_blocking(collect_metrics)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proc_stat_and_usage() {
        let s1 = "cpu  100 0 50 850 0 0 0 0 0 0\ncpu0 50 0 25 425 0 0 0 0 0 0\ncpu1 50 0 25 425 0 0 0 0 0 0\n";
        let s2 = "cpu  150 0 50 900 0 0 0 0 0 0\ncpu0 80 0 25 445 0 0 0 0 0 0\ncpu1 70 0 25 455 0 0 0 0 0 0\n";
        let (t1, c1) = parse_proc_stat(s1);
        let (t2, c2) = parse_proc_stat(s2);
        assert_eq!(c1.len(), 2);
        assert_eq!(c2.len(), 2);
        assert!((calc_cpu_pct(t1, t2) - 50.0).abs() < 0.01);
        assert!((calc_cpu_pct(c1[0], c2[0]) - 60.0).abs() < 0.01);
        assert!((calc_cpu_pct(c1[1], c2[1]) - 40.0).abs() < 0.01);
    }

    #[test]
    fn proc_meminfo() {
        let m = "MemTotal:       16000000 kB\nMemFree:         1000000 kB\nMemAvailable:    4000000 kB\nSwapTotal:       8000000 kB\nSwapFree:        6000000 kB\n";
        let (mt, mu, st, su) = parse_proc_meminfo(m);
        assert_eq!(mt, 16_000_000 * 1024);
        assert_eq!(mu, 12_000_000 * 1024);
        assert_eq!(st, 8_000_000 * 1024);
        assert_eq!(su, 2_000_000 * 1024);
    }

    #[test]
    fn proc_loadavg_and_uptime() {
        assert_eq!(
            parse_proc_loadavg("1.25 0.85 0.50 2/400 1234\n"),
            [1.25, 0.85, 0.50]
        );
        assert_eq!(parse_proc_uptime("7089.76 50136.38\n"), 7089);
    }

    #[test]
    fn proc_net_dev() {
        let n = "Inter-|   Receive                                                |  Transmit\n face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed\n    lo: 1000 10 0 0 0 0 0 0 1000 10 0 0 0 0 0 0\n  wlan0: 5000 10 0 0 0 0 0 0 2500 10 0 0 0 0 0 0\n  eth0: 1000 5 0 0 0 0 0 0 500 5 0 0 0 0 0 0\n";
        assert_eq!(parse_proc_net_dev(n), (6000, 3000));
    }

    #[test]
    fn proc_diskstats() {
        let d = "   7 0 loop0 0 0 100 0 0 0 100 0 0 0 0\n 259 0 nvme0n1 10 0 200 0 10 0 400 0 0 0 0\n 259 1 nvme0n1p1 10 0 200 0 10 0 400 0 0 0 0\n   8 0 sda 10 0 100 0 10 0 200 0 0 0 0\n   8 1 sda1 10 0 100 0 10 0 200 0 0 0 0\n";
        assert_eq!(parse_proc_diskstats(d), (300 * 512, 600 * 512));
    }
}
