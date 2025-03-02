#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod config;
mod window;

#[derive(Clone)]
struct AppState {
    game_data: std::sync::Arc<tokio::sync::Mutex<Option<serde_json::Value>>>,
    connection_active: std::sync::Arc<tokio::sync::Mutex<bool>>,
    connection_handle: std::sync::Arc<tokio::sync::Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            game_data: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
            connection_active: std::sync::Arc::new(tokio::sync::Mutex::new(false)),
            connection_handle: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            command::system::exit_app,
            command::system::ignore_cursor,
            command::connection::connect,
            command::connection::disconnect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}