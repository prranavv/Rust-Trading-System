use std::collections::HashMap;

use orderbook::{
    Orderbook
};


pub struct TradingEngine{
    pub orderbooks: HashMap<TradingPair,Orderbook> 
}

#[derive(Eq, Hash, PartialEq,Clone)]
pub struct TradingPair{
    pub base:String,
    pub quote:String
}

#[derive(Debug,PartialEq)]
pub enum TradingEngineError{
    TradingPairDoesNotExist
}

impl TradingPair{
    pub fn new(base:String,quote:String)->TradingPair{
        TradingPair { base, quote }
    }
}