use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::{
    endpoints::{
        collections::{
            delete_collection, get_all_collections, get_one_collection, post_collection,
            put_collection,
        },
        words::{delete_word, generate_question, get_all_words, get_one_word, post_word, put_word},
    },
    AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    let app: Router = Router::new()
        .nest_service("/", ServeDir::new("../app/dist"))
        .route("/api/words", get(get_all_words).post(post_word))
        .route(
            "/api/words/:id",
            get(get_one_word).put(put_word).delete(delete_word),
        )
        .route("/api/question/:id", get(generate_question))
        .route(
            "/api/collections",
            get(get_all_collections).post(post_collection),
        )
        .route(
            "/api/collections/:id",
            get(get_one_collection)
                .put(put_collection)
                .delete(delete_collection),
        )
        .with_state(state);

    app
}
