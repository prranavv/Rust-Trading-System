use orderbook::Depth;
use serde::{Deserialize, Serialize};
use trading_engine::{TradingEngineError, TradingPair};


#[derive(Serialize,Deserialize)]
pub struct MarketDepthRequest{
    pub trading_pair:TradingPair
}

#[derive(Serialize,Deserialize)]
pub struct MarketDepthResponse{
    depth:Option<Depth>,
    error:Option<TradingEngineError>
}

impl MarketDepthResponse {
    pub fn new(depth:Option<Depth>,error:Option<TradingEngineError>)->MarketDepthResponse{
        MarketDepthResponse { depth, error}
    }
}