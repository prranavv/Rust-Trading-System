use orderbook::{LimitOrder, OpenOrder, Side};
use rust_decimal::dec;

use crate::trading_engine::types::{TradingEngine, TradingEngineError, TradingPair};

#[test]
fn test_add_limit_order(){
    let mut engine = TradingEngine::new();
    let trading_pair =TradingPair::new("BTC".to_string(),"USDT".to_string());
    engine.create_market(trading_pair.clone());
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let result = engine.add_limit_order_into_market(trading_pair, limit_order);
    
    assert_ne!(result,Err(TradingEngineError::TradingPairDoesNotExist));
    
    let open_order = OpenOrder::new(dec!(105), dec!(200), Side::Asks, dec!(0), 1, 1);
    
    assert_eq!(result,Ok(open_order));

    let trading_pair =TradingPair::new("BTC".to_string(),"SOL".to_string());
    engine.create_market(trading_pair.clone());
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let result = engine.add_limit_order_into_market(trading_pair, limit_order);
    
    assert_ne!(result,Err(TradingEngineError::TradingPairDoesNotExist));
    
    let open_order = OpenOrder::new(dec!(105), dec!(200), Side::Asks, dec!(0), 1, 1);
    
    assert_eq!(result,Ok(open_order));
}