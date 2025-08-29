use std::sync::{Arc, Mutex};

use axum::{routing::{get}, Router};
use trading_engine::TradingEngine;

use crate::routes::depth::{
    get_market_depth
};

pub fn market_depth_router(state:Arc<Mutex<TradingEngine>>)->Router{
    Router::new()
        .route("/api/v1/limit-order", get(get_market_depth))
        .with_state(state)
}