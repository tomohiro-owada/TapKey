use axum::{
    extract::Json,
    http::{header, Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::config::{AppConfig, ButtonConfig};
use crate::keyboard;

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
            message: "認証成功".to_string(),
        })
    } else {
        Json(AuthResponse {
            success: false,
            message: "PINが正しくありません".to_string(),
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
                message: "PINが正しくありません".to_string(),
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
            message: "PINが正しくありません".to_string(),
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
                    message: "アクション実行完了".to_string(),
                }),
                Err(e) => Json(ActionResponse {
                    success: false,
                    message: format!("アクション実行失敗: {}", e),
                }),
            }
        }
        None => Json(ActionResponse {
            success: false,
            message: format!("ボタンが見つかりません: {}", req.button_id),
        }),
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

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    Router::new()
        .route("/", get(serve_ui))
        .route("/style.css", get(serve_css))
        .route("/app.js", get(serve_js))
        .route("/api/auth", post(auth))
        .route("/api/config", post(get_config))
        .route("/api/action", post(execute_action))
        .layer(cors)
}

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = create_router();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!("HTTP Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
