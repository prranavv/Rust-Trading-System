use orderbook::{
    DeleteResponse, Depth, ErrorResponse, LimitOrder, MarketOrder, MarketOrderResponse, ModifyOrderRequest, ModifyOrderResponse, OpenOrder, Orderbook
};

struct TradingEngine{
    orderbook: Orderbook 
}

impl TradingEngine{
    fn new(orderbook: Orderbook)->TradingEngine{
        TradingEngine { orderbook }
    }
    //Add logic to handle the matching of limit order
    fn add_limit_order(&mut self,order: LimitOrder)->OpenOrder{
        self.orderbook.add_limit_order(order)
    }

    fn add_market_order(&mut self,order:MarketOrder)->MarketOrderResponse{
        self.orderbook.add_market_order(order)
    }

    fn get_depth(&self)->Depth{
        self.orderbook.depth()
    }

    fn delete_order(&mut self,order_id:u64)->Result<DeleteResponse,ErrorResponse>{
        self.orderbook.delete_order(order_id)
    }

    fn modify_order(&mut self,order:ModifyOrderRequest)->Result<ModifyOrderResponse,ErrorResponse>{
        self.orderbook.modify_order(order)
    }

    fn get_order_by_id(&self,order_id:u64)->Result<OpenOrder,ErrorResponse>{
        self.orderbook.get_order(order_id)
    }
}
 