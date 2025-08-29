use orderbook::{CustomError, DeleteResponse};
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

impl DeleteOrderResponse{
    pub fn new(response:Option<DeleteResponse>,engine_error:Option<TradingEngineError>,orderbook_error:Option<CustomError>)->DeleteOrderResponse{
        DeleteOrderResponse{ response,engine_error,orderbook_error }
    }
}