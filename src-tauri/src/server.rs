use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Json, State,
    },
    http::{header, Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

use crate::config::{AppConfig, ButtonConfig};
use crate::keyboard;

/// WebSocket経由で送信するメッセージの種類
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    /// 設定が更新された
    ConfigUpdated,
    /// Ping（接続確認）
    Ping,
    /// Pong（接続確認応答）
    Pong,
}

/// アプリケーション状態（WebSocket broadcast用）
#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<WsMessage>,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pin: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    success: bool,
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct ActionRequest {
    button_id: String,
    pin: String,
}

#[derive(Debug, Serialize)]
pub struct ActionResponse {
    success: bool,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    grid: crate::config::GridConfig,
    buttons: Vec<ButtonConfig>,
}

/// PIN認証
async fn auth(Json(req): Json<AuthRequest>) -> Json<AuthResponse> {
    let config = AppConfig::load();

    if config.pin.is_empty() || req.pin == config.pin {
        Json(AuthResponse {
            success: true,
            message: "Authentication successful".to_string(),
        })
    } else {
        Json(AuthResponse {
            success: false,
            message: "Invalid PIN".to_string(),
        })
    }
}

/// ボタン設定を取得
async fn get_config(Json(req): Json<AuthRequest>) -> Response {
    let config = AppConfig::load();

    // PIN検証
    if !config.pin.is_empty() && req.pin != config.pin {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AuthResponse {
                success: false,
                message: "Invalid PIN".to_string(),
            }),
        )
            .into_response();
    }

    Json(ConfigResponse {
        grid: config.grid.clone(),
        buttons: config.buttons.clone(),
    })
    .into_response()
}

/// ボタンアクションを実行
async fn execute_action(Json(req): Json<ActionRequest>) -> Json<ActionResponse> {
    let config = AppConfig::load();

    // PIN検証
    if !config.pin.is_empty() && req.pin != config.pin {
        return Json(ActionResponse {
            success: false,
            message: "Invalid PIN".to_string(),
        });
    }

    // ボタンを探す
    let button = config.buttons.iter().find(|b| b.id == req.button_id);

    match button {
        Some(btn) => {
            let action = btn.action.clone();

            match keyboard::execute_action(&action) {
                Ok(()) => Json(ActionResponse {
                    success: true,
                    message: "Action executed".to_string(),
                }),
                Err(e) => Json(ActionResponse {
                    success: false,
                    message: format!("Action failed: {}", e),
                }),
            }
        }
        None => Json(ActionResponse {
            success: false,
            message: format!("Button not found: {}", req.button_id),
        }),
    }
}

/// WebSocket接続ハンドラ
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// WebSocket接続を処理
async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    // broadcast受信タスク
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // クライアントからのメッセージ受信タスク
    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // Pingに応答
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    if matches!(ws_msg, WsMessage::Ping) {
                        let _ = tx.send(WsMessage::Pong);
                    }
                }
            }
        }
    });

    // どちらかのタスクが終了したら両方終了
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}

/// スマホ用Web UI HTML
async fn serve_ui() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

/// スマホ用Web UI CSS
async fn serve_css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css")],
        include_str!("../static/style.css"),
    )
}

/// スマホ用Web UI JavaScript
async fn serve_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript")],
        include_str!("../static/app.js"),
    )
}

/// Favicon
async fn serve_favicon() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        include_bytes!("../static/favicon.png").as_slice(),
    )
}

/// Apple Touch Icon
async fn serve_apple_touch_icon() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        include_bytes!("../static/apple-touch-icon.png").as_slice(),
    )
}

/// Web App Manifest
async fn serve_manifest() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/manifest+json")],
        include_str!("../static/manifest.json"),
    )
}

/// PWA Icon 192x192
async fn serve_icon_192() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        include_bytes!("../static/icon-192.png").as_slice(),
    )
}

/// PWA Icon 512x512
async fn serve_icon_512() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/png")],
        include_bytes!("../static/icon-512.png").as_slice(),
    )
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    Router::new()
        .route("/", get(serve_ui))
        .route("/style.css", get(serve_css))
        .route("/app.js", get(serve_js))
        .route("/favicon.png", get(serve_favicon))
        .route("/apple-touch-icon.png", get(serve_apple_touch_icon))
        .route("/manifest.json", get(serve_manifest))
        .route("/icon-192.png", get(serve_icon_192))
        .route("/icon-512.png", get(serve_icon_512))
        .route("/api/auth", post(auth))
        .route("/api/config", post(get_config))
        .route("/api/action", post(execute_action))
        .route("/ws", get(ws_handler))
        .layer(cors)
        .with_state(state)
}

/// グローバルなbroadcast sender
static BROADCAST_TX: once_cell::sync::OnceCell<broadcast::Sender<WsMessage>> =
    once_cell::sync::OnceCell::new();

/// 設定更新を全クライアントに通知
pub fn notify_config_updated() {
    if let Some(tx) = BROADCAST_TX.get() {
        let _ = tx.send(WsMessage::ConfigUpdated);
    }
}

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (tx, _rx) = broadcast::channel::<WsMessage>(100);

    // グローバルにsenderを保存
    let _ = BROADCAST_TX.set(tx.clone());

    let state = Arc::new(AppState { tx });
    let app = create_router(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!("HTTP Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
