use std::collections::VecDeque;

use rust_decimal::dec;

use crate::{orderbook::types::{Order, Side}, LimitOrder, OpenOrder, Orderbook};
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
    //Match an exisiting asks order with a bids order which has the same price
    let mut orderbook =Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order);
    assert!(orderbook.asks.len()==1); // Check if the orderbook has gained an order in the asks

    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(100),side:Side::Bids,user_id:1};
    let open_order_2= orderbook.add_limit_order(limit_order);
    let expected_open_order = OpenOrder::new(dec!(105), dec!(100), Side::Bids, dec!(100), 1, 2); //The quantity should have been filled
    assert_eq!(open_order_2,expected_open_order);

    drop(orderbook);

    //Match an exisiting bids order with an asks order which hash the same price
    let mut orderbook =Orderbook::new();
    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order);
    assert!(orderbook.bids.len()==1); // Check if the orderbook has gained an order in the bids

    let limit_order = LimitOrder{price:dec!(105),quantity:dec!(100),side:Side::Asks,user_id:1};
    let open_order_2= orderbook.add_limit_order(limit_order);
    let expected_open_order = OpenOrder::new(dec!(105), dec!(100), Side::Asks, dec!(100), 1, 2); //The quantity should have been filled
    assert_eq!(open_order_2,expected_open_order);

    drop(orderbook);

    //Matching an exisiting Asks order with a bids order whose price is 
    //better than the Asks so it gets matched with orders until it hits the expected price
    let mut orderbook = Orderbook::new();
    let limit_order_1 = LimitOrder{price:dec!(105.1),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_2 = LimitOrder{price:dec!(105.2),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_3 = LimitOrder{price:dec!(105.5),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_4 = LimitOrder{price:dec!(105.8),quantity:dec!(200),side:Side::Asks,user_id:1};
    let limit_order_5 = LimitOrder{price:dec!(105.9),quantity:dec!(200),side:Side::Asks,user_id:1};
    orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    orderbook.add_limit_order(limit_order_3);
    orderbook.add_limit_order(limit_order_4);
    orderbook.add_limit_order(limit_order_5);

    let limit_order_6= LimitOrder{price:dec!(105.5),quantity:dec!(100),side:Side::Bids,user_id:1};
    let open_order=orderbook.add_limit_order(limit_order_6);
    let expected_open_order=OpenOrder::new(dec!(105.5), dec!(100), Side::Bids, dec!(100), 1, 6);
    assert_eq!(open_order,expected_open_order);
    //Check the first limit order that has been partially filled
    let entry = orderbook.asks
                            .get(&dec!(105.1))
                            .unwrap()
                            .iter()
                            .find(|v|v.order_id==1)
                            .unwrap();
    assert_eq!(entry.quantity_filled,dec!(100));

    let limit_order_7= LimitOrder{price:dec!(105.5),quantity:dec!(600),side:Side::Bids,user_id:1};
    let open_order=orderbook.add_limit_order(limit_order_7);
    let expected_open_order=OpenOrder::new(dec!(105.5), dec!(600), Side::Bids, dec!(500), 1, 7);
    assert_eq!(open_order,expected_open_order);

    let bids = orderbook.get_bids();
    let mut expected_bids:Vec<Order> =Vec::new();
    let order_1 = Order{price:open_order.price,quantity:dec!(100),order_count:1};
    expected_bids.push(order_1);
    assert_eq!(bids,expected_bids);

    drop(orderbook);

    //Matching an exisiting Bids order with a asks order whose price is 
    //better than the Asks so it gets matched with orders until it hits the expected price
    let mut orderbook = Orderbook::new();
    let limit_order_1 = LimitOrder{price:dec!(105.1),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_2 = LimitOrder{price:dec!(105.2),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_3 = LimitOrder{price:dec!(105.5),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_4 = LimitOrder{price:dec!(105.8),quantity:dec!(200),side:Side::Bids,user_id:1};
    let limit_order_5 = LimitOrder{price:dec!(105.9),quantity:dec!(200),side:Side::Bids,user_id:1};
    orderbook.add_limit_order(limit_order_1);
    orderbook.add_limit_order(limit_order_2);
    orderbook.add_limit_order(limit_order_3);
    orderbook.add_limit_order(limit_order_4);
    orderbook.add_limit_order(limit_order_5);

    let limit_order_6= LimitOrder{price:dec!(105.5),quantity:dec!(100),side:Side::Asks,user_id:1};
    let open_order=orderbook.add_limit_order(limit_order_6);
    let expected_open_order=OpenOrder::new(dec!(105.5), dec!(100), Side::Asks, dec!(100), 1, 6);
    assert_eq!(open_order,expected_open_order);
    //Check the first limit order that has been partially filled
    let entry = orderbook.bids
                            .get(&Reverse(dec!(105.9)))
                            .unwrap()
                            .iter()
                            .find(|v|v.order_id==5)
                            .unwrap();
    assert_eq!(entry.quantity_filled,dec!(100));

    let limit_order_7= LimitOrder{price:dec!(105.5),quantity:dec!(600),side:Side::Asks,user_id:1};
    let open_order=orderbook.add_limit_order(limit_order_7);
    let expected_open_order=OpenOrder::new(dec!(105.5), dec!(600), Side::Asks, dec!(500), 1, 7);
    assert_eq!(open_order,expected_open_order);

    let asks = orderbook.get_asks();
    let mut expected_asks:Vec<Order> =Vec::new();
    let order_1 = Order{price:open_order.price,quantity:dec!(100),order_count:1};
    expected_asks.push(order_1);
    assert_eq!(asks,expected_asks);
}