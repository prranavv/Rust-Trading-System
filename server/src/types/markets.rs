use trading_engine::{Markets, TradingPair};
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

#[derive(Deserialize,Serialize)]
pub struct GetMarketsResponse{
    markets:Vec<TradingPair>
}

impl CreateMarketResponse{
    pub fn new(status:bool,trading_pair:TradingPair)->CreateMarketResponse{
        CreateMarketResponse{
            created:status,
            trading_pair
        }
    }
}

impl GetMarketsResponse{
    pub fn new(markets:Markets)->GetMarketsResponse{
        GetMarketsResponse { markets:markets.markets }
    }
}