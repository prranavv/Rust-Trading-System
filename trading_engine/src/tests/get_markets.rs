use crate::trading_engine::types::{Markets, TradingEngine, TradingPair};



#[test]
fn test_get_markets(){
    let mut engine = TradingEngine::new();
    let td1 = TradingPair::new("BTC".to_string(), "USDC".to_string());
    let td2= TradingPair::new("BTC".to_string(), "USDT".to_string());
    let _ =engine.create_market(td1.clone());
    let _ = engine.create_market(td2.clone());
    let mut v = Vec::<TradingPair>::new();
    v.push(td1);
    v.push(td2);
    let markets = engine.get_markets();
    let expected_markets = Markets::new(v);
    assert_eq!(markets,expected_markets);
}