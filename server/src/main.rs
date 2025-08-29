use std::sync::{Arc, Mutex};

mod routes;
mod router;
mod types;

use router::init_router;

use trading_engine::TradingEngine;

#[tokio::main]
async fn main() {
    let trading_engine = Arc::new(Mutex::new(TradingEngine::new()));
    let app = init_router(trading_engine);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}