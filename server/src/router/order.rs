use std::sync::{Arc, Mutex};

use axum::{routing::{delete}, Router};
use trading_engine::TradingEngine;

use crate::routes::order::{
    delete_order
};

pub fn order_router(state:Arc<Mutex<TradingEngine>>)->Router{
    Router::new()
        .route("/api/v1/limit-order", delete(delete_order))
        .with_state(state)
}