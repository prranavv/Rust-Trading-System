# Orderbook Library

A high-performance order matching engine library written in Rust, providing core orderbook functionality for trading systems with support for limit and market orders, order modifications, and real-time market depth.

## Features

- **Efficient Order Matching**: Price-time priority matching algorithm
- **Order Types**:
  - Limit Orders with partial fills
  - Market Orders with immediate execution
- **Order Management**:
  - Add, modify, and delete orders
  - Track partially filled orders
  - Order status queries
- **Market Data**:
  - Real-time bid/ask depth
  - Best bid/ask prices
  - Spread calculation
  - Mid-price calculation
- **Data Structures**:
  - BTreeMap for sorted price levels
  - VecDeque for time-priority at each price level
  - HashMap for O(1) order lookups

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
orderbook = { path = "./orderbook" }
rust_decimal = "1.35"
serde = { version = "1.0", features = ["derive"] }
```

## Quick Start

```rust
use orderbook::{Orderbook, LimitOrder, MarketOrder, Side};
use rust_decimal::dec;

fn main() {
    // Create a new orderbook
    let mut orderbook = Orderbook::new();
    
    // Add a limit buy order
    let buy_order = LimitOrder {
        price: dec!(100.50),
        quantity: dec!(10),
        side: Side::Bids,
        user_id: 1,
    };
    let open_order = orderbook.add_limit_order(buy_order);
    println!("Order placed with ID: {}", open_order.order_id);
    
    // Add a market sell order
    let sell_order = MarketOrder {
        quantity: dec!(5),
        side: Side::Asks,
        user_id: 2,
    };
    let result = orderbook.add_market_order(sell_order);
    println!("Market order executed: {:?}", result);
}
```

## Core Types

### Orderbook
The main structure managing all orders:
```rust
pub struct Orderbook {
    pub bids: BTreeMap<Reverse<Decimal>, VecDeque<OpenOrder>>,
    pub asks: BTreeMap<Decimal, VecDeque<OpenOrder>>,
    pub order_id_index: u64,
    pub order_map: HashMap<u64, OpenOrder>,
}
```

### Order Types

#### LimitOrder
```rust
pub struct LimitOrder {
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
    pub user_id: u64,
}
```

#### MarketOrder
```rust
pub struct MarketOrder {
    pub quantity: Decimal,
    pub side: Side,
    pub user_id: u64,
}
```

#### OpenOrder
Represents an order in the book:
```rust
pub struct OpenOrder {
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
    pub quantity_filled: Decimal,
    pub user_id: u64,
    pub order_id: u64,
}
```

### Side Enum
```rust
pub enum Side {
    Asks,  // Sell orders
    Bids,  // Buy orders
}
```

## API Reference

### Creating an Orderbook
```rust
let mut orderbook = Orderbook::new();
```

### Adding Orders

#### Add Limit Order
```rust
pub fn add_limit_order(&mut self, order: LimitOrder) -> OpenOrder
```
Adds a limit order to the book. Returns an `OpenOrder` with assigned ID and fill information.

#### Add Market Order
```rust
pub fn add_market_order(&mut self, order: MarketOrder) -> MarketOrderResponse
```
Executes a market order immediately against available liquidity.

### Order Management

#### Modify Order
```rust
pub fn modify_order(&mut self, request: ModifyOrderRequest) 
    -> Result<ModifyOrderResponse, ErrorResponse>
```
Modifies price and/or quantity of an existing order.

Example:
```rust
let modify_request = ModifyOrderRequest {
    order_id: 123,
    price: Some(dec!(101.00)),
    quantity: Some(dec!(15)),
};
let result = orderbook.modify_order(modify_request);
```

#### Delete Order
```rust
pub fn delete_order(&mut self, order_id: u64) 
    -> Result<DeleteResponse, ErrorResponse>
```
Removes an order from the book.

#### Get Order
```rust
pub fn get_order(&self, order_id: u64) 
    -> Result<OpenOrder, ErrorResponse>
```
Retrieves order details by ID.

### Market Data

#### Get Depth
```rust
pub fn get_depth(&self) -> Depth
```
Returns current market depth with aggregated quantities at each price level.

#### Get Best Prices
```rust
pub fn get_best_bid(&self) -> Option<&Reverse<Decimal>>
pub fn get_best_ask(&self) -> Option<&Decimal>
```

#### Calculate Spread
```rust
pub fn get_spread(&self) -> Option<Decimal>
```
Returns the difference between best ask and best bid.

#### Calculate Mid Price
```rust
pub fn mid_price(&self) -> Option<Decimal>
```
Returns the average of best bid and ask prices.

## Error Handling

The library uses a custom error enum for various failure scenarios:

```rust
pub enum CustomError {
    OrderDoesNotExist,
    ModifyQuantityCannotBeLesserThanFilledQuantity,
    LimitOrderDoesNotExist,
    OrderAlreadyMatched,
}
```

## Order Matching Algorithm

The orderbook implements price-time priority matching:

1. **Limit Orders**: 
   - Incoming orders check for crosses with opposite side
   - Matches at best available prices first
   - Partial fills are supported
   - Unmatched quantity rests in the book

2. **Market Orders**:
   - Execute immediately against available liquidity
   - Walk through price levels until filled
   - Return average execution price
   - Fail if insufficient liquidity

## Performance Characteristics

- **Add Order**: O(log n) for price level lookup + O(1) for insertion
- **Cancel Order**: O(log n) for price level lookup + O(m) where m is orders at that price
- **Modify Order**: O(log n) + O(m) similar to cancel
- **Best Bid/Ask**: O(1) using BTreeMap properties
- **Market Order Execution**: O(k) where k is number of orders to match

## Thread Safety

The orderbook itself is not thread-safe. For concurrent access, wrap it in appropriate synchronization primitives:

```rust
use std::sync::{Arc, Mutex};

let orderbook = Arc::new(Mutex::new(Orderbook::new()));
```

## Example: Building a Simple Trading System

```rust
use orderbook::{Orderbook, LimitOrder, Side, ModifyOrderRequest};
use rust_decimal::dec;

fn main() {
    let mut ob = Orderbook::new();
    
    // Add some liquidity
    let buy1 = LimitOrder {
        price: dec!(99.50),
        quantity: dec!(100),
        side: Side::Bids,
        user_id: 1,
    };
    ob.add_limit_order(buy1);
    
    let sell1 = LimitOrder {
        price: dec!(100.50),
        quantity: dec!(100),
        side: Side::Asks,
        user_id: 2,
    };
    ob.add_limit_order(sell1);
    
    // Check the spread
    if let Some(spread) = ob.get_spread() {
        println!("Current spread: {}", spread);
    }
    
    // Get market depth
    let depth = ob.get_depth();
    println!("Bids: {:?}", depth.bids);
    println!("Asks: {:?}", depth.asks);
    
    // Place a crossing limit order (will match)
    let aggressive_buy = LimitOrder {
        price: dec!(100.50),
        quantity: dec!(50),
        side: Side::Bids,
        user_id: 3,
    };
    let result = ob.add_limit_order(aggressive_buy);
    println!("Filled quantity: {}", result.quantity_filled);
}
```

## Testing

The library includes unit tests. Run them with:

```bash
cargo test
```

For verbose output:
```bash
cargo test -- --nocapture
```

## Dependencies

- `rust_decimal`: For precise decimal arithmetic (critical for financial calculations)
- `serde`: For serialization/deserialization support
- `pretty_assertions` (dev): For better test assertions

## Contributing

Contributions are welcome! Please ensure:
1. All tests pass
2. New features include tests
3. Code follows Rust conventions
4. Documentation is updated

## Safety Considerations

When using this library in production:

1. **Decimal Precision**: Always use `rust_decimal::Decimal` for prices and quantities to avoid floating-point errors
2. **Overflow Protection**: Consider maximum order sizes and price levels
3. **Order ID Management**: The library uses sequential IDs; consider UUID for distributed systems
4. **Audit Trail**: Implement logging for all order operations in production
5. **State Persistence**: The orderbook is in-memory; implement persistence for production use

## Roadmap

- [ ] Stop orders and stop-limit orders
- [ ] Iceberg orders
- [ ] Time-in-force conditions (IOC, FOK, GTC)
- [ ] Order expiration
- [ ] Fee calculation hooks
- [ ] WebSocket streaming for real-time updates
- [ ] Performance benchmarks
- [ ] Order history tracking
- [ ] Circuit breaker mechanisms
