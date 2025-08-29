use orderbook::{MarketOrder, MarketOrderResponse as MarketOrderResp};
use serde::{Deserialize, Serialize};
use trading_engine::{TradingEngineError, TradingPair};


#[derive(Serialize,Deserialize)]
pub struct MarketOrderRequest{
    pub trading_pair: TradingPair,
    pub order: MarketOrder
}

#[derive(Serialize,Deserialize)]
pub struct MarketOrderResponse{
    response:Option<MarketOrderResp>,
    error: Option<TradingEngineError>
}

impl MarketOrderResponse{
    pub fn new(response:Option<MarketOrderResp>,e:Option<TradingEngineError>)->MarketOrderResponse{
        MarketOrderResponse { response, error: e }
    }
}