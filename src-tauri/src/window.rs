use tauri::{Window, PhysicalSize, Size};

pub fn update_window_state(window: &Window, connected: bool) -> Result<(), String> {
    if connected {
        // 动态获取主屏幕分辨率
        let monitor = window.primary_monitor().map_err(|e| format!("获取主屏幕信息失败: {}", e))?;
        let csize = if let Some(monitor) = monitor {
            let size = monitor.size();
            PhysicalSize { width: size.width, height: size.height }
        } else {
            PhysicalSize { width: 1920, height: 1080 } // 获取失败时兜底
        };
        window
            .set_size(Size::Physical(csize))
            .map_err(|e| format!("设置窗口大小失败: {}", e))?;
        window
            .set_ignore_cursor_events(true)
            .map_err(|e| format!("设置忽略鼠标失败: {}", e))?;
        window.center().map_err(|e| format!("居中失败: {}", e))?;
    } else {
        window
            .set_ignore_cursor_events(false)
            .map_err(|e| window.emit("log", "恢复鼠标事件失败".to_string()));
        let dsize = PhysicalSize { width: 480, height: 740 };
        window
            .set_size(Size::Physical(dsize))
            .map_err(|e| format!("设置窗口大小失败: {}", e))?;
        window.center().map_err(|e| format!("居中失败: {}", e))?;
    }
    Ok(())
}