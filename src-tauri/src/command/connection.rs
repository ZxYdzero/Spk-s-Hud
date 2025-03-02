use crate::config::get_config;
use crate::window::update_window_state;

use super::super::AppState;
use tauri::{Manager, Window};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tauri::command]
pub async fn connect(
    window: Window,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    // 先更新窗口状态（变为全屏并忽略鼠标）
    update_window_state(&window, true)?;

    // 确保同一时间只有一个连接
    {
        let mut active = state.connection_active.lock().await;
        if *active {
            return Err("TCP连接已存在".into());
        }
        *active = true;
    }

    // 克隆 state 和窗口句柄以便在异步任务中使用
    let state_clone = state.inner().clone();
    let window_clone = window.clone();

    let join_handle = tokio::spawn(async move {
        let config = match get_config() {
            Ok(c) => c,
            Err(e) => {
                let _ = window_clone.emit_all("log", format!("读取配置失败: {}", e));
                let mut active = state_clone.connection_active.lock().await;
                *active = false;
                return;
            }
        };

        match TcpStream::connect(&config.host).await {
            Ok(mut stream) => {
                let auth_packet = format!("{}\n", config.password);
                if let Err(e) = stream.write_all(auth_packet.as_bytes()).await {
                    let _ = window_clone.emit_all("log", format!("发送密码失败: {}", e));
                    let mut active = state_clone.connection_active.lock().await;
                    *active = false;
                    return;
                }
                let mut auth_response = [0u8; 12];
                match stream.read_exact(&mut auth_response).await {
                    Ok(_) => {
                        if auth_response == *b"AUTH_SUCCESS" {
                            let _ = window_clone.emit_all("log", "认证成功, 开始接收数据".to_string());
                            let mut buffer = [0u8; 2048];
                            loop {
                                match stream.read(&mut buffer).await {
                                    Ok(size) if size > 0 => {
                                        let data = String::from_utf8_lossy(&buffer[..size]).to_string();
                                        if let Ok(json) = serde_json::from_str::<Value>(&data) {
                                            let _ = window_clone.emit_all("update_data", json.clone());
                                            let mut game_data = state_clone.game_data.lock().await;
                                            *game_data = Some(json);
                                        }
                                    }
                                    Ok(_) => {
                                        let _ = window_clone.emit_all("log", "连接已关闭".to_string());
                                        break;
                                    }
                                    Err(e) => {
                                        let _ = window_clone.emit_all("log", format!("读取数据失败: {}", e));
                                        break;
                                    }
                                }
                            }
                        } else {
                            let _ = window_clone.emit_all("log", "认证失败，请检查密码".to_string());
                        }
                    }
                    Err(e) => {
                        let _ = window_clone.emit_all("log", format!("读取认证响应失败: {}", e));
                    }
                }
            }
            Err(e) => {
                let _ = window_clone.emit_all("log", format!("连接失败: {}", e));
            }
        }
        let mut active = state_clone.connection_active.lock().await;
        *active = false;
    });

    {
        let mut handle_lock = state.connection_handle.lock().await;
        *handle_lock = Some(join_handle);
    }

    Ok(())
}

#[tauri::command]
pub async fn disconnect(
    window: Window,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    {
        let mut handle_lock = state.connection_handle.lock().await;
        if let Some(handle) = handle_lock.take() {
            handle.abort();
        }
    }
    {
        let mut active = state.connection_active.lock().await;
        *active = false;
    }
    update_window_state(&window, false)?;

    let _ = window.emit_all("log", "断开连接".to_string());
    Ok(())
}