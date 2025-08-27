use std::sync::{Arc, Mutex};

use  axum::{
    extract::State, http::StatusCode, Json
};
use trading_engine::TradingEngine;

use crate::types::markets::{
    CreateMarketRequest,
    CreateMarketResponse
};

pub async fn create_market(
        State(state):State<Arc<Mutex<TradingEngine>>>,
        Json(payload):Json<CreateMarketRequest>,
    )->(StatusCode,Json<CreateMarketResponse>){
    let mut trading_engine = state.lock().unwrap();
    let response = match trading_engine.create_market(payload.trading_pair.clone()){
        Ok(_)=>CreateMarketResponse::new(true, payload.trading_pair),
        Err(_)=>CreateMarketResponse::new(false, payload.trading_pair)
    };
    (StatusCode::CREATED,Json(response))
}