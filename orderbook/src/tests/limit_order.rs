use rust_decimal::dec;

use crate::{orderbook::types::Side, LimitOrder, MarketOrder, Orderbook};


#[test]
fn create_limit_order(){
    let mut orderbook =Orderbook::new();
    let limit_order = LimitOrder{price:dec!(10.5),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order.clone());
}

