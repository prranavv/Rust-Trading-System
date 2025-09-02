use orderbook::{CustomError, MarketOrder, MarketOrderResponse, Side};
use rust_decimal::dec;

use crate::trading_engine::types::{TradingEngine, TradingPair};


#[test]
fn test_market_order_if_orderbook_is_empty(){
    let mut engine = TradingEngine::new();
    let trading_pair =TradingPair::new("BTC".to_string(),"USDT".to_string());
    let _ =engine.create_market(trading_pair.clone());
    let market_order =MarketOrder::new(dec!(100), Side::Asks, 1);
    let result = engine.add_market_order_into_market(trading_pair, market_order);
    let expected_response = MarketOrderResponse::new(false, None,None , Some(CustomError::LimitOrderDoesNotExist));
    assert_eq!(result,Ok(expected_response));
}