use std::collections::HashSet;
use crate::account::order::Order;
use crate::candles::candle::Candle;
use crate::strategies::strategy::Strategy;
use crate::strategies::strategy_monitor::StrategyMonitor;

#[derive(Copy, Clone)]
pub enum TradeSide {
    LONG,
    SHORT,
}

pub struct Trade {
    pub exit_reason: String,

    pub position: HashSet<Order>,
    pub exit_orders: HashSet<Order>,

    average_price: f64,

    total_unspent: f64,
    total_units: f64,

    ticks: u64,

    max: f64,
    min: f64,
    min_change: f64,
    max_change: f64,
    change: f64,

    first_candle: Candle,
    opening_strategy: Box<dyn Strategy>,

    stopped: bool,

    finalized_quantity: f64,

    actual_profit_loss: f64,
    actual_profit_loss_pct: f64,

    strategy_monitors: Vec<Box<dyn StrategyMonitor>>,
}
