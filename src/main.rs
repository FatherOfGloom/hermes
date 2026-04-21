mod hermes;
mod market;
mod strategy;
mod risk;

use crate::hermes::Hermes;
use crate::market::MockMarket;
use crate::risk::MockRiskManager;
use crate::strategy::MockStrategy;

fn main() {
    let market = MockMarket::new();
    let strategy = MockStrategy::new();
    let risk_manager = MockRiskManager::new();
    Hermes::new(risk_manager, None, market, strategy).trade().unwrap();
}