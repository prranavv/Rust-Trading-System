use std::sync::{Arc, Mutex};

use  axum::{
    extract::State, http::StatusCode, Json
};
use trading_engine::TradingEngine;

use crate::types::depth::{
    MarketDepthRequest,
    MarketDepthResponse
};

pub async fn get_market_depth(
    State(state):State<Arc<Mutex<TradingEngine>>>,
    Json(payload):Json<MarketDepthRequest>,
)->(StatusCode,Json<MarketDepthResponse>){
    let mut trading_engine = state.lock().unwrap();
    let result =trading_engine.get_market_depth(payload.trading_pair);
    match result{
        Ok(r)=>{
            let response = MarketDepthResponse::new(Some(r),None);
            return (StatusCode::OK,Json(response))
        },
        Err(r)=>{
            let response = MarketDepthResponse::new(None,Some(r));
            return (StatusCode::BAD_REQUEST,Json(response))
        }
    }
}