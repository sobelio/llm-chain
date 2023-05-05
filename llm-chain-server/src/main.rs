use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse},
    routing::{get},
    Json, Router,
};
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
        .route("/resources/:name", get(get_resource))
        .with_state(rsx);

    axum::Server::bind(&(&config.listen).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
