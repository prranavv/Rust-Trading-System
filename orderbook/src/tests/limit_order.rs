use std::collections::VecDeque;

use rust_decimal::dec;

use crate::{orderbook::types::Side, LimitOrder, OpenOrder, Orderbook};
use std::cmp::Reverse;
#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne,};

#[test]
fn create_limit_order(){
    let mut orderbook =Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let open_order=orderbook.add_limit_order(limit_order);

    //Wrong Side
    assert_ne!(open_order,OpenOrder::new(dec!(105), dec!(200), Side::Bids, dec!(0), 1, 1));
    //Wrong Filled Quantity
    assert_ne!(open_order,OpenOrder::new(dec!(105), dec!(200), Side::Asks, dec!(100), 1, 1));
    //Wrong UserID
    assert_ne!(open_order,OpenOrder::new(dec!(105), dec!(200), Side::Asks, dec!(0), 2, 1));
    //Wrong OrderID
    assert_ne!(open_order,OpenOrder::new(dec!(105), dec!(200), Side::Asks, dec!(0), 1, 23));
    //Everything Correct
    assert_eq!(open_order,OpenOrder::new(dec!(105), dec!(200), Side::Asks, dec!(0), 1, 1));
}

#[test]
fn adding_multiple_same_price_limit_order(){
    let mut orderbook =Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let open_order_1= orderbook.add_limit_order(limit_order);
    assert!(orderbook.asks.len()==1); // Check if the orderbook has gained an order in the asks

    let limit_order_2 = LimitOrder{price:dec!(105),quantity:dec!(800),side:Side::Asks,user_id:1};
    let open_order_2 =orderbook.add_limit_order(limit_order_2);
    assert!(orderbook.asks.len()==1); // Check if the orderbook has gained an order at the same price

    let asks=orderbook.asks.get(&open_order_1.price).unwrap();
    let mut expected_vec:VecDeque<OpenOrder> = VecDeque::new();
    expected_vec.push_back(open_order_1);
    expected_vec.push_back(open_order_2);
    assert_eq!(asks,&expected_vec); // Check if the asks is same as the one we expected
    
    //Do the same "BIDS"
    let limit_order = LimitOrder{price:dec!(100),quantity:dec!(200),side:Side::Bids,user_id:1};
    let open_order_1= orderbook.add_limit_order(limit_order);
    assert!(orderbook.bids.len()==1);

    let limit_order_2 = LimitOrder{price:dec!(100),quantity:dec!(800),side:Side::Bids,user_id:1};
    let open_order_2 =orderbook.add_limit_order(limit_order_2);
    assert!(orderbook.bids.len()==1);

    let bids=orderbook.bids.get(&Reverse(open_order_1.price)).unwrap();
    let mut expected_vec:VecDeque<OpenOrder> = VecDeque::new();
    expected_vec.push_back(open_order_1);
    expected_vec.push_back(open_order_2);
    assert_eq!(bids,&expected_vec);
}

#[test]
fn matching_limit_orders(){
    
}