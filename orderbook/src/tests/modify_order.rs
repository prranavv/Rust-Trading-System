use rust_decimal::{dec};
use crate::{orderbook::{response::CustomError, types::Side}, ErrorResponse, LimitOrder, ModifyOrderRequest, ModifyOrderResponse, Orderbook};
#[cfg(test)]
use pretty_assertions::{assert_eq};


#[test]
fn test_modify_order(){
    let mut orderbook = Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order);
    let modify_order_request=ModifyOrderRequest{price:Some(dec!(107)),quantity:Some(dec!(300)),order_id:1};
    let result=orderbook.modify_order(modify_order_request);
    assert_eq!(result,Ok(ModifyOrderResponse::new(dec!(107),dec!(300),1)));

    drop(orderbook);

    let mut orderbook = Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order);
    let modify_order_request=ModifyOrderRequest{price:Some(dec!(107)),quantity:Some(dec!(300)),order_id:1};
    let result=orderbook.modify_order(modify_order_request);
    assert_eq!(result,Ok(ModifyOrderResponse::new(dec!(107),dec!(300),1)));
}

#[test]
fn test_modify_order_if_order_does_not_exist(){
    let mut orderbook = Orderbook::new();
    let modify_order_request=ModifyOrderRequest{price:Some(dec!(107)),quantity:Some(dec!(300)),order_id:1};
    let result=orderbook.modify_order(modify_order_request);
    assert_eq!(result,Err(ErrorResponse::new(CustomError::OrderDoesNotExist)));
}

#[test]
fn test_modify_order_if_order_already_matched(){
    let mut orderbook = Orderbook::new();
    let limit_order_1 = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_2 = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    let modify_order_request=ModifyOrderRequest{price:Some(dec!(107)),quantity:Some(dec!(300)),order_id:1};
    let result=orderbook.modify_order(modify_order_request);
    assert_eq!(result,Err(ErrorResponse::new(CustomError::OrderAlreadyMatched)));
}