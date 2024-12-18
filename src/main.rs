use std::env;

use crate::errors::ApiError;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

mod agent;
mod errors;
mod pipeline;
mod state;
use agent::{Agent, LinkedInAgent, Researcher, TwitterAgent, Writer};
use pipeline::{ContentPipeline, Pipeline};
use shuttle_runtime::SecretStore;
use state::AppState;

async fn hello_world() -> &'static str {
    "Hola mundo!"
}

#[derive(Deserialize, Serialize)]
pub struct Prompt {
    q: String,
}

#[axum::debug_handler]
async fn prompt(
    State(_state): State<AppState>,
    Json(prompt): Json<Prompt>,
) -> Result<Json<String>, ApiError> {
    let agents: Vec<Box<dyn Agent>> = vec![
        Box::new(Researcher::new()),
        // Box::new(Writer::new()),
        Box::new(TwitterAgent::new()),
        Box::new(LinkedInAgent::new()),
    ];
    let pipeline = ContentPipeline::new(agents);
    let res = pipeline.run_pipeline(&prompt.q).await?;

    Ok(Json(res))
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    secrets.into_iter().for_each(|x| env::set_var(x.0, x.1));
    let state = AppState::new();

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/prompt", post(prompt))
        .with_state(state);

    Ok(router.into())
}
