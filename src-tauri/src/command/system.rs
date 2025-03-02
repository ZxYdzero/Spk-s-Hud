#[tauri::command]
pub fn exit_app() {
    std::process::exit(0);
}

#[tauri::command]
pub fn ignore_cursor(window: tauri::Window, ignore: bool) {
    window.set_ignore_cursor_events(ignore).unwrap();
}