use anyhow::Ok;

use crate::market::{Market, Symbol};
use crate::strategy::Strategy;
use crate::risk::{Risk, RiskManager};

struct Portfolio {
    balance: f64,
}

pub struct Config {
    symbol: Symbol,
    risk: Risk,
}

pub struct Hermes<M: Market, S: Strategy, RM: RiskManager> {
    market: M,
    strategy: S,
    risk_manager: RM,
    config: Option<Config>,
}

impl<M: Market, S: Strategy, RM: RiskManager> Hermes<M, S, RM> {
    pub fn new(risk_manager: RM, config: Option<Config>, market: M, strategy: S) -> Self {
        Hermes {
            market,
            strategy,
            risk_manager,
            config,
        }
    }

    pub fn trade(&self) -> anyhow::Result<()> {
        let config = self.config.as_ref().unwrap();
        let price = self.market.get_price(&config.symbol);
        let signal = self.strategy.on_price(price);

        if let Some(order) = self.risk_manager.assess(signal).risk() {
            // Handle the order
        }

        Ok(())
    }
}