#[cfg(test)]
use pretty_assertions::{assert_eq};
use rust_decimal::dec;

use crate::{orderbook::{response::CustomError, types::Side}, DeleteResponse, ErrorResponse, LimitOrder, Orderbook};

#[test]
fn test_delete_order(){
    let mut orderbook = Orderbook::new();
    
    //Asks Order
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let open_order=orderbook.add_limit_order(limit_order);
    let result = orderbook.delete_order(open_order.order_id);
    assert_eq!(result,Ok(DeleteResponse::new(dec!(105),dec!(200),dec!(0),open_order.order_id)));

    drop(orderbook);

    let mut orderbook = Orderbook::new();
    
    //Bids Order
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    let open_order=orderbook.add_limit_order(limit_order);
    let result = orderbook.delete_order(open_order.order_id);
    assert_eq!(result,Ok(DeleteResponse::new(dec!(105),dec!(200),dec!(0),open_order.order_id)));
}

#[test]
fn test_deleting_an_non_existent_order(){
    let mut orderbook = Orderbook::new();
    let  result = orderbook.delete_order(4);
    assert_eq!(result,Err(ErrorResponse::new(CustomError::OrderDoesNotExist)));
}

#[test]
fn test_deleting_an_matched_order(){
    let mut orderbook = Orderbook::new();
    let limit_order_1 = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_2= LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    assert!(orderbook.asks.is_empty());
    assert!(orderbook.bids.is_empty());

    let result =orderbook.delete_order(1);
    assert_eq!(result,Err(ErrorResponse::new(CustomError::OrderAlreadyMatched)));
}