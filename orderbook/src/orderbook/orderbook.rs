use std::collections::{BTreeMap, HashMap, VecDeque};
use rust_decimal::{dec, Decimal};
use crate::{orderbook::{response::{CustomError,DeleteResponse,ErrorResponse, MarketOrderResponse,ModifyOrderResponse}, types::{Depth, ModifyOrderRequest, OpenOrder, Order, Side}}, LimitOrder, MarketOrder, Orderbook};
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

    fn clear_empty_bids_or_asks(&mut self){
        let mut to_be_removed: Vec<Reverse<Decimal>>=Vec::new();
        for (price,_) in self.bids.iter(){
            if self.bids.get(price).unwrap().is_empty(){
                to_be_removed.push(*price);
            }
        }
        for i in to_be_removed.iter(){
            self.bids.remove(i);
        }

        let mut to_be_removed_asks: Vec<Decimal>=Vec::new();
        for (price,_) in self.asks.iter(){
            if self.asks.get(price).unwrap().is_empty(){
                to_be_removed_asks.push(*price);
            }
        }
        for j in to_be_removed_asks.iter(){
            self.asks.remove(j);
        }

    }

    fn is_bids_empty(&self)->bool{
        self.bids.is_empty()
    }

    fn is_asks_empty(&self)->bool{
        self.asks.is_empty()
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

    pub fn get_spread(&self)->Option<Decimal>{
        let best_ask = self.get_best_ask()?.to_owned();
        let best_bid = self.get_best_bid()?.0;
        Some(best_ask-best_bid)
    }

    pub fn mid_price(&self)->Option<Decimal>{
        let best_ask = self.get_best_ask()?.to_owned();
        let best_bid = self.get_best_bid()?.0;
        return Some((best_ask+best_bid)/dec!(2));
    }

    pub fn get_order(&self,order_id:u64)->Result<OpenOrder,ErrorResponse>{
        let order =self.order_map.get(&order_id);
        match order{
            Some(o)=>{
                return Ok(o.clone())
            },
            None=>{
                return Err(ErrorResponse::new(CustomError::OrderDoesNotExist))
            }
        }
    }

    pub fn depth(&self)->Depth{
        let bids=self.get_bids();
        let asks=self.get_asks();
        Depth { bids, asks }
    }

    pub fn get_bids(&self) -> Vec<Order>{
        let mut bids:Vec<Order> = Vec::new();
        if self.is_bids_empty(){
            return bids
        }
        for (price,orders) in self.bids.iter(){
            bids.push(Order{price:price.0,quantity:orders.iter().map(|v|v.quantity-v.quantity_filled).sum(),order_count:orders.iter().count() as u64})
        }

        bids
    }

    pub fn get_asks(&self) -> Vec<Order>{
        let mut asks:Vec<Order> = Vec::new();
        if self.is_asks_empty(){
            return asks;
        }
        for (price,orders) in self.asks.iter(){
            asks.push(Order{price:*price,quantity:orders.iter().map(|v|v.quantity-v.quantity_filled).sum(),order_count:orders.iter().count() as u64})
        }
        asks
    }

    pub fn delete_order(&mut self,order_id:u64)->Result<DeleteResponse,ErrorResponse>{
        let order=self.order_map.get(&order_id);
        if let Some(o)=order{
            let side=o.side.clone();
            let price = o.price;
            if o.quantity==o.quantity_filled{
                return Err(ErrorResponse::new(CustomError::OrderAlreadyMatched))
            }
            let response =match side{
                Side::Asks=>{
                    let open_orders=self.asks.get_mut(&price).unwrap();
                    let (price,quantity,quantity_filled)=open_orders
                        .iter()
                        .find(|v|v.order_id==order_id)
                        .map(|v|(v.price,v.quantity,v.quantity_filled))
                        .unwrap();
                    
                    open_orders.retain(|v|v.order_id!=order_id);
                    DeleteResponse::new(price, quantity, quantity_filled, order_id)
                },
                Side::Bids=>{
                    let open_orders=self.bids.get_mut(&Reverse(price)).unwrap();
                    let (price,quantity,quantity_filled)=open_orders
                        .iter()
                        .find(|v|v.order_id==order_id)
                        .map(|v|(v.price,v.quantity,v.quantity_filled))
                        .unwrap();
                    open_orders.retain(|v|v.order_id!=order_id);
                    DeleteResponse::new(price, quantity, quantity_filled, order_id)
                }
            };
            self.clear_empty_bids_or_asks();
            Ok(response)
        }else{
            let err = CustomError::OrderDoesNotExist;
            Err(ErrorResponse::new(err))
        }   
    }

    //TODO - MODIFY IN PLACE WITHOUT CHANGING THE TIME PRIORITY
    pub fn modify_order(&mut self,modify_order_request:ModifyOrderRequest)->Result<ModifyOrderResponse,ErrorResponse>{
        let order = self.order_map.get_mut(&modify_order_request.order_id);
        if let Some(o)=order{
            let side=o.side.clone();
            let price = o.price;
            if o.quantity==o.quantity_filled{
                return Err(ErrorResponse::new(CustomError::OrderAlreadyMatched));
            }
            let response =match side{
                Side::Asks=>{
                    let open_orders=self.asks.get_mut(&price).unwrap();
                    let open_order =open_orders
                        .iter_mut()
                        .find(|v|v.order_id==modify_order_request.order_id)
                        .unwrap();
                    match modify_order_request.price{
                        Some(x)=>{open_order.price=x},
                        None=>{}
                    }
                    match modify_order_request.quantity{
                        Some(x)=>{
                            if x >=open_order.quantity_filled{
                                open_order.quantity=x;
                            }
                            else{
                                return Err(ErrorResponse::new(CustomError::ModifyQuantityCannotBeLesserThanFilledQuantity))
                            }
                        },
                        None=>{}
                    }
                    let open_order =open_orders
                        .iter_mut()
                        .find(|v|v.order_id==modify_order_request.order_id)
                        .unwrap();
                    o.price=open_order.price;
                    o.quantity=open_order.quantity;
                    ModifyOrderResponse::new(open_order.price, open_order.quantity, open_order.order_id)
                },
                Side::Bids=>{
                    let open_orders=self.bids.get_mut(&Reverse(price)).unwrap();
                    let open_order =open_orders
                        .iter_mut()
                        .find(|v|v.order_id==modify_order_request.order_id)
                        .unwrap();
                    match modify_order_request.price{
                        Some(x)=>{open_order.price=x},
                        None=>{}
                    }
                    match modify_order_request.quantity{
                        Some(x)=>{
                            if x >=open_order.quantity_filled{
                                open_order.quantity=x;
                            }
                            else{
                                return Err(ErrorResponse::new(CustomError::ModifyQuantityCannotBeLesserThanFilledQuantity))
                            }
                        },
                        None=>{}
                    }
                    let open_order =open_orders
                        .iter_mut()
                        .find(|v|v.order_id==modify_order_request.order_id)
                        .unwrap();
                    o.price=open_order.price;
                    o.quantity=open_order.quantity;
                    ModifyOrderResponse::new(open_order.price, open_order.quantity, open_order.order_id)
                }
            };
            self.clear_empty_bids_or_asks();
            return Ok(response)
        }else{
            let err = CustomError::OrderDoesNotExist;
            Err(ErrorResponse::new(err))
        }
    }

    pub fn add_limit_order(&mut self,order: LimitOrder)->OpenOrder{
        self.order_id_index+=1;
        let order_id=self.order_id_index;
        let open_order=self.match_limit_order(order, order_id);
        self.order_map.insert(order_id, open_order.clone());
        self.clear_empty_bids_or_asks();
        open_order
    }
    
    //TODO - MATCHING IS DONE.NOW EXECUTE THE TRADES
    fn match_limit_order(&mut self,order: LimitOrder,order_id:u64)->OpenOrder{
        let price = order.price;
        let mut remaining_quantity=order.quantity;
        match order.side{
            Side::Asks=>{
                let option_best_bid = self.get_best_bid();
                let is_first_bids=self.is_bids_empty();
                if let Some(b)=option_best_bid{
                    let best_bid=b.0;
                    if !is_first_bids && price<best_bid {
                        let mut new_best_bid = self.get_best_bid().unwrap().0;
                        let mut bids = self.bids.iter_mut();
                        let mut price_array:Vec<Decimal>=Vec::new();
                        'outer: while remaining_quantity>dec!(0) && new_best_bid>=price{
                            if let Some(open_order)=bids.next(){
                                let open_orders = open_order.1;
                                let order_price=open_order.0.0;
                                let mut iter = open_orders.iter_mut();
                                let mut to_remove:Vec<u64> = Vec::new();
                                new_best_bid=order_price;
                                if new_best_bid<price{
                                    break 'outer;
                                }
                                while remaining_quantity>dec!(0){
                                    if let Some(o) =iter.next(){
                                        let quantity_remaining=o.quantity-o.quantity_filled;
                                        if remaining_quantity>=quantity_remaining{
                                            remaining_quantity-=quantity_remaining;
                                            let order_id=o.order_id;
                                            let order_map_order=self.order_map.get_mut(&order_id).unwrap();
                                            order_map_order.quantity_filled=order_map_order.quantity;
                                            to_remove.push(o.order_id);
                                        }else{
                                            o.quantity_filled+=remaining_quantity;
                                            let order_id=o.order_id;
                                            let order_map_order=self.order_map.get_mut(&order_id).unwrap();
                                            order_map_order.quantity_filled+=remaining_quantity;
                                            remaining_quantity=dec!(0);
                                        }
                                        price_array.push(o.price); 
                                    }else{
                                        break;
                                    }
                                }
                                open_orders.retain(|v|!to_remove.contains(&v.order_id));
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
                                let order_id=o.order_id;
                                let order_map_order=self.order_map.get_mut(&order_id).unwrap();
                                order_map_order.quantity_filled=order_map_order.quantity;
                                to_remove.push(o.order_id);
                            }else{
                                o.quantity_filled+=remaining_quantity;
                                let order_id=o.order_id;
                                let order_map_order=self.order_map.get_mut(&order_id).unwrap();
                                order_map_order.quantity_filled+=remaining_quantity;
                                remaining_quantity=dec!(0);
                            }
                        }
                        //if there is no element in the iterator do this 
                        else{
                            break;
                        }
                    }
                    open_orders.retain(|v|!to_remove.contains(&v.order_id));
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
                let option_best_ask = self.get_best_ask();
                let is_first_ask=self.is_asks_empty();
                if let Some(b)=option_best_ask{
                    let best_ask=b.to_owned();
                    if !is_first_ask && price>best_ask{
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
                                            let order_id=o.order_id;
                                            let order_map_order=self.order_map.get_mut(&order_id).unwrap();
                                            order_map_order.quantity_filled=order_map_order.quantity;
                                            to_remove.push(o.order_id);
                                            // open_orders.pop_front();
                                        }else{
                                            o.quantity_filled+=remaining_quantity;
                                            let order_id=o.order_id;
                                            let order_map_order=self.order_map.get_mut(&order_id).unwrap();
                                            order_map_order.quantity_filled+=remaining_quantity;
                                            remaining_quantity=dec!(0);
                                        }
                                        price_array.push(o.price); 
                                    }else{
                                        break;
                                    }
                                }
                                open_orders.retain(|v|!to_remove.contains(&v.order_id));
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
                                let order_id=o.order_id;
                                let order_map_order=self.order_map.get_mut(&order_id).unwrap();
                                order_map_order.quantity_filled=order_map_order.quantity;
                                to_remove.push(o.order_id);
                            }else{
                                o.quantity_filled+=remaining_quantity;
                                let order_id=o.order_id;
                                let order_map_order=self.order_map.get_mut(&order_id).unwrap();
                                order_map_order.quantity_filled+=remaining_quantity;
                                remaining_quantity=dec!(0);
                            }
                        }
                        //if there is no element in the iterator do this 
                        else{
                            break;
                        }
                    }
                    open_orders.retain(|v|!to_remove.contains(&v.order_id));
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
        let response =self.match_market_order(remaining_quantity, order);
        self.clear_empty_bids_or_asks();
        response
    }

    fn match_market_order(&mut self,mut remaining_quantity:Decimal,order:MarketOrder)->MarketOrderResponse{
        match order.side{
            Side::Asks=>{
                if self.is_bids_empty(){
                    return MarketOrderResponse::new(false, None, None,Some(CustomError::LimitOrderDoesNotExist))
                }
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
                        
                        open_orders.retain(|v|!to_remove.contains(&v.order_id));
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
                let market_order_response = MarketOrderResponse::new(true, Some(average_price), Some(quantity_filled),None);
                return market_order_response;
            },
            Side::Bids=>{
                if self.is_asks_empty(){
                    return MarketOrderResponse::new(false, None, None, Some(CustomError::LimitOrderDoesNotExist));
                }
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
                let market_order_response = MarketOrderResponse::new(true, Some(average_price), Some(quantity_filled),None);
                return market_order_response;
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::{Orderbook,Reverse,dec,VecDeque};
    
    #[cfg(test)]
    use pretty_assertions::{assert_eq};
    #[test]
    fn test_clear_empty_bids_or_asks(){
        let mut orderbook =Orderbook::new();

        orderbook.asks.insert(dec!(110), VecDeque::new());
        assert_eq!(orderbook.asks.get(&dec!(110)).unwrap(),&VecDeque::new());
        orderbook.clear_empty_bids_or_asks();
        assert_eq!(orderbook.asks.get(&dec!(110)),None);
    
        orderbook.bids.insert(Reverse(dec!(110)), VecDeque::new());
        assert_eq!(orderbook.bids.get(&Reverse(dec!(110))).unwrap(),&VecDeque::new());
        orderbook.clear_empty_bids_or_asks();
        assert_eq!(orderbook.bids.get(&Reverse(dec!(110))),None);
    }

}