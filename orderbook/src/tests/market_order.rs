use std::collections::{BTreeMap, VecDeque};

use rust_decimal::{dec, Decimal};

use crate::{orderbook::{response::CustomError, types::Side}, LimitOrder, MarketOrder, MarketOrderResponse, OpenOrder, Orderbook};
#[cfg(test)]
use pretty_assertions::{assert_eq};
use std::cmp::Reverse;

#[test]
fn create_market_order(){
    let mut orderbook = Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order);
    let market_order = MarketOrder::new(dec!(10), Side::Bids, 1);
    let market_order_response = orderbook.add_market_order(market_order);
    let expected_market_order_response = MarketOrderResponse::new(true, Some(dec!(105)), Some(dec!(10)),None);
    assert_eq!(market_order_response,expected_market_order_response);
}

#[test]
fn if_there_is_no_order(){
    let mut orderbook = Orderbook::new();
    // There is no BIDS
    let market_order = MarketOrder::new(dec!(10), Side::Asks, 1);
    let market_order_response = orderbook.add_market_order(market_order);
    let expected_market_order_response = MarketOrderResponse::new(false, None, None,Some(CustomError::LimitOrderDoesNotExist));
    assert_eq!(market_order_response,expected_market_order_response);

    //There is no ASKS
    let market_order = MarketOrder::new(dec!(10), Side::Bids, 1);
    let market_order_response = orderbook.add_market_order(market_order);
    let expected_market_order_response = MarketOrderResponse::new(false, None, None,Some(CustomError::LimitOrderDoesNotExist));
    assert_eq!(market_order_response,expected_market_order_response);

}

#[test]
fn if_there_is_not_enough_quantity_in_orderbook(){
    let mut orderbook = Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order);
    let market_order = MarketOrder::new(dec!(300), Side::Bids, 1);
    let market_order_response = orderbook.add_market_order(market_order);
    let expected_market_order_response = MarketOrderResponse::new(true, Some(dec!(105)), Some(dec!(200)),None);
    assert_eq!(market_order_response,expected_market_order_response);
    
    drop(orderbook);

    let mut orderbook = Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order);
    let market_order = MarketOrder::new(dec!(300), Side::Asks, 1);
    let market_order_response = orderbook.add_market_order(market_order);
    let expected_market_order_response = MarketOrderResponse::new(true, Some(dec!(105)), Some(dec!(200)),None);
    assert_eq!(market_order_response,expected_market_order_response);
}

#[test]
fn filling_multiple_orders_on_orderbook(){
    //Asks order
    let mut orderbook = Orderbook::new();
    let limit_order_1= LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_2= LimitOrder{price:dec!(107),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    let market_order = MarketOrder::new(dec!(300), Side::Bids, 1);
    let market_order_response = orderbook.add_market_order(market_order);
    let expected_market_order_response = MarketOrderResponse::new(true, Some(dec!(106)), Some(dec!(300)),None);
    assert_eq!(market_order_response,expected_market_order_response);
    
    let asks=orderbook.asks;
    let mut expected_asks:BTreeMap<Decimal,VecDeque<OpenOrder>>=BTreeMap::new();
    expected_asks.entry(dec!(107)).or_insert(VecDeque::new()).push_back(OpenOrder::new(dec!(107), dec!(200), Side::Asks, dec!(100), 1, 2));
    assert_eq!(asks,expected_asks);

    //Bids Order
    let mut orderbook = Orderbook::new();
    let limit_order_1= LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_2= LimitOrder{price:dec!(107),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    let market_order = MarketOrder::new(dec!(300), Side::Asks, 1);
    let market_order_response = orderbook.add_market_order(market_order);
    let expected_market_order_response = MarketOrderResponse::new(true, Some(dec!(106)), Some(dec!(300)),None);
    assert_eq!(market_order_response,expected_market_order_response);
    let bids = orderbook.bids;
    let mut expected_bids:BTreeMap<Reverse<Decimal>,VecDeque<OpenOrder>>=BTreeMap::new();
    expected_bids.entry(Reverse(dec!(105))).or_insert(VecDeque::new()).push_back(OpenOrder::new(dec!(105), dec!(200), Side::Bids, dec!(100), 1, 1));
    assert_eq!(bids,expected_bids);
}