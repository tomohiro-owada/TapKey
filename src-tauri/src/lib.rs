mod config;
mod keyboard;
mod qr;
mod server;

use config::AppConfig;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

#[tauri::command]
fn get_config() -> AppConfig {
    AppConfig::load()
}

#[tauri::command]
fn save_config(config: AppConfig) -> Result<(), String> {
    config.save().map_err(|e| e.to_string())?;
    // WebSocket経由で全クライアントに設定更新を通知
    server::notify_config_updated();
    Ok(())
}

#[tauri::command]
fn get_server_url() -> String {
    let config = AppConfig::load();
    qr::get_server_url(config.port)
}

#[tauri::command]
fn get_qr_code() -> Result<String, String> {
    let config = AppConfig::load();
    let url = qr::get_server_url_with_pin(config.port, &config.pin);
    qr::generate_qr_code(&url)
}

#[tauri::command]
fn get_local_ip() -> Option<String> {
    qr::get_local_ip().map(|ip| ip.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 設定を読み込み
    let config = AppConfig::load();
    let port = config.port;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            // HTTPサーバーを別スレッドで起動
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
                rt.block_on(async {
                    if let Err(e) = server::start_server(port).await {
                        eprintln!("HTTP Server error: {}", e);
                    }
                });
            });

            // Create tray menu
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let settings_item = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
            let show_qr_item = MenuItem::with_id(app, "show_qr", "Show QR Code", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_qr_item, &settings_item, &quit_item])?;

            // トレイアイコンを作成
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "settings" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().ok();
                            window.set_focus().ok();
                        }
                    }
                    "show_qr" => {
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().ok();
                            window.set_focus().ok();
                            window.emit("show-qr", ()).ok();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().ok();
                            window.set_focus().ok();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            get_server_url,
            get_qr_code,
            get_local_ip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
