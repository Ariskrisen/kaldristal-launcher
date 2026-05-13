use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;
use tracing::info;

#[derive(Clone)]
struct AppState {
    client: Client,
    github_token: String,
    repo_path: String,
    auth_token: Option<String>,
}

#[derive(Deserialize)]
struct FileQuery {
    path: String,
}

#[derive(Deserialize)]
struct FileUpload {
    content: String,
    message: String,
}

#[derive(Deserialize)]
struct ManifestUpdate {
    content: String,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn gh_url(path: &str) -> String {
    format!("https://api.github.com/repos/{}/contents/{}", get_repo(), path)
}

fn get_repo() -> String {
    env::var("REPO_PATH").expect("REPO_PATH not set")
}

fn check_auth(state: &AppState, headers: &axum::http::HeaderMap) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    if let Some(ref expected) = state.auth_token {
        let provided = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        // strip "Bearer " prefix if present
        let token = provided.strip_prefix("Bearer ").unwrap_or(provided);
        if token != expected {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse { error: "Invalid auth token".into() }),
            ));
        }
    }
    Ok(())
}

async fn handle_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let resp = state
        .client
        .get(format!("https://api.github.com/repos/{}", state.repo_path))
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            let data: serde_json::Value = r.json().await.unwrap_or_default();
            Json(serde_json::json!({
                "ok": true,
                "repo": data["full_name"],
                "visibility": data["visibility"],
            }))
        }
        Ok(r) => {
            let status = r.status();
            Json(serde_json::json!({
                "ok": false,
                "error": format!("GitHub returned {}", status),
            }))
        }
        Err(e) => Json(serde_json::json!({
            "ok": false,
            "error": e.to_string(),
        })),
    }
}

async fn handle_get_manifest(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    if let Err(e) = check_auth(&state, &headers) {
        return e.into_response();
    }

    let resp = state
        .client
        .get(gh_url("manifest.json"))
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            match r.json::<serde_json::Value>().await {
                Ok(data) => {
                    (StatusCode::OK, Json(data)).into_response()
                }
                Err(e) => {
                    (
                        StatusCode::BAD_GATEWAY,
                        Json(serde_json::json!({"error": e.to_string()})),
                    )
                        .into_response()
                }
            }
        }
        Ok(r) if r.status() == StatusCode::NOT_FOUND => {
            // No manifest yet — return a default
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "content": null,
                    "exists": false,
                })),
            )
                .into_response()
        }
        Ok(r) => {
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("GitHub: {}", r.status())})),
            )
                .into_response()
        }
        Err(e) => {
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response()
        }
    }
}

async fn handle_put_manifest(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Json(body): Json<ManifestUpdate>,
) -> impl IntoResponse {
    if let Err(e) = check_auth(&state, &headers) {
        return e.into_response();
    }

    // First, get the current file SHA (if exists)
    let sha = get_file_sha(&state, "manifest.json").await;

    let mut payload = serde_json::json!({
        "message": body.message,
        "content": body.content,
    });
    if let Some(s) = sha {
        payload["sha"] = serde_json::json!(s);
    };

    let resp = state
        .client
        .put(gh_url("manifest.json"))
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .json(&payload)
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            match r.json::<serde_json::Value>().await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => (
                    StatusCode::BAD_GATEWAY,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
                    .into_response(),
            }
        }
        Ok(r) => {
            let status = r.status();
            let text = r.text().await.unwrap_or_default();
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("GitHub {}: {}", status, text)})),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn handle_get_file(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<FileQuery>,
) -> impl IntoResponse {
    if let Err(e) = check_auth(&state, &headers) {
        return e.into_response();
    }

    let resp = state
        .client
        .get(gh_url(&query.path))
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            match r.json::<serde_json::Value>().await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => (
                    StatusCode::BAD_GATEWAY,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
                    .into_response(),
            }
        }
        Ok(r) => {
            let status = r.status();
            (
                StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
                Json(serde_json::json!({"error": format!("GitHub: {}", status)})),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn handle_put_file(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<FileQuery>,
    Json(body): Json<FileUpload>,
) -> impl IntoResponse {
    if let Err(e) = check_auth(&state, &headers) {
        return e.into_response();
    }

    // Get SHA of existing file (if any)
    let sha = get_file_sha(&state, &query.path).await;

    let mut payload = serde_json::json!({
        "message": body.message,
        "content": body.content,
    });
    if let Some(s) = sha {
        payload["sha"] = serde_json::json!(s);
    };

    let resp = state
        .client
        .put(gh_url(&query.path))
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .json(&payload)
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            match r.json::<serde_json::Value>().await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => (
                    StatusCode::BAD_GATEWAY,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
                    .into_response(),
            }
        }
        Ok(r) => {
            let status = r.status();
            let text = r.text().await.unwrap_or_default();
            (
                StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
                Json(serde_json::json!({"error": format!("GitHub {}: {}", status, text)})),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn handle_delete_file(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<FileQuery>,
) -> impl IntoResponse {
    if let Err(e) = check_auth(&state, &headers) {
        return e.into_response();
    }

    // Get SHA first
    let sha = match get_file_sha(&state, &query.path).await {
        Some(s) => s,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "File not found"})),
            )
                .into_response();
        }
    };

    let payload = serde_json::json!({
        "message": format!("Delete {}", query.path),
        "sha": sha,
    });

    let resp = state
        .client
        .delete(gh_url(&query.path))
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .json(&payload)
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            match r.json::<serde_json::Value>().await {
                Ok(data) => (StatusCode::OK, Json(data)).into_response(),
                Err(e) => (
                    StatusCode::BAD_GATEWAY,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
                    .into_response(),
            }
        }
        Ok(r) => {
            let status = r.status();
            let text = r.text().await.unwrap_or_default();
            (
                StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
                Json(serde_json::json!({"error": format!("GitHub {}: {}", status, text)})),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn get_file_sha(state: &AppState, path: &str) -> Option<String> {
    let resp = state
        .client
        .get(gh_url(path))
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let data: serde_json::Value = resp.json().await.ok()?;
    data["sha"].as_str().map(|s| s.to_string())
}

async fn handle_admin() -> Html<&'static str> {
    Html(include_str!("admin.html"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let repo_path = env::var("REPO_PATH").expect("REPO_PATH not set");
    let bind = env::var("BIND").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let auth_token = env::var("AUTH_TOKEN").ok();

    info!("Starting kd-backend on {}", bind);
    info!("Repo: {}", repo_path);

    let client = Client::builder()
        .user_agent("kd-backend/0.1")
        .build()
        .expect("Failed to build HTTP client");

    let state = Arc::new(AppState {
        client,
        github_token,
        repo_path,
        auth_token,
    });

    let app = Router::new()
        .route("/api/check", get(handle_check))
        .route("/api/manifest", get(handle_get_manifest).put(handle_put_manifest))
        .route("/api/file", get(handle_get_file).put(handle_put_file).delete(handle_delete_file))
        .route("/", get(handle_admin))
        .route("/admin", get(handle_admin))
        .route("/admin/", get(handle_admin))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
