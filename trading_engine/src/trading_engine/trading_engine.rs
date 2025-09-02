use std::collections::{BTreeMap};
use orderbook::{
    DeleteResponse, Depth, ErrorResponse, LimitOrder, MarketOrder, MarketOrderResponse, ModifyOrderRequest, ModifyOrderResponse, OpenOrder, Orderbook
};
use rust_decimal::{Decimal};
use crate::trading_engine::types::{Markets, TradingEngine, TradingEngineError, TradingPair};



impl TradingEngine{
    pub fn new()->TradingEngine{
        TradingEngine { orderbooks:BTreeMap::new() }
    }

    fn check_if_market_exists(&self,trading_pair:TradingPair)->bool{
        let engine=&self.orderbooks;
        engine.contains_key(&trading_pair)
    }

    fn add_limit_order_for_trading_pair(&mut self,trading_pair:TradingPair,order: LimitOrder)->OpenOrder{
        self.orderbooks.entry(trading_pair).or_insert(Orderbook::new()).add_limit_order(order)
    }

    fn add_market_order_for_trading_pair(&mut self,trading_pair:TradingPair,order:MarketOrder)->MarketOrderResponse{
        self.orderbooks.entry(trading_pair).or_insert(Orderbook::new()).add_market_order(order)
    }

    fn get_depth_for_trading_pair(&mut self,trading_pair:TradingPair)->Depth{
        self.orderbooks.entry(trading_pair).or_insert(Orderbook::new()).get_depth()
    }

    fn delete_order_for_trading_pair(&mut self,trading_pair:TradingPair,order_id:u64)->Result<DeleteResponse,ErrorResponse>{
        self.orderbooks.entry(trading_pair).or_insert(Orderbook::new()).delete_order(order_id)
    }

    fn modify_order_for_trading_pair(&mut self,trading_pair:TradingPair,order:ModifyOrderRequest)->Result<ModifyOrderResponse,ErrorResponse>{
        self.orderbooks.entry(trading_pair).or_insert(Orderbook::new()).modify_order(order)
    }
    
    fn get_order_by_id_for_trading_pair(&mut self,trading_pair:TradingPair,order_id:u64)->Result<OpenOrder,ErrorResponse>{
        self.orderbooks.entry(trading_pair).or_insert(Orderbook::new()).get_order(order_id)
    }

    fn get_mid_price_for_trading_pair(&mut self,trading_pair:TradingPair)->Option<Decimal>{
        self.orderbooks.entry(trading_pair).or_insert(Orderbook::new()).mid_price()
    }

    fn _get_markets(&self)->Markets{
        let keys=self.orderbooks.keys();
        let mut vec=Vec::<TradingPair>::new();
        for key in keys{
            vec.push(key.to_owned());
        }
        Markets::new(vec)
    }

    pub fn get_markets(&self)->Markets{
        self._get_markets()
    }


    pub fn create_market(&mut self,trading_pair:TradingPair)->Result<(),TradingEngineError>{
        if self.check_if_market_exists(trading_pair.clone()){
            return Err(TradingEngineError::TradingPairAlreadyExists)
        }
        self.orderbooks.insert(trading_pair, Orderbook::new());
        Ok(())
    }

    pub fn add_limit_order_into_market(&mut self,trading_pair:TradingPair,order: LimitOrder)->Result<OpenOrder,TradingEngineError>{
        if !self.check_if_market_exists(trading_pair.clone()){
            return Err(TradingEngineError::TradingPairDoesNotExist)
        }
        Ok(self.add_limit_order_for_trading_pair(trading_pair, order))
    }

    pub fn add_market_order_into_market(&mut self,trading_pair:TradingPair,order:MarketOrder)->Result<MarketOrderResponse,TradingEngineError>{
        if !self.check_if_market_exists(trading_pair.clone()){
            return Err(TradingEngineError::TradingPairDoesNotExist)
        }
        Ok(self.add_market_order_for_trading_pair(trading_pair, order))
    }

    pub fn get_market_depth(&mut self,trading_pair:TradingPair)->Result<Depth,TradingEngineError>{
        if !self.check_if_market_exists(trading_pair.clone()){
            return Err(TradingEngineError::TradingPairDoesNotExist)
        }
        Ok(self.get_depth_for_trading_pair(trading_pair))
    }

    pub fn delete_order_for_market(&mut self,trading_pair:TradingPair,order_id:u64)->Result<Result<DeleteResponse,ErrorResponse>,TradingEngineError>{
        if !self.check_if_market_exists(trading_pair.clone()){
            return Err(TradingEngineError::TradingPairDoesNotExist)
        }
        Ok(self.delete_order_for_trading_pair(trading_pair, order_id))
    }

    pub fn modify_order_for_market(&mut self,trading_pair:TradingPair,order:ModifyOrderRequest)->Result<Result<ModifyOrderResponse,ErrorResponse>,TradingEngineError>{
        if !self.check_if_market_exists(trading_pair.clone()){
            return Err(TradingEngineError::TradingPairDoesNotExist)
        }
        Ok(self.modify_order_for_trading_pair(trading_pair, order))
    }

    pub fn get_order_by_id_for_market(&mut self,trading_pair:TradingPair,order_id:u64)->Result<Result<OpenOrder,ErrorResponse>,TradingEngineError>{
        if !self.check_if_market_exists(trading_pair.clone()){
            return Err(TradingEngineError::TradingPairDoesNotExist)
        }
        Ok(self.get_order_by_id_for_trading_pair(trading_pair, order_id))
    }

    pub fn get_mid_price_for_market(&mut self,trading_pair:TradingPair)->Result<Option<Decimal>,TradingEngineError>{
        if !self.check_if_market_exists(trading_pair.clone()){
            return Err(TradingEngineError::TradingPairDoesNotExist)
        }
        Ok(self.get_mid_price_for_trading_pair(trading_pair))
    }
}
 