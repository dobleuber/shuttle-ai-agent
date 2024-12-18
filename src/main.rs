use std::env;

use crate::errors::ApiError;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

mod agent;
mod state;
use agent::Agent;
use shuttle_runtime::SecretStore;
use state::AppState;

mod errors;

async fn hello_world() -> &'static str {
    "Hola mundo!"
}

#[derive(Deserialize, Serialize)]
pub struct Prompt {
    q: String,
}

#[axum::debug_handler]
async fn prompt(
    State(state): State<AppState>,
    Json(prompt): Json<Prompt>,
) -> Result<Json<String>, ApiError> {
    let res = state.researcher.prepare_data(&prompt.q).await?;
    let res = state.researcher.prompt(&prompt.q, res).await?;
    let res = state.writer.prompt(&prompt.q, res).await?;

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
