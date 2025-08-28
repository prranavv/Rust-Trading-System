use std::sync::{Arc, Mutex};

use axum::Router;
use trading_engine::TradingEngine;

use crate::router::markets::markets_router;

mod markets;

pub fn init_router(state: Arc<Mutex<TradingEngine>>)->Router{
    Router::new()
        .merge(markets_router(state))
}