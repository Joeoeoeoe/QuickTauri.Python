use tauri::State;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::cache::InstanceCache;


#[tauri::command]
pub fn test_command() -> String {
    format!("Hello, this is command.rs from Rust!")
}

#[tauri::command]
#[allow(dead_code)]
pub async fn call_python_script(
    script_path: String,
    function_name: String,
    args: Vec<String>,
    state: State<'_, Arc<Mutex<Mutex<InstanceCache>>>>,
) -> Result<String, String> {
    let script_path = PathBuf::from(script_path);
    let function_name = function_name.clone();
    let args = args.clone();
    let state = state.inner().clone();

    let result = tokio::task::spawn_blocking(move || {
        let outer_guard = state.lock()
            .map_err(|e| format!("Failed to acquire cache lock: {}", e))?;
        
        let mut instance_cache = outer_guard.lock()
            .map_err(|e| format!("Failed to acquire instance cache lock: {}", e))?;

        let interface = instance_cache.get_or_create(script_path);

        interface.call_python_function(&function_name, args)
            .map_err(|e| e.to_string())
    }).await;

    match result {
        Ok(inner_result) => inner_result,
        Err(join_error) => Err(format!("Task failed to complete: {}", join_error)),
    }
}
