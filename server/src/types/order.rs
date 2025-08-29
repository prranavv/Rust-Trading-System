use orderbook::{CustomError, DeleteResponse,ModifyOrderRequest as OrderBookModifyOrderRequest,ModifyOrderResponse as OrderBookModifyOrderResponse, OpenOrder};
use serde::{Deserialize, Serialize};
use trading_engine::{TradingEngineError, TradingPair};



#[derive(Serialize,Deserialize)]
pub struct DeleteOrderRequest{
    pub trading_pair:TradingPair,
    pub order_id: u64
}

#[derive(Serialize,Deserialize)]
pub struct DeleteOrderResponse{
    response: Option<DeleteResponse>,
    engine_error: Option<TradingEngineError>,
    orderbook_error:Option<CustomError>
}

#[derive(Serialize,Deserialize)]
pub struct ModifyOrderRequest{
    pub trading_pair:TradingPair,
    pub order_request:OrderBookModifyOrderRequest
}

#[derive(Serialize,Deserialize)]
pub struct ModifyOrderResponse{
    response:Option<OrderBookModifyOrderResponse>,
    engine_error: Option<TradingEngineError>,
    orderbook_error:Option<CustomError>
}

#[derive(Serialize,Deserialize)]
pub struct GetOrderRequest{
    pub trading_pair:TradingPair,
    pub order_id:u64
}

#[derive(Serialize,Deserialize)]
pub struct GetOrderResponse{
    response: Option<OpenOrder>,
    engine_error: Option<TradingEngineError>,
    orderbook_error:Option<CustomError>
}

impl DeleteOrderResponse{
    pub fn new(response:Option<DeleteResponse>,engine_error:Option<TradingEngineError>,orderbook_error:Option<CustomError>)->DeleteOrderResponse{
        DeleteOrderResponse{ response,engine_error,orderbook_error }
    }
}

impl ModifyOrderResponse{
    pub fn new(response:Option<OrderBookModifyOrderResponse>,engine_error: Option<TradingEngineError>,orderbook_error:Option<CustomError>)->ModifyOrderResponse{
        ModifyOrderResponse {response, engine_error, orderbook_error }
    }
}

impl GetOrderResponse{
    pub fn new(response:Option<OpenOrder>,engine_error: Option<TradingEngineError>,orderbook_error:Option<CustomError>)->GetOrderResponse{
        GetOrderResponse { response, engine_error, orderbook_error }
    }
}