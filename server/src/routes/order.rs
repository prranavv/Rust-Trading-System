use std::sync::{Arc, Mutex};

use  axum::{
    extract::State, http::StatusCode, Json
};
use trading_engine::TradingEngine;

use crate::types::order::{
    DeleteOrderRequest,
    DeleteOrderResponse, 
    ModifyOrderRequest,
    ModifyOrderResponse,
    GetOrderRequest,
    GetOrderResponse
};

pub async fn delete_order(
    State(state):State<Arc<Mutex<TradingEngine>>>,
    Json(payload):Json<DeleteOrderRequest>,
)->(StatusCode,Json<DeleteOrderResponse>){
    let mut trading_engine = state.lock().unwrap();
    let result =trading_engine.delete_order_for_market(payload.trading_pair, payload.order_id);
    match result{
        Ok(r)=>{
            match r{
                Ok(res)=>{
                    let response = DeleteOrderResponse::new(Some(res),None,None);
                    return (StatusCode::OK,Json(response))
                },
                Err(e)=>{
                    let response = DeleteOrderResponse::new(None, None, Some(e.error));
                    return (StatusCode::BAD_REQUEST,Json(response))
                }
            }
        },
        Err(r)=>{
            let response = DeleteOrderResponse::new(None,Some(r),None);
            return (StatusCode::BAD_REQUEST,Json(response))
        }
    }
}

pub async fn modify_order(
    State(state):State<Arc<Mutex<TradingEngine>>>,
    Json(payload):Json<ModifyOrderRequest>,
)->(StatusCode,Json<ModifyOrderResponse>){
    let mut trading_engine = state.lock().unwrap();
    let result = trading_engine.modify_order_for_market(payload.trading_pair, payload.order_request);
    match result{
        Ok(r)=>{
            match r{
                Ok(res)=>{
                    let response = ModifyOrderResponse::new(Some(res),None,None);
                    return (StatusCode::OK,Json(response))

                },
                Err(res)=>{
                    let response = ModifyOrderResponse::new(None,None, Some(res.error));
                    return (StatusCode::BAD_REQUEST,Json(response))
                }
            }
        },
        Err(r)=>{
            let response = ModifyOrderResponse::new(None,Some(r), None);
            return (StatusCode::BAD_REQUEST,Json(response))
        }
    }
}

pub async fn get_order(
    State(state):State<Arc<Mutex<TradingEngine>>>,
    Json(payload):Json<GetOrderRequest>
)->(StatusCode,Json<GetOrderResponse>){
    let mut trading_engine = state.lock().unwrap();
    let result = trading_engine.get_order_by_id_for_market(payload.trading_pair, payload.order_id);
    match result{
        Ok(r)=>{
            match r{
                Ok(res)=>{
                    let response = GetOrderResponse::new(Some(res), None, None);
                    return (StatusCode::OK,Json(response))
                }
                Err(res)=>{
                    let response = GetOrderResponse::new(None, None, Some(res.error));
                    return (StatusCode::BAD_REQUEST,Json(response))
                }
            }
        },
        Err(r)=>{
            let response = GetOrderResponse::new(None, Some(r), None);
            return (StatusCode::BAD_REQUEST,Json(response))
        }
    }
}