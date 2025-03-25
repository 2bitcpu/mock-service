use axum::{
    BoxError, Router,
    body::Body,
    extract::{Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{any, post},
};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static MOCK_DATA_DIR: LazyLock<String> =
    LazyLock::new(|| std::env::var("MOCK_DATA_DIR").unwrap_or_else(|_| "./service".to_string()));

static MOCK_HOST: LazyLock<String> =
    LazyLock::new(|| std::env::var("MOCK_HOST").unwrap_or_else(|_| "0.0.0.0:3000".to_string()));

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let app = Router::new()
        .route("/manage-mock", post(save_handler))
        .route("/service/{*path}", any(get_handler));

    let listener = tokio::net::TcpListener::bind(&*MOCK_HOST).await?;
    println!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub end_point: String,
    pub status_code: u16,
    pub content_type: String,
    pub content: Option<String>,
}

pub async fn save_handler(Json(meta): Json<MetaData>) -> impl IntoResponse {
    match save_meta(meta).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "success"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": e.to_string()})),
        ),
    }
}

pub async fn get_handler(Path(path): Path<String>) -> impl IntoResponse {
    let meta_data = match get_meta(path).await {
        Ok(meta) => meta,
        Err(e) => {
            return build_response(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "application/json".to_string(),
                Some(
                    serde_json::to_string(&serde_json::json!({"message": e.to_string()})).unwrap(),
                ),
            );
        }
    };

    build_response(
        meta_data.status_code,
        meta_data.content_type,
        meta_data.content,
    )
}

fn build_response(
    status_code: u16,
    content_type: String,
    content: Option<String>,
) -> impl IntoResponse {
    let builder = Response::builder();
    builder
        .status(status_code)
        .header("Content-Type", content_type)
        .body(Body::from(content.unwrap_or_default()))
        .unwrap()
        .into_response()
}

pub async fn save_meta(meta: MetaData) -> Result<(), BoxError> {
    let base_dir = std::path::Path::new(&*MOCK_DATA_DIR);
    if !base_dir.is_dir() {
        return Err("MOCK_DATA_DIR is not a directory".into());
    }

    let meta_dir = base_dir.join(&meta.end_point);
    tokio::fs::create_dir_all(&meta_dir).await?;

    let meta_path = meta_dir.join("meta.json");
    tokio::fs::write(&meta_path, serde_json::to_string(&meta)?).await?;

    Ok(())
}

pub async fn get_meta(end_point: String) -> Result<MetaData, BoxError> {
    let base_dir = std::path::Path::new(&*MOCK_DATA_DIR);
    if !base_dir.is_dir() {
        return Err("MOCK_DATA_DIR is not a directory".into());
    }

    let meta_dir = base_dir.join(&end_point);
    if !meta_dir.is_dir() {
        return Err("MOCK_DATA_DIR is not a directory".into());
    }

    let meta_path = meta_dir.join("meta.json");
    let meta_text = tokio::fs::read_to_string(&meta_path).await?;

    Ok(serde_json::from_str(&meta_text)?)
}
