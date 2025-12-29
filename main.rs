// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::thread;
use std::time::Duration;

// --- Data Structures ---

#[derive(Serialize)]
struct SystemDiagnostics {
    cpu_temp: f32,
    battery_health: u8,
    fan_rpm: u32,
    model: String,
}

// --- Tauri Commands (The "API") ---

#[tauri::command]
fn get_diagnostics() -> SystemDiagnostics {
    // Mocking real hardware reads
    SystemDiagnostics {
        cpu_temp: 45.5,
        battery_health: 92,
        fan_rpm: 3200,
        model: "Google Pixelbook (Eve)".to_string(),
    }
}

#[tauri::command]
fn set_fan_curve(profile: String) -> String {
    // Simulate applying EC commands
    println!("Applying Fan Profile: {}", profile);
    thread::sleep(Duration::from_millis(500)); // Fake delay for realism
    format!("Fan curve set to '{}' mode successfully.", profile)
}

#[tauri::command]
fn set_backlight(brightness: u8) -> String {
    println!("Setting Keyboard Backlight to: {}%", brightness);
    format!("Backlight updated to {}%", brightness)
}

#[tauri::command]
fn remap_key(source: String, target: String) -> String {
    println!("Remapping Key: {} -> {}", source, target);
    format!("Key '{}' is now mapped to '{}'", source, target)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_diagnostics,
            set_fan_curve,
            set_backlight,
            remap_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}