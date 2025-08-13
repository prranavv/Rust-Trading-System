use std::collections::HashMap;
use std::collections::{BTreeMap, VecDeque};

use rust_decimal::Decimal;

use std::cmp::Ordering;

use std::cmp::Reverse;


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

pub struct DeleteResponse{
    price:Decimal,
    quantity:Decimal,
    quantity_filled:Decimal,
    order_id:u64
}

pub enum CustomError{
    OrderDoesNotExist
}

pub struct DeleteResponseError{
    error:CustomError
}

impl DeleteResponseError{
    pub fn new(err: CustomError)->DeleteResponseError{
        DeleteResponseError { error:err}
    }
}

impl DeleteResponse{
    pub fn new(price:Decimal,quantity:Decimal,quantity_filled:Decimal,order_id:u64)->DeleteResponse{
        DeleteResponse { price,quantity, quantity_filled, order_id }
    }
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

#[derive(Clone)]
pub enum Side{
    Asks,
    Bids
}

pub struct MarketOrderResponse{
    success: bool,
    average_price: Decimal,
    quantity:Decimal
}

impl MarketOrderResponse{
    pub fn new(success: bool,average_price:Decimal,quantity:Decimal)->MarketOrderResponse{
        MarketOrderResponse { success, average_price, quantity }
    }
}