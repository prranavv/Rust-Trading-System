use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};

mod routes;
mod types;

use trading_engine::TradingEngine;
use routes::markets::create_market;

#[tokio::main]
async fn main() {
    let trading_engine = Arc::new(Mutex::new(TradingEngine::new()));
    let app = Router::new()
                        .route("/", get(|| async { "Hello, World!" }))
                        .route("/create-market", post(create_market))
                        .with_state(trading_engine);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}