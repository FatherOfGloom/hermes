mod hermes;
mod market;
mod strategy;

use crate::hermes::Hermes;
use crate::market::MockMarket;
use crate::strategy::MockStrategy;

fn main() {
    let hermes = Hermes::new();
    let market = MockMarket::new();
    let strategy = MockStrategy::new();

    hermes.trade(market, strategy);
}