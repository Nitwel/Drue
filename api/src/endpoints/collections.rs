use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;

use crate::{
    controllers::{
        collections::{Collection, CollectionsController},
        controller::Controller,
    },
    AppState,
};

pub async fn get_all_collections(State(state): State<Arc<AppState>>) -> Json<Vec<Collection>> {
    let controller = CollectionsController::new(&state.pool);

    let words = controller.get_all().await.unwrap();

    Json(words)
}

pub async fn get_one_collection(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Json<Collection> {
    let controller = CollectionsController::new(&state.pool);

    let word = controller.get_one(id).await.unwrap();

    Json(word)
}

pub async fn post_collection(
    State(state): State<Arc<AppState>>,
    Json(mut word): Json<Value>,
) -> Json<Collection> {
    let controller = CollectionsController::new(&state.pool);

    let id = controller.create(word).await.unwrap();

    let word = controller.get_one(id).await.unwrap();

    Json(word)
}

pub async fn put_collection(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(mut word): Json<Value>,
) -> Json<Collection> {
    let controller = CollectionsController::new(&state.pool);

    controller.update(id, word).await.unwrap();

    let word = controller.get_one(id).await.unwrap();

    Json(word)
}

pub async fn delete_collection(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<()>, StatusCode> {
    let controller = CollectionsController::new(&state.pool);

    controller.delete(id).await.unwrap();

    Ok(Json(()))
}
