use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use llm_chain::executor;
use llm_chain_openai::chatgpt::Executor;
use resources::JsonFilesData;
use serde;
use serde::{Deserialize, Serialize};

use std::sync::Arc;

mod resources;

#[derive(Deserialize, Debug, Serialize)]
struct Config {
    #[serde(default = "default_listen")]
    listen: String,
    resource_directory: String,
}

fn default_listen() -> String {
    "0.0.0.0:3000".to_string()
}

struct AppState {
    pub executor: Executor,
    pub resources: JsonFilesData,
}

impl AppState {
    pub async fn new(config: &Config) -> Result<AppState> {
        let executor = executor!()?;
        let resources = resources::JsonFilesData::new(&config.resource_directory)
            .await
            .unwrap();
        Ok(AppState {
            executor,
            resources,
        })
    }
    pub async fn new_arc(config: &Config) -> Result<Arc<AppState>> {
        Ok(Arc::new(Self::new(config).await?))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config = envy::from_env::<Config>().unwrap();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/resources/:name", post(trigger_resource))
        .with_state(AppState::new_arc(&config).await.unwrap());

    axum::Server::bind(&(&config.listen).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn trigger_resource(
    Path(name): Path<String>,
    State(a): State<Arc<AppState>>,
) -> impl IntoResponse {
    match a.resources.get_by_name(&name).await {
        Some(resource) => (StatusCode::OK, Json(Some(resource))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}
