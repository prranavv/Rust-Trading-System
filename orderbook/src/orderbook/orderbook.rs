use std::collections::{BTreeMap, HashMap, VecDeque};

use rust_decimal::{dec, Decimal};

use crate::{orderbook::types::{CustomError, DeleteResponse, DeleteResponseError, Depth, MarketOrderResponse, OpenOrder, Order, Side}, LimitOrder, MarketOrder, Orderbook};

use std::cmp::Reverse;

impl Orderbook{
    pub fn new()->Orderbook{
        Orderbook{
            asks:BTreeMap::new(),
            bids: BTreeMap::new(),
            order_id_index:0,
            order_map:HashMap::new()
        }
    }

    pub fn get_best_bid(&self)->Option<&Reverse<Decimal>>{
        self.bids.keys().next()
    }

    pub fn get_best_ask(&self)->Option<&Decimal>{
        self.asks.keys().next()
    }

    pub fn get_worst_bid(&self)->Option<&Reverse<Decimal>>{
        self.bids.keys().last()
    }

    pub fn get_worst_ask(&self)->Option<&Decimal>{
        self.asks.keys().last()
    }

    pub fn get_spread(&self)->Decimal{
        let best_ask = self.get_best_ask().unwrap().to_owned();
        let best_bid = self.get_best_bid().unwrap().0;
        best_ask-best_bid
    }

    pub fn mid_price(&self)->Decimal{
        let best_ask = self.get_best_ask().unwrap().to_owned();
        let best_bid = self.get_best_bid().unwrap().0;
        return (best_ask+best_bid)/dec!(2);
    }

    pub fn depth(&self)->Depth{
        let bids=self.get_bids();
        let asks=self.get_asks();
        Depth { bids, asks }
    }

    pub fn get_bids(&self) -> Vec<Order>{
        let mut bids:Vec<Order> = Vec::new();

        for (price,orders) in self.bids.iter(){
            bids.push(Order{price:price.0,quantity:orders.iter().map(|v|v.quantity-v.quantity_filled).sum(),order_count:orders.iter().count() as u64})
        }

        bids
    }

    pub fn get_asks(&self) -> Vec<Order>{
        let mut asks:Vec<Order> = Vec::new();

        for (price,orders) in self.asks.iter(){
            asks.push(Order{price:*price,quantity:orders.iter().map(|v|v.quantity-v.quantity_filled).sum(),order_count:orders.iter().count() as u64})
        }
        asks
    }

    //TODO
    pub fn delete_order(&mut self,order_id:u64)->Result<DeleteResponse,DeleteResponseError>{
        let order=self.order_map.get(&order_id);
        if let Some(o)=order{
            let side=o.side.clone();
            let price = o.price;
            match side{
                Side::Asks=>{
                    let open_orders=self.asks.get_mut(&price).unwrap();
                    let (price,quantity,quantity_filled)=open_orders
                        .iter()
                        .find(|v|v.order_id==order_id)
                        .map(|v|(v.price,v.quantity,v.quantity_filled))
                        .unwrap();
                    
                    open_orders.retain(|v|v.order_id!=order_id);
                    Ok(DeleteResponse::new(price, quantity, quantity_filled, order_id))
                },
                Side::Bids=>{
                    let open_orders=self.bids.get_mut(&Reverse(price)).unwrap();
                    let (price,quantity,quantity_filled)=open_orders
                        .iter()
                        .find(|v|v.order_id==order_id)
                        .map(|v|(v.price,v.quantity,v.quantity_filled))
                        .unwrap();
                    open_orders.retain(|v|v.order_id!=order_id);
                    Ok(DeleteResponse::new(price, quantity, quantity_filled, order_id))
                }
            }
        }else{
            let err = CustomError::OrderDoesNotExist;
            Err(DeleteResponseError::new(err))
        }   
    }

    //TODO - MODIFY IN PLACE WITHOUT CHANGING THE TIME PRIORITY
    pub fn modify_order(&mut self,order_id:u64){

    }

    pub fn add_limit_order(&mut self,order: LimitOrder)->OpenOrder{
        self.order_id_index+=1;
        let order_id=self.order_id_index;
        let open_order=self.match_limit_order(order, order_id);
        self.order_map.insert(order_id, open_order.clone());
        open_order
    }
    
    fn match_limit_order(&mut self,order: LimitOrder,order_id:u64)->OpenOrder{
        let price = order.price;
        let mut remaining_quantity=order.quantity;
        match order.side{
            Side::Asks=>{
                let best_bid = self.get_best_bid().unwrap().0;
                if price>best_bid{
                    let mut new_best_bid = self.get_best_bid().unwrap().0;
                    let mut bids = self.bids.iter_mut();
                    let mut price_array:Vec<Decimal>=Vec::new();
                    'outer: while remaining_quantity>dec!(0) && new_best_bid<=price{
                        if let Some(open_order)=bids.next(){
                            let open_orders = open_order.1;
                            let order_price=open_order.0.0;
                            let mut iter = open_orders.iter_mut();
                            let mut to_remove:Vec<u64> = Vec::new();
                            new_best_bid=order_price;
                            if new_best_bid>price{
                                break 'outer;
                            }
                            while remaining_quantity>dec!(0){
                                if let Some(o) =iter.next(){
                                    let quantity_remaining=o.quantity-o.quantity_filled;
                                    if remaining_quantity>=quantity_remaining{
                                        remaining_quantity-=quantity_remaining;
                                        to_remove.push(o.order_id);
                                        // open_orders.pop_front();
                                    }else{
                                        o.quantity_filled+=remaining_quantity;
                                        remaining_quantity=dec!(0);
                                    }
                                    price_array.push(o.price); 
                                }else{
                                    break;
                                }
                            }
                            open_orders.retain(|v|to_remove.contains(&v.order_id));
                        }else {
                            break;
                        }
                    }
                    let open_order=OpenOrder::new(price, order.quantity, order.side, order.quantity-remaining_quantity, order.user_id, order_id);
                    if remaining_quantity!=dec!(0){
                        self.asks.entry(price).or_insert(VecDeque::new()).push_back(open_order.clone());
                    }
                    return open_order;
                }
                //checks if a order exists for a particular price
                if let Some(open_orders)  = self.bids.get_mut(&Reverse(price)){
                    let mut iter = open_orders.iter_mut();
                    let mut to_remove:Vec<u64> = Vec::new();
                    //while the remaining_quantity_to_be_filled is greater than 0
                    while remaining_quantity>dec!(0){
                        //if there is a element in the iterator
                        if let Some(o)=iter.next(){
                            //quantity remaining for this particular order
                            let quantity_remaining=o.quantity-o.quantity_filled;
                            if remaining_quantity>=quantity_remaining{
                                remaining_quantity-=quantity_remaining;
                                to_remove.push(o.order_id);
                            }else{
                                o.quantity_filled+=remaining_quantity;
                                remaining_quantity=dec!(0);
                            }
                        }
                        //if there is no element in the iterator do this 
                        else{
                            break;
                        }
                    }
                    open_orders.retain(|v|to_remove.contains(&v.order_id));
                    let open_order=OpenOrder::new(price, order.quantity, order.side, order.quantity-remaining_quantity, order.user_id, order_id);
                    if remaining_quantity!=dec!(0){
                        self.asks.entry(price).or_insert(VecDeque::new()).push_back(open_order.clone());
                    }
                    return open_order;
                }
                //if there is no order for that particular price do this
                else{
                    let open_order=OpenOrder::new(price, order.quantity, order.side, dec!(0), order.user_id, order_id);
                    self.asks.entry(price).or_insert(VecDeque::new()).push_back(open_order.clone());
                    return open_order;
                }
            },
            Side::Bids=>{
                let best_ask = self.get_best_ask().unwrap().clone();
                if price>best_ask{
                    let mut new_best_ask = self.get_best_ask().unwrap().clone();
                    let mut bids = self.asks.iter_mut();
                    let mut price_array:Vec<Decimal>=Vec::new();
                    'outer: while remaining_quantity>dec!(0) && new_best_ask<=price{
                        if let Some(open_order)=bids.next(){
                            let open_orders = open_order.1;
                            let order_price=open_order.0.clone();
                            let mut iter = open_orders.iter_mut();
                            let mut to_remove:Vec<u64> = Vec::new();
                            new_best_ask=order_price;
                            if new_best_ask>price{
                                break 'outer;
                            }
                            while remaining_quantity>dec!(0){
                                if let Some(o) =iter.next(){
                                    let quantity_remaining=o.quantity-o.quantity_filled;
                                    if remaining_quantity>=quantity_remaining{
                                        remaining_quantity-=quantity_remaining;
                                        to_remove.push(o.order_id);
                                        // open_orders.pop_front();
                                    }else{
                                        o.quantity_filled+=remaining_quantity;
                                        remaining_quantity=dec!(0);
                                    }
                                    price_array.push(o.price); 
                                }else{
                                    break;
                                }
                            }
                            open_orders.retain(|v|to_remove.contains(&v.order_id));
                        }else {
                            break;
                        }
                    }
                    let open_order=OpenOrder::new(price, order.quantity, order.side, order.quantity-remaining_quantity, order.user_id, order_id);
                    if remaining_quantity!=dec!(0){
                        self.bids.entry(Reverse(price)).or_insert(VecDeque::new()).push_back(open_order.clone());
                    }
                    return open_order;
                }
                //checks if a order exists for a particular price
                if let Some(open_orders)  = self.asks.get_mut(&price){
                    let mut iter = open_orders.iter_mut();
                    let mut to_remove:Vec<u64> = Vec::new();
                    //while the remaining_quantity_to_be_filled is greater than 0
                    while remaining_quantity>dec!(0){
                        //if there is a element in the iterator
                        if let Some(o)=iter.next(){
                            //quantity remaining for this particular order
                            let quantity_remaining=o.quantity-o.quantity_filled;
                            if remaining_quantity>=quantity_remaining{
                                remaining_quantity-=quantity_remaining;
                                to_remove.push(o.order_id);
                            }else{
                                o.quantity_filled+=remaining_quantity;
                                remaining_quantity=dec!(0);
                            }
                        }
                        //if there is no element in the iterator do this 
                        else{
                            break;
                        }
                    }
                    open_orders.retain(|v|to_remove.contains(&v.order_id));
                    let open_order=OpenOrder::new(price, order.quantity, order.side, order.quantity-remaining_quantity, order.user_id, order_id);
                    if remaining_quantity!=dec!(0){
                        self.bids.entry(Reverse(price)).or_insert(VecDeque::new()).push_back(open_order.clone());
                    }
                    return open_order;
                }
                //if there is no order for that particular price do this
                else{
                    let open_order=OpenOrder::new(price, order.quantity, order.side, dec!(0), order.user_id, order_id);
                    self.bids.entry(Reverse(price)).or_insert(VecDeque::new()).push_back(open_order.clone());
                    return open_order;
                }
            }
        }
    }

    pub fn add_market_order(&mut self,order:MarketOrder)->MarketOrderResponse{
        let remaining_quantity=order.quantity;
        self.match_market_order(remaining_quantity, order)
    }

    fn match_market_order(&mut self,mut remaining_quantity:Decimal,order:MarketOrder)->MarketOrderResponse{
        match order.side{
            Side::Asks=>{
                let mut bids =self.bids.iter_mut();
                let mut price_array: Vec<Decimal> = Vec::new();
                while remaining_quantity>dec!(0){
                    if let Some(open_order)=bids.next(){
                        let open_orders=open_order.1;
                        let mut iter = open_orders.iter_mut();
                        let mut to_remove:Vec<u64> = Vec::new();
                        while remaining_quantity>dec!(0) {
                            if let Some(o) =iter.next(){
                                let quantity_remaining=o.quantity-o.quantity_filled;
                                if remaining_quantity>=quantity_remaining{
                                    remaining_quantity-=quantity_remaining;
                                    to_remove.push(o.order_id);
                                    // open_orders.pop_front();
                                }else{
                                    o.quantity_filled+=remaining_quantity;
                                    remaining_quantity=dec!(0);
                                }
                                price_array.push(o.price); 
                            }else{
                                break;
                            }
                        }
                        
                        open_orders.retain(|v|to_remove.contains(&v.order_id));
                    }else{
                        break;
                    }
                }
                let quantity_filled = order.quantity-remaining_quantity;
                let length_of_price_array=price_array.len() as i64;
                let length_of_price =Decimal::new(length_of_price_array, 0);
                let iter = price_array.iter();
                let mut total_sum=dec!(0);
                for i in iter{
                    total_sum+=i;
                }
                let average_price = total_sum/length_of_price;
                let market_order_response = MarketOrderResponse::new(true, average_price, quantity_filled);
                return market_order_response;
            },
            Side::Bids=>{
                let mut asks = self.asks.iter_mut();
                let mut price_array = Vec::new();
                while remaining_quantity>dec!(0){
                    if let Some(open_order) = asks.next(){
                        let open_orders = open_order.1;
                        let mut iter = open_orders.iter_mut();
                        let mut to_remove: Vec<u64> = Vec::new();
                        while remaining_quantity>dec!(0){
                            if let Some(o)=iter.next(){
                                let quantity_remaing=o.quantity-o.quantity_filled;
                                if remaining_quantity>=(quantity_remaing){
                                    remaining_quantity-=quantity_remaing;
                                    to_remove.push(o.order_id);
                                }else{
                                    o.quantity_filled+=remaining_quantity;
                                    remaining_quantity=dec!(0);
                                }
                                price_array.push(o.price);
                            }else{
                                break;
                            }
                        }
                        open_orders.retain(|v|!to_remove.contains(&v.order_id));
                    }else{
                        break;
                    }
                }
                let quantity_filled=order.quantity-remaining_quantity;
                let length_of_price_array=price_array.len() as i64;
                let length_of_price =Decimal::new(length_of_price_array, 0);
                let iter = price_array.iter();
                let mut total_sum=dec!(0);
                for i in iter{
                    total_sum+=i;
                }
                let average_price = total_sum/length_of_price;
                let market_order_response = MarketOrderResponse::new(true, average_price, quantity_filled);
                return market_order_response;
            }
        }
    }
}