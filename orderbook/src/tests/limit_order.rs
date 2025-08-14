use rust_decimal::dec;

use crate::{orderbook::types::Side, LimitOrder, OpenOrder, Orderbook};
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