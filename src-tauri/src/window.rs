use tauri::{Window, PhysicalSize, Size};

pub fn update_window_state(window: &Window, connected: bool) -> Result<(), String> {
    if connected {
        let csize = PhysicalSize { width: 1920, height: 1080 };
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
            .map_err(|e| format!("恢复鼠标事件失败: {}", e))?;
        let dsize = PhysicalSize { width: 400, height: 600 };
        window
            .set_size(Size::Physical(dsize))
            .map_err(|e| format!("设置窗口大小失败: {}", e))?;
        window.center().map_err(|e| format!("居中失败: {}", e))?;
    }
    Ok(())
}