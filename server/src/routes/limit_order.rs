use std::sync::{Arc, Mutex};

use  axum::{
    extract::State, http::StatusCode, Json
};
use trading_engine::TradingEngine;

use crate::types::limit_order::{
    LimitOrderRequest,
    LimitOrderResponse
};

pub async fn create_limit_order(
    State(state):State<Arc<Mutex<TradingEngine>>>,
    Json(payload):Json<LimitOrderRequest>,
)->(StatusCode,Json<LimitOrderResponse>){
    let mut trading_engine = state.lock().unwrap();
    let result = trading_engine.add_limit_order_into_market(payload.trading_pair, payload.order);
    match result{
        Ok(o)=>{
            let response = LimitOrderResponse::new(Some(o), None);
            return (StatusCode::CREATED,Json(response))
        }
        Err(e)=>{
            let response = LimitOrderResponse::new(None, Some(e));
            return (StatusCode::BAD_REQUEST,Json(response))
        }
    }
}