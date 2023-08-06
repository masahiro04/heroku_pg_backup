mod handler;
use anyhow::Context;
use axum::Router;
use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use dotenv::dotenv;

#[derive(Clone, Debug)]
pub struct AppState {
    pub heroku_app_name: String,
    pub heroku_api_key: String,
}

fn router(shared_state: AppState) -> Router {
    Router::new()
        .merge(handler::heroku::route())
        .with_state(shared_state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    let heroku_api_key = env::var("HEROKU_API_KEY").context("HEROKU_API_KEY not found")?;
    env::set_var("HEROKU_API_KEY", &heroku_api_key);
    let heroku_app_name = env::var("HEROKU_APP_NAME").context("HEROKU_APP_NAME not found")?;
    let shared_state = AppState {
        heroku_api_key,
        heroku_app_name,
    };
    let app = router(shared_state);

    let port_as_string = env::var("PORT").or_else(|e| match e {
        env::VarError::NotPresent => Ok("3000".to_owned()),
        env::VarError::NotUnicode(_) => anyhow::bail!("PORT is not unicode"),
    })?;
    let port = u16::from_str(port_as_string.as_str()).context("PORT range is (0..=65535)")?;
    let socket_addr = SocketAddr::new(
        IpAddr::from_str("0.0.0.0").expect("0.0.0.0 is valid host"),
        port,
    );

    Ok(axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?)
}
