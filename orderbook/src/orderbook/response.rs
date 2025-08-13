use rust_decimal::Decimal;

pub enum CustomError{
    OrderDoesNotExist,
    ModifyQuantityCannotBeLesserThanFilledQuantity
}

#[derive(Debug)]
pub struct MarketOrderResponse{
    success: bool,
    average_price: Decimal,
    quantity:Decimal
}

pub struct ErrorResponse{
    error:CustomError
}

pub struct DeleteResponse{
    success: bool,
    price:Decimal,
    quantity:Decimal,
    quantity_filled:Decimal,
    order_id:u64
}

pub struct ModifyOrderResponse{
    pub success:bool,
    pub price:Decimal,
    pub quantity: Decimal,
    pub order_id:u64
}


impl MarketOrderResponse{
    pub fn new(success: bool,average_price:Decimal,quantity:Decimal)->MarketOrderResponse{
        MarketOrderResponse { success, average_price, quantity }
    }
}

impl ModifyOrderResponse{
    pub fn new(price:Decimal,quantity:Decimal,order_id:u64)->ModifyOrderResponse{
        ModifyOrderResponse { success: true, price, quantity, order_id }
    }
}

impl ErrorResponse{
    pub fn new(err: CustomError)->ErrorResponse{
        ErrorResponse { error:err}
    }
}

impl DeleteResponse{
    pub fn new(price:Decimal,quantity:Decimal,quantity_filled:Decimal,order_id:u64)->DeleteResponse{
        DeleteResponse { success:true,price,quantity, quantity_filled, order_id }
    }
}