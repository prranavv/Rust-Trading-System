use trading_engine::TradingPair;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct CreateMarketRequest{
    pub trading_pair: TradingPair
}

#[derive(Deserialize,Serialize)]
pub struct CreateMarketResponse{
    created:bool,
    trading_pair:TradingPair,
}

impl CreateMarketResponse{
    pub fn new(status:bool,trading_pair:TradingPair)->CreateMarketResponse{
        CreateMarketResponse{
            created:status,
            trading_pair
        }
    }
}