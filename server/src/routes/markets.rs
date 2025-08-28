use std::sync::{Arc, Mutex};

use  axum::{
    extract::State, http::StatusCode, Json
};
use trading_engine::TradingEngine;

use crate::types::markets::{
    CreateMarketRequest,
    CreateMarketResponse,
    GetMarketsResponse
};

pub async fn create_market(
    State(state):State<Arc<Mutex<TradingEngine>>>,
    Json(payload):Json<CreateMarketRequest>,
)->(StatusCode,Json<CreateMarketResponse>){
    let mut trading_engine = state.lock().unwrap();
    match trading_engine.create_market(payload.trading_pair.clone()){
        Ok(_)=>{(StatusCode::CREATED,Json(CreateMarketResponse::new(true, payload.trading_pair)))},
        Err(_)=>{(StatusCode::BAD_REQUEST,Json(CreateMarketResponse::new(false, payload.trading_pair)))}
    }
}

pub async fn get_markets(
    State(state):State<Arc<Mutex<TradingEngine>>>
)->(StatusCode,Json<GetMarketsResponse>){
    let trading_engine=state.lock().unwrap();
    let markets = trading_engine.get_markets();
    let response =GetMarketsResponse::new(markets);
    return (StatusCode::OK,Json(response))
}