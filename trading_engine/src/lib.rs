use orderbook::{
    Depth, LimitOrder, MarketOrder, MarketOrderResponse, OpenOrder, Orderbook
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
}
 