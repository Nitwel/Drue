use std::{env, sync::Arc};

use dotenv::dotenv;
use reqwest::{header, Client};
use routes::create_router;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

mod controllers;
mod endpoints;
mod routes;
mod util;

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
    http_client: Client,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", env::var("OPENAI_API_KEY").unwrap()))
            .unwrap(),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let client = Client::builder().default_headers(headers).build().unwrap();

    let shared_state = Arc::new(AppState {
        pool,
        http_client: client,
    });

    let app = create_router(shared_state);

    let listener = TcpListener::bind(&env::var("URL").unwrap()).await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
