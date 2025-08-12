// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod python_interface;
mod cache;

use commands::call_python_script;
use std::sync::{Arc, Mutex};

fn main() {
    // 创建实例缓存
    let instance_cache = crate::cache::InstanceCache::new();
    let cached_interface = Arc::new(Mutex::new(Mutex::new(instance_cache)));

    tauri::Builder::default()
        .manage(cached_interface)
        .invoke_handler(tauri::generate_handler![
            commands::test_command,
            call_python_script
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}