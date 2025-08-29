use std::sync::{Arc, Mutex};

use axum::Router;
use trading_engine::TradingEngine;

use crate::router::markets::markets_router;
use crate::router::limit_order::limit_order_router;
use crate::router::market_order::market_order_router;

mod markets;
mod limit_order;
mod market_order;

pub fn init_router(state: Arc<Mutex<TradingEngine>>)->Router{
    Router::new()
        .merge(markets_router(state.clone()))
        .merge(limit_order_router(state.clone()))
        .merge(market_order_router(state.clone()))
}