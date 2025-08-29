use std::sync::{Arc, Mutex};

use axum::{routing::{post}, Router};
use trading_engine::TradingEngine;

use crate::routes::limit_order::{
    create_limit_order
};

pub fn limit_order_router(state:Arc<Mutex<TradingEngine>>)->Router{
    Router::new()
        .route("/api/v1/limit-order", post(create_limit_order))
        .with_state(state)
}