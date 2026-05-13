use axum::{
    extract::{DefaultBodyLimit, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use base64::Engine;
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

    // Decode base64 to check size
    let raw_bytes = match base64::engine::general_purpose::STANDARD.decode(&body.content) {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": format!("Invalid base64: {}", e)})),
            )
                .into_response();
        }
    };

    // Extract filename from path
    let file_name = query.path.split('/').last().unwrap_or(&query.path);

    // Route based on size
    if raw_bytes.len() <= MAX_CONTENTS_SIZE {
        // Small file — use GitHub Contents API
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
    } else {
        // Large file — upload to GitHub Releases
        let file_size = raw_bytes.len();
        let release_tag = env::var("RELEASE_TAG").unwrap_or_else(|_| "mod-files".to_string());

        let release = match ensure_release(&state, &release_tag).await {
            Ok(r) => r,
            Err(e) => return e.into_response(),
        };

        let download_url = match upload_asset(&state, &release, file_name, raw_bytes).await {
            Ok(u) => u,
            Err(e) => return e.into_response(),
        };

        (StatusCode::OK, Json(serde_json::json!({
            "ok": true,
            "download_url": download_url,
            "size": file_size,
        })))
        .into_response()
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

    // Try Contents API first (for files in git)
    if let Some(sha) = get_file_sha(&state, &query.path).await {
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

        return match resp {
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
        };
    }

    // Not in git — check if it's a release asset
    let file_name = query.path.split('/').last().unwrap_or(&query.path);
    let release_tag = env::var("RELEASE_TAG").unwrap_or_else(|_| "mod-files".to_string());

    // Get the release
    let release_url = format!(
        "https://api.github.com/repos/{}/releases/tags/{}",
        state.repo_path, release_tag
    );
    let release_resp = state
        .client
        .get(&release_url)
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .send()
        .await;

    let release = match release_resp {
        Ok(r) if r.status().is_success() => r.json::<serde_json::Value>().await.unwrap_or_default(),
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "File not found in git or releases"})),
            )
                .into_response();
        }
    };

    // Find asset by name
    let assets = release["assets"].as_array().cloned().unwrap_or_default();
    let asset = assets.iter().find(|a| {
        a["name"].as_str().unwrap_or("") == file_name
    });

    let asset_id = match asset {
        Some(a) => a["id"].as_i64().unwrap_or(0),
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "File not found in release assets"})),
            )
                .into_response();
        }
    };

    // Delete asset
    let delete_url = format!(
        "https://api.github.com/repos/{}/releases/assets/{}",
        state.repo_path, asset_id
    );
    let del_resp = state
        .client
        .delete(&delete_url)
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .send()
        .await;

    match del_resp {
        Ok(r) if r.status().is_success() || r.status() == StatusCode::NO_CONTENT => {
            // Remove from manifest
            let manifest_result = remove_from_manifest(&state, &query.path).await;
            if let Err(e) = manifest_result {
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "ok": true,
                        "warning": "Asset deleted but manifest update failed",
                        "error": e.1 .0.error,
                    })),
                )
                    .into_response();
            }
            (StatusCode::OK, Json(serde_json::json!({"ok": true})))
                .into_response()
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

/// Remove a file entry from the manifest
async fn remove_from_manifest(
    state: &AppState,
    path: &str,
) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    // Fetch current manifest
    let resp = state
        .client
        .get(gh_url("manifest.json"))
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse { error: e.to_string() }),
            )
        })?;

    if !resp.status().is_success() {
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: format!("Failed to fetch manifest: {}", resp.status()),
            }),
        ));
    }

    let gh_data: serde_json::Value = resp.json().await.map_err(|e| {
        (
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse { error: e.to_string() }),
        )
    })?;

    let sha = gh_data["sha"].as_str().unwrap_or("");
    let raw_content = gh_data["content"].as_str().unwrap_or("");
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(raw_content)
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse {
                    error: format!("Failed to decode manifest: {}", e),
                }),
            )
        })?;

    let mut manifest: serde_json::Value = serde_json::from_slice(&decoded).map_err(|e| {
        (
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: format!("Failed to parse manifest: {}", e),
            }),
        )
    })?;

    // Remove the file entry
    if let Some(files) = manifest["files"].as_array_mut() {
        files.retain(|f| f["path"] != path);
    }

    let manifest_json = serde_json::to_string(&manifest).map_err(|e| {
        (
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: format!("Failed to stringify manifest: {}", e),
            }),
        )
    })?;

    let new_content = base64::engine::general_purpose::STANDARD.encode(&manifest_json);

    let put_payload = serde_json::json!({
        "message": format!("Remove {} from manifest", path),
        "content": new_content,
        "sha": sha,
    });

    let put_resp = state
        .client
        .put(gh_url("manifest.json"))
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .json(&put_payload)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse { error: e.to_string() }),
            )
        })?;

    if !put_resp.status().is_success() {
        let status = put_resp.status();
        let text = put_resp.text().await.unwrap_or_default();
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: format!("Failed to save manifest ({}): {}", status, text),
            }),
        ));
    }

    Ok(())
}

const MAX_CONTENTS_SIZE: usize = 1_000_000; // 1MB — GitHub Contents API limit

/// Get or create a release with the given tag, return its id
async fn ensure_release(state: &AppState, tag: &str) -> Result<serde_json::Value, (StatusCode, Json<ErrorResponse>)> {
    // Try to get existing release
    let url = format!(
        "https://api.github.com/repos/{}/releases/tags/{}",
        state.repo_path, tag
    );
    let resp = state
        .client
        .get(&url)
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse { error: e.to_string() }),
            )
        })?;

    if resp.status().is_success() {
        return resp.json().await.map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse { error: e.to_string() }),
            )
        });
    }

    // Create new release
    let create_url = format!("https://api.github.com/repos/{}/releases", state.repo_path);
    let payload = serde_json::json!({
        "tag_name": tag,
        "name": tag,
        "body": "Auto-uploaded mod files",
        "draft": false,
        "prerelease": false,
    });

    let create_resp = state
        .client
        .post(&create_url)
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse { error: e.to_string() }),
            )
        })?;

    if create_resp.status().is_success() {
        create_resp.json().await.map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse { error: e.to_string() }),
            )
        })
    } else {
        let status = create_resp.status();
        let text = create_resp.text().await.unwrap_or_default();
        Err((
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: format!("Failed to create release ({}): {}", status, text),
            }),
        ))
    }
}

/// Upload a file as an asset to a release, return the download URL
async fn upload_asset(
    state: &AppState,
    release: &serde_json::Value,
    file_name: &str,
    data: Vec<u8>,
) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    let upload_url_template = release["upload_url"]
        .as_str()
        .ok_or_else(|| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse {
                    error: "No upload_url in release".into(),
                }),
            )
        })?;

    // Strip the {?name,label} suffix from the template
    let upload_url = upload_url_template
        .split('{')
        .next()
        .unwrap_or(upload_url_template);

    let asset_url = format!("{}?name={}", upload_url, urlencode(file_name));

    let resp = state
        .client
        .post(&asset_url)
        .header("Authorization", format!("Bearer {}", state.github_token))
        .header("User-Agent", "kd-backend/0.1")
        .header("Content-Type", "application/octet-stream")
        .body(data)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse { error: e.to_string() }),
            )
        })?;

    if resp.status().is_success() {
        let asset: serde_json::Value = resp.json().await.map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(ErrorResponse { error: e.to_string() }),
            )
        })?;
        let browser_url = asset["browser_download_url"]
            .as_str()
            .unwrap_or("")
            .to_string();
        Ok(browser_url)
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err((
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse {
                error: format!("Failed to upload asset ({}): {}", status, text),
            }),
        ))
    }
}

fn urlencode(s: &str) -> String {
    let mut result = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(b as char);
            }
            b' ' => result.push_str("%20"),
            _ => result.push_str(&format!("%{:02X}", b)),
        }
    }
    result
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
        .layer(DefaultBodyLimit::max(150 * 1024 * 1024))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
