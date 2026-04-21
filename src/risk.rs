use crate::market::{Order, Symbol, Signal};


pub struct Risk {
    max_position: f64,
    max_loss: f64,
}

pub enum Risky<T> {
    Fine(T),
    Risk,
    KillSwitch,
}

impl<T> Risky<T> {
    pub fn risk(self) -> Option<T> {
        match self {
            Risky::Fine(value) => Some(value),
            Risky::KillSwitch => {
                println!("Kill switch activated! Shutting down trading.");
                std::process::exit(1);
            },
            _ => None,
        }
    }
}

pub trait RiskManager {
    fn assess(&self, signal: Signal) -> Risky<Signal>;
}

pub struct MockRiskManager;

impl MockRiskManager {
    pub fn new() -> Self {
        MockRiskManager
    }
}

impl RiskManager for MockRiskManager {
    fn assess(&self, signal: Signal) -> Risky<Signal> {
        Risky::Fine(signal)
    }
}