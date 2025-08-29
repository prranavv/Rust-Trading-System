use std::sync::{Arc, Mutex};

use  axum::{
    extract::State, http::StatusCode, Json
};
use trading_engine::TradingEngine;

use crate::types::market_order::{
    MarketOrderRequest,
    MarketOrderResponse
};

pub async fn create_market_order(
    State(state):State<Arc<Mutex<TradingEngine>>>,
    Json(payload):Json<MarketOrderRequest>,
)->(StatusCode,Json<MarketOrderResponse>){
    let mut trading_engine = state.lock().unwrap();
    let result = trading_engine.add_market_order_into_market(payload.trading_pair, payload.order);
    
    match result{
        Ok(r)=>{
            let response =MarketOrderResponse::new(Some(r), None);
            return (StatusCode::OK,Json(response))
        },
        Err(e)=>{
            let  response = MarketOrderResponse::new(None, Some(e));
            return (StatusCode::BAD_REQUEST,Json(response))
        }

    }

}