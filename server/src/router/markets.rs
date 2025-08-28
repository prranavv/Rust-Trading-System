use axum::{routing::{get, post}, Router};

use crate::routes::markets::{
    create_market,
    get_markets
};

pub fn markets_router(state:std::sync::Arc<std::sync::Mutex<trading_engine::TradingEngine>>)->Router{
    Router::new()
        .route("/api/v1/create-market", post(create_market))
        .route("/api/v1/get-market", get(get_markets))
        .with_state(state)
}