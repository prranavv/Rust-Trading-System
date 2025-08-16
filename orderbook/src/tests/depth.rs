#[cfg(test)]
use pretty_assertions::{assert_eq};
use rust_decimal::dec;

use crate::{orderbook::{types::{Order, Side}}, Depth, LimitOrder, Orderbook};

#[test]
fn test_get_depth(){
    let mut orderbook = Orderbook::new();
    let limit_order_1= LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_2 = LimitOrder{price:dec!(106),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_3 = LimitOrder{price:dec!(107),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_4 = LimitOrder{price:dec!(108),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    orderbook.add_limit_order(limit_order_3);
    orderbook.add_limit_order(limit_order_4);

    let depth =orderbook.get_depth();
    let expected_bids=Vec::<Order>::new();
    let mut expected_asks=Vec::<Order>::new();
    let order_1=Order::new(dec!(105), dec!(200), 1);
    let order_2=Order::new(dec!(106), dec!(200), 1);
    let order_3=Order::new(dec!(107), dec!(200), 1);
    let order_4=Order::new(dec!(108), dec!(200), 1);
    expected_asks.push(order_1);
    expected_asks.push(order_2);
    expected_asks.push(order_3);
    expected_asks.push(order_4);
    let expected_depth=Depth{asks:expected_asks,bids:expected_bids};
    assert_eq!(depth,expected_depth);

    let limit_order_1= LimitOrder{price:dec!(103),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_2 = LimitOrder{price:dec!(102),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_3 = LimitOrder{price:dec!(101),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_4 = LimitOrder{price:dec!(100),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    orderbook.add_limit_order(limit_order_3);
    orderbook.add_limit_order(limit_order_4);

    let depth =orderbook.get_depth();

    let mut expected_bids=Vec::<Order>::new();
    let mut expected_asks=Vec::<Order>::new();
    let order_1=Order::new(dec!(105), dec!(200), 1);
    let order_2=Order::new(dec!(106), dec!(200), 1);
    let order_3=Order::new(dec!(107), dec!(200), 1);
    let order_4=Order::new(dec!(108), dec!(200), 1);
    expected_asks.push(order_1);
    expected_asks.push(order_2);
    expected_asks.push(order_3);
    expected_asks.push(order_4);
    let order_1=Order::new(dec!(103), dec!(200), 1);
    let order_2=Order::new(dec!(102), dec!(200), 1);
    let order_3=Order::new(dec!(101), dec!(200), 1);
    let order_4=Order::new(dec!(100), dec!(200), 1);
    expected_bids.push(order_1);
    expected_bids.push(order_2);
    expected_bids.push(order_3);
    expected_bids.push(order_4);

    let expected_depth=Depth{asks:expected_asks,bids:expected_bids};
    assert_eq!(depth,expected_depth);
}