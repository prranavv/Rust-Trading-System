use std::sync::{Arc, Mutex};

use axum::{routing::{get}, Router};
use trading_engine::TradingEngine;

use crate::routes::depth::{
    get_market_depth,
    get_market_mid_price
};

pub fn market_depth_router(state:Arc<Mutex<TradingEngine>>)->Router{
    Router::new()
        .route("/api/v1/depth", get(get_market_depth))
        .route("/api/v1/mid-price", get(get_market_mid_price))
        .with_state(state)
}