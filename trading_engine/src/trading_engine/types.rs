use std::collections::HashMap;

use orderbook::{
    Orderbook
};
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Clone)]
pub struct TradingEngine{
    pub orderbooks: HashMap<TradingPair,Orderbook> 
}

#[derive(Debug, Clone, PartialEq, Eq, Hash,Serialize,Deserialize)]
pub struct TradingPair{
    pub base:String,
    pub quote:String
}

#[derive(Debug,PartialEq,Serialize,Deserialize)]
pub enum TradingEngineError{
    TradingPairDoesNotExist,
    TradingPairAlreadyExists
}

#[derive(PartialEq,Debug,Serialize,Deserialize)]
pub struct Markets{
    pub markets: Vec<TradingPair>
}

impl Markets{
    pub fn new(markets: Vec<TradingPair>)->Markets{
        Markets { markets }
    }
}

impl TradingPair{
    pub fn new(base:String,quote:String)->TradingPair{
        TradingPair { base, quote }
    }
}