use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{self, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config = envy::from_env::<Config>().unwrap();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let rsx = Arc::new(
        resources::JsonFilesData::new(&config.resource_directory)
            .await
            .unwrap(),
    );

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/startupz", get(startupz))
        .route("/resources/:name", get(get_resource))
        .with_state(rsx);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(config.listen).await.unwrap();
    axum::Server::bind(&(config.listen).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
fn healthz() -> String {
    "OK".to_owned()
}

fn readyz() -> String {
    "OK".to_owned()
}

fn startupz() -> String {
    "OK".to_owned()
}

async fn get_resource(
    Path(name): Path<String>,
    State(rsx): State<Arc<resources::JsonFilesData>>,
) -> impl IntoResponse {
    match rsx.get_by_name(&name).await {
        Some(resource) => (StatusCode::OK, Json(Some(resource))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}
