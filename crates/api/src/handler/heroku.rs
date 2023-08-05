use crate::AppState;

use axum::extract::State;
use axum::{routing, Router};
use hyper::StatusCode;
use reqwest::get;
use std::fs::File;
use std::io::copy;
use std::process::Command;

fn fetch_latest_s3_backup_url(app_name: &str) -> Result<String, anyhow::Error> {
    let output = Command::new("heroku")
        .arg("pg:backups:url")
        .arg("--app")
        .arg(app_name)
        .output()?;
    let download_url = String::from_utf8(output.stdout)?;
    let trimed_url = download_url.trim().to_string();
    println!("trimed_url: {}", trimed_url);
    Ok(trimed_url)
}

async fn download_backup_file(url: &str) -> Result<(), anyhow::Error> {
    let response = get(url).await?;
    let bytes = response.bytes().await?;
    let mut out = File::create("backup.dump")?;
    copy(&mut bytes.as_ref(), &mut out)?;
    Ok(())
}

async fn handler(
    State(AppState {
        heroku_app_name, ..
    }): State<AppState>,
) -> Result<(), StatusCode> {
    let download_url = fetch_latest_s3_backup_url(&heroku_app_name).or_else(|e| match e {
        anyhow::Error { .. } => Err(StatusCode::INTERNAL_SERVER_ERROR),
    })?;
    download_backup_file(&download_url)
        .await
        .or_else(|e| match e {
            anyhow::Error { .. } => Err(StatusCode::INTERNAL_SERVER_ERROR),
        })
}

pub fn route() -> Router<AppState> {
    Router::new().route("/backup", routing::get(handler))
}
