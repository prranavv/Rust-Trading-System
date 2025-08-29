# Trading Engine Library

A multi-market trading engine library built in Rust that manages multiple orderbooks for different trading pairs. This library provides a high-level interface for creating markets, managing orders across different trading pairs, and retrieving market data.

## Overview

The Trading Engine acts as a management layer over multiple orderbooks, allowing you to:
- Create and manage multiple trading markets (trading pairs)
- Route orders to the appropriate orderbook
- Ensure market existence validation before operations
- Provide unified error handling across all markets

## Features

- **Multi-Market Support**: Manage unlimited trading pairs in a single engine instance
- **Market Management**: Create new markets dynamically
- **Order Operations**: 
  - Place limit and market orders
  - Modify existing orders
  - Cancel orders
  - Query order status
- **Market Data**:
  - Market depth for each trading pair
  - Mid-price calculations
  - List all available markets
- **Safety**: All operations validate market existence before execution

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
trading_engine = { path = "./trading_engine" }
orderbook = { path = "./orderbook" }
rust_decimal = "1.35"
serde = { version = "1.0", features = ["derive"] }
```

## Quick Start

```rust
use trading_engine::{TradingEngine, TradingPair};
use orderbook::{LimitOrder, MarketOrder, Side};
use rust_decimal::dec;

fn main() {
    // Initialize the trading engine
    let mut engine = TradingEngine::new();
    
    // Create a new market
    let btc_usd = TradingPair::new(
        "BTC".to_string(), 
        "USD".to_string()
    );
    engine.create_market(btc_usd.clone()).unwrap();
    
    // Place a limit order
    let order = LimitOrder {
        price: dec!(50000),
        quantity: dec!(0.5),
        side: Side::Bids,
        user_id: 1,
    };
    
    let result = engine.add_limit_order_into_market(btc_usd, order);
    match result {
        Ok(open_order) => println!("Order placed: {:?}", open_order),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

## Core Types

### TradingEngine
```rust
pub struct TradingEngine {
    pub orderbooks: HashMap<TradingPair, Orderbook>
}
```
The main engine that manages multiple orderbooks indexed by trading pair.

### TradingPair
```rust
pub struct TradingPair {
    pub base: String,   // Base currency (e.g., "BTC")
    pub quote: String,  // Quote currency (e.g., "USD")
}
```
Represents a trading pair/market identifier.

### TradingEngineError
```rust
pub enum TradingEngineError {
    TradingPairDoesNotExist,
    TradingPairAlreadyExists,
}
```
Engine-specific errors for market validation.

### Markets
```rust
pub struct Markets {
    pub markets: Vec<TradingPair>
}
```
Container for listing all available trading pairs.

## API Reference

### Engine Initialization
```rust
let mut engine = TradingEngine::new();
```
Creates a new trading engine with no markets.

### Market Management

#### Create Market
```rust
pub fn create_market(&mut self, trading_pair: TradingPair) 
    -> Result<(), TradingEngineError>
```
Creates a new market for the specified trading pair.

Example:
```rust
let eth_usd = TradingPair::new("ETH".to_string(), "USD".to_string());
engine.create_market(eth_usd)?;
```

#### Get Markets
```rust
pub fn get_markets(&self) -> Markets
```
Returns all available trading pairs.

### Order Operations

#### Add Limit Order
```rust
pub fn add_limit_order_into_market(
    &mut self, 
    trading_pair: TradingPair, 
    order: LimitOrder
) -> Result<OpenOrder, TradingEngineError>
```
Places a limit order in the specified market.

#### Add Market Order
```rust
pub fn add_market_order_into_market(
    &mut self, 
    trading_pair: TradingPair, 
    order: MarketOrder
) -> Result<MarketOrderResponse, TradingEngineError>
```
Executes a market order in the specified market.

#### Modify Order
```rust
pub fn modify_order_for_market(
    &mut self, 
    trading_pair: TradingPair, 
    order: ModifyOrderRequest
) -> Result<Result<ModifyOrderResponse, ErrorResponse>, TradingEngineError>
```
Modifies an existing order in the specified market.

Note: Returns nested Result - outer for market validation, inner for order operation.

#### Delete Order
```rust
pub fn delete_order_for_market(
    &mut self, 
    trading_pair: TradingPair, 
    order_id: u64
) -> Result<Result<DeleteResponse, ErrorResponse>, TradingEngineError>
```
Cancels an order in the specified market.

#### Get Order
```rust
pub fn get_order_by_id_for_market(
    &mut self, 
    trading_pair: TradingPair, 
    order_id: u64
) -> Result<Result<OpenOrder, ErrorResponse>, TradingEngineError>
```
Retrieves order details from the specified market.

### Market Data

#### Get Market Depth
```rust
pub fn get_market_depth(
    &mut self, 
    trading_pair: TradingPair
) -> Result<Depth, TradingEngineError>
```
Returns the current orderbook depth for a trading pair.

#### Get Mid Price
```rust
pub fn get_mid_price_for_market(
    &mut self, 
    trading_pair: TradingPair
) -> Result<Option<Decimal>, TradingEngineError>
```
Calculates the mid-price for a trading pair. Returns None if no bids or asks exist.

## Usage Examples

### Creating Multiple Markets
```rust
use trading_engine::{TradingEngine, TradingPair};

let mut engine = TradingEngine::new();

// Create multiple markets
let pairs = vec![
    ("BTC", "USD"),
    ("ETH", "USD"),
    ("BTC", "EUR"),
    ("SOL", "USDT"),
];

for (base, quote) in pairs {
    let pair = TradingPair::new(base.to_string(), quote.to_string());
    engine.create_market(pair).unwrap();
}

// List all markets
let markets = engine.get_markets();
println!("Available markets: {:?}", markets.markets);
```

### Cross-Market Operations
```rust
use trading_engine::{TradingEngine, TradingPair};
use orderbook::{LimitOrder, Side};
use rust_decimal::dec;

let mut engine = TradingEngine::new();

// Create markets
let btc_usd = TradingPair::new("BTC".to_string(), "USD".to_string());
let eth_usd = TradingPair::new("ETH".to_string(), "USD".to_string());

engine.create_market(btc_usd.clone()).unwrap();
engine.create_market(eth_usd.clone()).unwrap();

// Place orders in different markets
let btc_order = LimitOrder {
    price: dec!(50000),
    quantity: dec!(1),
    side: Side::Bids,
    user_id: 1,
};

let eth_order = LimitOrder {
    price: dec!(3000),
    quantity: dec!(10),
    side: Side::Asks,
    user_id: 2,
};

engine.add_limit_order_into_market(btc_usd.clone(), btc_order).unwrap();
engine.add_limit_order_into_market(eth_usd.clone(), eth_order).unwrap();

// Get depth for each market
let btc_depth = engine.get_market_depth(btc_usd).unwrap();
let eth_depth = engine.get_market_depth(eth_usd).unwrap();
```

### Error Handling
```rust
use trading_engine::{TradingEngine, TradingPair, TradingEngineError};

let mut engine = TradingEngine::new();
let pair = TradingPair::new("XYZ".to_string(), "USD".to_string());

// Attempt to get depth for non-existent market
match engine.get_market_depth(pair.clone()) {
    Ok(depth) => println!("Depth: {:?}", depth),
    Err(TradingEngineError::TradingPairDoesNotExist) => {
        println!("Market doesn't exist, creating it...");
        engine.create_market(pair).unwrap();
    }
    Err(e) => println!("Other error: {:?}", e),
}
```

## Architecture

```
trading_engine/
├── lib.rs              # Public API exports
├── mod.rs              # Module declarations
├── trading_engine.rs   # Core implementation
└── types.rs           # Type definitions
```

The trading engine maintains a HashMap of orderbooks, with each TradingPair as the key. All public methods:
1. Validate that the market exists (or doesn't exist for creation)
2. Delegate to the appropriate orderbook
3. Wrap responses with engine-level error handling

## Thread Safety

Like the underlying orderbook, the TradingEngine is not thread-safe by default. For concurrent access:

```rust
use std::sync::{Arc, Mutex};
use trading_engine::TradingEngine;

let engine = Arc::new(Mutex::new(TradingEngine::new()));

// In threads
let engine_clone = engine.clone();
thread::spawn(move || {
    let mut engine = engine_clone.lock().unwrap();
    // Use engine...
});
```

## Performance Considerations

- **Market Lookup**: O(1) average case using HashMap
- **Market Creation**: O(1) amortized
- **Order Operations**: Inherits performance from underlying orderbook
- **Get Markets**: O(n) where n is number of markets

## Testing

Run tests with:
```bash
cargo test
```

## Integration with Orderbook

The Trading Engine is designed to work seamlessly with the orderbook library. Each market maintains its own independent orderbook instance, ensuring:
- Order IDs are unique within each market
- Price-time priority is maintained per market
- Market operations don't affect other markets

## Best Practices

1. **Market Creation**: Create all markets at startup when possible
2. **Error Handling**: Always handle `TradingPairDoesNotExist` errors
3. **Market Validation**: Use `get_markets()` to validate trading pairs before operations
4. **Consistent Naming**: Establish conventions for trading pair strings (e.g., always uppercase)

## Limitations

- No cross-market order routing
- No automatic market maker functionality
- Markets cannot be deleted once created
- No persistence - all data is in-memory

## Future Enhancements

- [ ] Delete/suspend market functionality
- [ ] Cross-market analytics
- [ ] Market statistics and metrics
- [ ] Trading pair aliasing
- [ ] Market configuration (tick size, lot size, etc.)
- [ ] Event streaming for market updates
- [ ] Persistence layer integration
- [ ] Market hours/scheduling support

## Dependencies

- `orderbook`: Core orderbook implementation
- `rust_decimal`: Decimal arithmetic
- `serde`: Serialization support
- `std::collections::HashMap`: Market storage

## License

[Specify your license here]

## See Also

- [Orderbook Library](../orderbook/README.md) - Underlying orderbook implementation
- [Trading Server](../server/README.md) - HTTP API server using this engine