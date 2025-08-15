use rust_decimal::dec;

use crate::{orderbook::types::Side, LimitOrder,Orderbook};
use std::cmp::Reverse;

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

#[test]
fn test_get_best_ask(){
    let mut orderbook = Orderbook::new();

    //Check if the asks is empty
    assert!(orderbook.get_best_ask()==None);

    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order.clone());
    assert_ne!(orderbook.get_best_ask(),None);
    assert_eq!(orderbook.get_best_ask(),Some(&limit_order.price));
    
    let limit_order_2 = LimitOrder{price:dec!(110),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order_2.clone());
    assert_ne!(orderbook.get_best_ask(),Some(&limit_order_2.price));
    assert_eq!(orderbook.get_best_ask(),Some(&limit_order.price));
    
    let limit_order_3= LimitOrder{price:dec!(100),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order_3.clone());
    assert_ne!(orderbook.get_best_ask(),Some(&limit_order.price));
    assert_eq!(orderbook.get_best_ask(),Some(&limit_order_3.price));
}

#[test]
fn test_get_best_bid(){
    let mut orderbook = Orderbook::new();
    
    assert!(orderbook.get_best_bid()==None);

    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order.clone());
    assert_ne!(orderbook.get_best_bid(),None);
    assert_eq!(orderbook.get_best_bid(),Some(&Reverse(limit_order.price)));

    let limit_order_2 = LimitOrder{price:dec!(105.5),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order_2.clone());
    assert_ne!(orderbook.get_best_bid(),Some(&Reverse(limit_order.price)));
    assert_eq!(orderbook.get_best_bid(),Some(&Reverse(limit_order_2.price)));

    let limit_order_3 = LimitOrder{price:dec!(104.5),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order_3.clone());
    assert_ne!(orderbook.get_best_bid(),Some(&Reverse(limit_order_3.price)));
    assert_eq!(orderbook.get_best_bid(),Some(&Reverse(limit_order_2.price)));
}

#[test]
fn test_get_spread(){
    let mut orderbook = Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order.clone());
    let limit_order_2 = LimitOrder{price:dec!(104),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order_2.clone());

    let result = orderbook.get_spread();
    assert_eq!(result,Some(dec!(1)));

    drop(orderbook);

    let orderbook = Orderbook::new();
    let result = orderbook.get_spread();
    assert_eq!(result,None);
}

#[test]
fn test_get_mid_price(){
    let mut orderbook = Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order.clone());
    let limit_order_2 = LimitOrder{price:dec!(104),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order_2.clone());

    let result = orderbook.mid_price();
    assert_eq!(result,Some(dec!(104.5)));

    drop(orderbook);

    let orderbook = Orderbook::new();
    let result = orderbook.get_spread();
    assert_eq!(result,None);
}