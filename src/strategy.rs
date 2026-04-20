use crate::market::Signal;
use std::sync::mpsc::Sender;

pub trait Strategy {
    fn on_price(&self, price: f64) -> Signal;
}

pub struct MockStrategy;

impl MockStrategy {
    pub fn new() -> Self {
        MockStrategy
    }
}

impl Strategy for MockStrategy {
    fn on_price(&self, _price: f64) -> Signal {
        Signal::Hold
    }
}