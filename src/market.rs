use std::ops::Deref;

pub type OrderId = u64;

pub enum Signal {
    Buy,
    Sell,
    Hold,
}

pub struct Symbol(pub String);

impl Deref for Symbol {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Order {
    pub id: OrderId,
    pub quantity: f64,
    pub price: f64,
    pub symbol: Symbol,
}

struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    timestamp: u64,
}

pub trait Market {
    fn get_price(&self, symbol: &str) -> f64;
    fn place_buy_order(&self, order: Order) -> anyhow::Result<OrderId>;
    fn place_sell_order(&self, order: Order) -> anyhow::Result<OrderId>;
    fn cancel_order(&self, id: OrderId) -> anyhow::Result<()>;
    fn get_balance(&self) -> anyhow::Result<f64>;
}

pub struct MockMarket;

impl MockMarket {
    pub fn new() -> Self {
        MockMarket
    }
}

impl Market for MockMarket {
    fn get_price(&self, _symbol: &str) -> f64 {
        100.0
    }

    fn place_buy_order(&self, _order: Order) -> anyhow::Result<OrderId> {
        Ok(1)
    }

    fn place_sell_order(&self, _order: Order) -> anyhow::Result<OrderId> {
        Ok(1)
    }

    fn cancel_order(&self, _id: OrderId) -> anyhow::Result<()> {
        Ok(())
    }

    fn get_balance(&self) -> anyhow::Result<f64> {
        Ok(1000.0)
    }
}