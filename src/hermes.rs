use crate::market::Market;
use crate::strategy::Strategy;

struct Portfolio {
    balance: f64,
}

pub struct Risk {
    max_position: f64,
    max_loss: f64,
}

pub struct Config {
    symbol: String,
    risk: Risk,
}

pub struct Hermes;

impl Hermes {
    pub fn new() -> Self {
        Hermes
    }

    pub fn trade(&self, _m: impl Market, _s: impl Strategy) {
        todo!();
    }
}