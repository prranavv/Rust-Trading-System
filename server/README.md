# Trading Server

A high-performance trading server built with Rust, featuring a robust order book implementation and trading engine for managing multiple trading pairs and order types.

## 🚀 Features

- **Multiple Trading Pairs**: Support for creating and managing multiple markets
- **Order Types**: 
  - Limit Orders
  - Market Orders
- **Order Management**:
  - Place orders
  - Modify existing orders
  - Cancel orders
  - Query order status
- **Market Data**:
  - Real-time market depth
  - Mid-price calculation
  - Order book visualization
- **RESTful API**: Clean HTTP endpoints for all trading operations
- **Thread-Safe**: Concurrent request handling with Arc<Mutex> pattern
- **Built with Axum**: Modern, ergonomic web framework for Rust

## 📋 Prerequisites

- Rust 1.70+ 
- Cargo (comes with Rust)
- tokio runtime
- axum web framework

## 🛠️ Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/trading-server.git
cd trading-server
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run --release
```

The server will start on `http://0.0.0.0:8000`

## 📡 API Endpoints

### Market Management

#### Create Market
- **POST** `/api/v1/create-market`
- Creates a new trading pair/market

Request body:
```json
{
  "trading_pair": {
    "base": "BTC",
    "quote": "USD"
  }
}
```

Response:
```json
{
  "created": true,
  "trading_pair": {
    "base": "BTC",
    "quote": "USD"
  }
}
```

#### Get Markets
- **GET** `/api/v1/get-market`
- Returns all available trading pairs

Response:
```json
{
  "markets": [
    {
      "base": "BTC",
      "quote": "USD"
    },
    {
      "base": "ETH",
      "quote": "USD"
    }
  ]
}
```

### Order Operations

#### Place Limit Order
- **POST** `/api/v1/limit-order`
- Places a new limit order

Request body:
```json
{
  "trading_pair": {
    "base": "BTC",
    "quote": "USD"
  },
  "order": {
    "price": "50000.00",
    "quantity": "0.5",
    "side": "Buy"
  }
}
```

Response:
```json
{
  "open_order": {
    "id": 12345,
    "price": "50000.00",
    "quantity": "0.5",
    "side": "Buy",
    "timestamp": 1234567890
  },
  "error": null
}
```

#### Place Market Order
- **POST** `/api/v1/market-order`
- Places a market order for immediate execution

Request body:
```json
{
  "trading_pair": {
    "base": "BTC",
    "quote": "USD"
  },
  "order": {
    "quantity": "0.5",
    "side": "Buy"
  }
}
```

Response:
```json
{
  "response": {
    "executed_quantity": "0.5",
    "average_price": "50250.00",
    "trades": [...]
  },
  "error": null
}
```

#### Modify Order
- **POST** `/api/v1/modify-order`
- Modifies an existing order

Request body:
```json
{
  "trading_pair": {
    "base": "BTC",
    "quote": "USD"
  },
  "order_request": {
    "order_id": 12345,
    "new_price": "51000.00",
    "new_quantity": "0.6"
  }
}
```

#### Delete Order
- **DELETE** `/api/v1/delete-order`
- Cancels an existing order

Request body:
```json
{
  "trading_pair": {
    "base": "BTC",
    "quote": "USD"
  },
  "order_id": 12345
}
```

#### Get Order
- **GET** `/api/v1/get-order`
- Retrieves details of a specific order

Request body:
```json
{
  "trading_pair": {
    "base": "BTC",
    "quote": "USD"
  },
  "order_id": 12345
}
```

### Market Data

#### Get Market Depth
- **GET** `/api/v1/depth`
- Returns the order book depth for a trading pair

Request body:
```json
{
  "trading_pair": {
    "base": "BTC",
    "quote": "USD"
  }
}
```

Response:
```json
{
  "depth": {
    "bids": [
      {"price": "49900.00", "quantity": "1.5"},
      {"price": "49850.00", "quantity": "2.0"}
    ],
    "asks": [
      {"price": "50100.00", "quantity": "1.2"},
      {"price": "50150.00", "quantity": "1.8"}
    ]
  },
  "error": null
}
```

#### Get Mid Price
- **GET** `/api/v1/mid-price`
- Returns the mid-price for a trading pair

Request body:
```json
{
  "trading_pair": {
    "base": "BTC",
    "quote": "USD"
  }
}
```

Response:
```json
{
  "price": "50000.00",
  "error": null
}
```

## 🔧 Configuration

The server runs on port 8000 by default. To change this, modify the bind address in `main.rs`:

```rust
let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
```

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

Run with verbose output:
```bash
cargo test -- --nocapture
```

## 📊 Performance

The trading server is designed for high performance:
- Async/await with Tokio for concurrent request handling
- Arc<Mutex> for thread-safe state management
- Efficient order matching algorithms
- Minimal allocations in hot paths

## 🔒 Thread Safety

The trading engine is wrapped in `Arc<Mutex<TradingEngine>>` to ensure thread-safe access across multiple concurrent requests. Each request handler acquires the lock, performs its operation, and releases it immediately.

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## ⚠️ Disclaimer

This is a trading server implementation for educational and development purposes. Please ensure proper testing, security audits, and regulatory compliance before using in production environments.

## 🚦 Status Codes

- `200 OK`: Successful GET request
- `201 Created`: Successfully created resource (orders, markets)
- `400 Bad Request`: Invalid request or operation failed
- `500 Internal Server Error`: Server-side error

## 🔮 Future Enhancements

- [ ] WebSocket support for real-time updates
- [ ] Authentication and authorization
- [ ] Order history and trade logs
- [ ] Advanced order types (stop-loss, take-profit)
- [ ] Performance metrics and monitoring
- [ ] Database persistence
- [ ] Horizontal scaling support
- [ ] Market maker integration
- [ ] FIX protocol support

## 📧 Contact

For questions and support, please open an issue on GitHub or contact the maintainers.

---

Built with ❤️ in Rust
