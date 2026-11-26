pub mod asusctl;

use asusctl::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_info,
            get_thermals,
            profile_list,
            profile_get,
            profile_set,
            leds_get,
            leds_set,
            leds_next,
            leds_prev,
            aura_effect,
            aura_power,
            battery_info,
            battery_set_limit,
            battery_one_shot,
            fan_curve_get,
            fan_curve_set,
            fan_curve_enable,
            fan_curve_reset,
            armoury_list,
            armoury_get,
            armoury_set,
            screenpad,
            slash
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
