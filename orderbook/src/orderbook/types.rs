use std::collections::HashMap;
use std::collections::{BTreeMap, VecDeque};

use rust_decimal::Decimal;

use std::cmp::Ordering;

use std::cmp::Reverse;

#[derive(Clone)]
pub enum Side{
    Asks,
    Bids
}
pub struct Orderbook{
    pub bids: BTreeMap<Reverse<Decimal>,VecDeque<OpenOrder>>,
    pub asks: BTreeMap<Decimal,VecDeque<OpenOrder>>,
    pub order_id_index:u64,
    pub order_map:HashMap<u64,OpenOrder>
}

pub struct LimitOrder{
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
    pub user_id:u64
}

pub struct MarketOrder{
    pub quantity: Decimal,
    pub side: Side,
    pub user_id: u64
}

pub struct Depth{
    pub bids: Vec<Order>,
    pub asks: Vec<Order>
}

pub struct Order{
    pub price: Decimal,
    pub quantity: Decimal,
    pub order_count:u64
    
}

pub struct ModifyOrderRequest{
    pub price:Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub order_id:u64
}

#[derive(Clone)]
pub struct OpenOrder{
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
    pub quantity_filled:Decimal,
    pub user_id: u64,
    pub order_id: u64
}

impl OpenOrder {
    pub fn new(price:Decimal,quantity:Decimal,side:Side,quantity_filled:Decimal,user_id:u64,order_id:u64)->OpenOrder{
        OpenOrder { price, quantity, side, quantity_filled, user_id, order_id }
    }
}

