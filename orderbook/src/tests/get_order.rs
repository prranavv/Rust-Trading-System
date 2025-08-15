#[cfg(test)]
use pretty_assertions::{assert_eq};
use rust_decimal::dec;

use crate::{orderbook::{response::CustomError, types::Side}, ErrorResponse, LimitOrder, OpenOrder, Orderbook};

#[test]
fn test_get_existing_order(){
    let mut orderbook = Orderbook::new();
    
    let limit_order_1= LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    let open_order_1 = orderbook.add_limit_order(limit_order_1);
    let result = orderbook.get_order(open_order_1.order_id);
    assert_eq!(result,Ok(open_order_1));
}

#[test]
fn test_get_matched_order(){
    let mut orderbook = Orderbook::new();
    
    let limit_order_1= LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_2= LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let open_order_1 = orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    let result = orderbook.get_order(open_order_1.order_id);
    let expected_result = OpenOrder::new(dec!(105), dec!(200), Side::Bids, dec!(200), 1, 1);
    assert_eq!(result,Ok(expected_result));
}

#[test]
fn test_get_partially_filled_order(){
    let mut orderbook = Orderbook::new();
    
    let limit_order_1= LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_2= LimitOrder{price:dec!(105),quantity:dec!(100),side:Side::Asks,user_id:1};
    let open_order_1=orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    let result = orderbook.get_order(open_order_1.order_id);
    let expected_result = OpenOrder::new(dec!(105), dec!(200), Side::Bids, dec!(100), 1, 1);
    assert_eq!(result,Ok(expected_result));
}

#[test]
fn test_non_existent_order(){
    let orderbook =Orderbook::new();
    let result =orderbook.get_order(1);
    assert_eq!(result,Err(ErrorResponse::new(CustomError::OrderDoesNotExist)));
}