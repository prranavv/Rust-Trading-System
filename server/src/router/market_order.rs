use std::sync::{Arc, Mutex};

use axum::{routing::{post}, Router};
use trading_engine::TradingEngine;

use crate::routes::market_order::{
    create_market_order
};

pub fn market_order_router(state:Arc<Mutex<TradingEngine>>)->Router{
    Router::new()
        .route("/api/v1/market-order", post(create_market_order))
        .with_state(state)
}