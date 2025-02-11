#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
    sync::{Arc, Mutex},
};
use tauri::Manager;

#[derive(Default)]
struct AppState {
    game_data: Arc<Mutex<Option<Value>>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    host: String,
    password: String,
    debug_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost:32456".to_string(),
            password: "default_password".to_string(),
            debug_mode: false,
        }
    }
}

fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = "config.toml";

    // 如果配置文件不存在则创建
    if !Path::new(config_path).exists() {
        let default_config = Config::default();
        let toml_str = toml::to_string_pretty(&default_config)?;
        let mut file = fs::File::create(config_path)?;
        file.write("# 请注意,IP后面需要加端口号.\n".as_bytes())
            .unwrap();
        file.write_all(toml_str.as_bytes())?;
    }

    let content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

fn main() {
    let config = get_config().unwrap();
    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            let handle = app.handle();
            let state = app.state::<AppState>().game_data.clone();
            std::thread::spawn(move || {
                match TcpStream::connect(&config.host) {
                    Ok(mut stream) => {
                        // ====== 认证阶段 ======
                        // 发送密码（带换行符作为结束标志）
                        let auth_packet = format!("{}\n", config.password);
                        if let Err(e) = stream.write_all(auth_packet.as_bytes()) {
                            eprintln!("[ERR] 发送密码失败: {}", e);
                            return;
                        }

                        // 读取服务端响应
                        let mut auth_response = [0u8; 12];
                        match stream.read_exact(&mut auth_response) {
                            Ok(_) => {
                                if auth_response == *b"AUTH_SUCCESS" {
                                    // ====== 认证成功，开始接收数据 ======
                                    let mut buffer = [0; 2048];
                                    loop {
                                        match stream.read(&mut buffer) {
                                            Ok(size) if size > 0 => {
                                                let data = String::from_utf8_lossy(&buffer[..size]);
                                                if let Ok(json) =
                                                    serde_json::from_str::<Value>(&data)
                                                {
                                                    let mut game_data = state.lock().unwrap();
                                                    handle
                                                        .emit_all("update_data", json.clone())
                                                        .unwrap();
                                                    *game_data = Some(json);
                                                }
                                            }
                                            _ => break,
                                        }
                                    }
                                } else {
                                    eprintln!("[ERR] 认证失败，请检查密码");
                                }
                            }
                            Err(e) => eprintln!("[ERR] 读取响应失败: {}", e),
                        }
                    }
                    Err(e) => eprintln!("[ERR] 连接失败: {}", e),
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![ignore_cursor, exit_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn ignore_cursor(window: tauri::Window, ignore: bool) {
    window.set_ignore_cursor_events(ignore).unwrap();
}

#[tauri::command]
fn exit_app() {
    std::process::exit(0);
}
