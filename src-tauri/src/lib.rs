use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WebviewUrl, WebviewWindowBuilder,
};

/// Show the floating body-double widget. Creates it on first call,
/// then just shows + focuses it on subsequent calls.
#[tauri::command]
async fn open_floating_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("floating") {
        win.show().map_err(|e| e.to_string())?;
        win.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(&app, "floating", WebviewUrl::App("/#/floating".into()))
        .title("Tea Table")
        .inner_size(280.0, 400.0)
        .min_inner_size(240.0, 340.0)
        .resizable(true)
        .decorations(false)
        .always_on_top(true)
        .build()
        .map_err(|e: tauri::Error| e.to_string())?;

    Ok(())
}

/// Hide the floating widget (keeps it alive in memory).
#[tauri::command]
async fn close_floating_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("floating") {
        win.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            open_floating_window,
            close_floating_window
        ])
        .setup(|app| {
            // ── System tray ──────────────────────────────────────────────
            let show_item = MenuItem::with_id(app, "show", "Show Tea Table", true, None::<&str>)?;
            let widget_item = MenuItem::with_id(app, "widget", "Show Widget", true, None::<&str>)?;
            let sep = tauri::menu::PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let tray_menu = Menu::with_items(app, &[&show_item, &widget_item, &sep, &quit_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .tooltip("Tea Table — House of Katha")
                // Left-click: toggle main window
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            if win.is_visible().unwrap_or(false) {
                                let _ = win.hide();
                            } else {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                    }
                })
                // Menu item clicks
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "widget" => {
                        // Show floating widget
                        if let Some(win) = app.get_webview_window("floating") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        // ── Prevent close; hide to tray instead ──────────────────────────
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Both main and floating hide rather than close
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
