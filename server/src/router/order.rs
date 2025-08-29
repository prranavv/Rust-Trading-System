use std::sync::{Arc, Mutex};

use axum::{routing::{delete, post}, Router};
use trading_engine::TradingEngine;

use crate::routes::order::{
    delete_order,
    modify_order
};

pub fn order_router(state:Arc<Mutex<TradingEngine>>)->Router{
    Router::new()
        .route("/api/v1/delete-order", delete(delete_order))
        .route("/api/v1/modify-order", post(modify_order))
        .with_state(state)
}