use orderbook::{LimitOrder, OpenOrder};
use serde::{Deserialize, Serialize};
use trading_engine::{TradingEngineError, TradingPair};


#[derive(Serialize,Deserialize)]
pub struct LimitOrderRequest{
    pub trading_pair: TradingPair,
    pub order: LimitOrder
}

#[derive(Serialize,Deserialize)]
pub struct LimitOrderResponse{
    open_order: Option<OpenOrder>,
    error: Option<TradingEngineError>
}

impl LimitOrderResponse{
    pub fn new(o:Option<OpenOrder>,error:Option<TradingEngineError>)->LimitOrderResponse{
        LimitOrderResponse { open_order: o, error:error }
    }
}