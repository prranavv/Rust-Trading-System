use orderbook::Depth;
use rust_decimal::Decimal;
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

#[derive(Serialize,Deserialize)]
pub struct MarketMidPriceRequest{
    pub trading_pair:TradingPair
}

#[derive(Serialize,Deserialize)]
pub struct MarketMidPriceResponse{
    price:Option<Decimal>,
    error:Option<TradingEngineError>
}

impl MarketDepthResponse {
    pub fn new(depth:Option<Depth>,error:Option<TradingEngineError>)->MarketDepthResponse{
        MarketDepthResponse { depth, error}
    }
}

impl MarketMidPriceResponse{
    pub fn new(price:Option<Decimal>,error:Option<TradingEngineError>)->MarketMidPriceResponse{
        MarketMidPriceResponse { price, error }
    }
}