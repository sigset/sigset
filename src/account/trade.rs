use std::collections::HashSet;
use crate::account::order::{Order, OrderSide};
use crate::account::trader::Trader;
use crate::candles::candle::Candle;
use crate::strategies::strategy::Strategy;
use crate::strategies::strategy_monitor::StrategyMonitor;

#[derive(Copy, Clone)]
pub enum TradeSide {
    LONG,
    SHORT,
}

pub fn positive_price_change_ratio(
    spent: f64,
    current_price: f64,
) -> f64 {
    (current_price / spent) - 1.0
}

pub fn negative_price_change_ratio(
    spent: f64,
    current_price: f64,
) -> f64 {
    1.0 - (current_price / spent)
}

#[derive(Clone, Debug)]
pub struct Trade<'a> {
    id: u64,

    pub exit_reason: Option<String>,

    pub position: HashSet<Order>,
    pub exit_orders: HashSet<Order>,

    pub is_placeholder: bool,

    pub monitors: Vec<Box<dyn StrategyMonitor>>,
    pub trader: &'a Trader,

    average_price: f64,

    total_unspent: f64,
    total_units: f64,

    ticks: u64,

    max: f64,
    min: f64,
    min_change: f64,
    max_change: f64,
    change: f64,

    first_candle: Option<Candle>,
    opening_strategy: Option<Box<dyn Strategy>>,

    stopped: bool,

    finalized: bool,
    finalized_quantity: f64,

    actual_profit_loss: f64,
    actual_profit_loss_pct: f64,

    side: OrderSide,

    strategy_monitors: Vec<Box<dyn StrategyMonitor>>,
}

impl<'a> Trade<'a> {
    pub fn new_from_order(
        id: u64,
        opening_order: Order,
        trader: &'a Trader,
        opening_strategy: Option<Box<dyn Strategy>>,
    ) -> Trade {
        let side =
            if opening_order.side == OrderSide::Buy {
                OrderSide::Sell
            } else {
                OrderSide::Buy
            };

        let trade = Trade::new(
            id,
            trader,
            side,
            opening_strategy,
            // todo fix
            Vec::new(),
            false,
        );

        // trade.increase_position(id, opening_order, trader);

        trade
    }

    pub fn new(
        id: u64,
        trader: &'a Trader,
        side: OrderSide,
        opening_strategy: Option<Box<dyn Strategy>>,
        monitors: Vec<Box<dyn StrategyMonitor>>,
        is_placeholder: bool,
    ) -> Trade {
        let trade = Trade {
            id,
            exit_reason: None,
            position: HashSet::new(),
            exit_orders: HashSet::new(),
            is_placeholder,
            monitors,
            trader,
            average_price: 0.0,
            total_unspent: 0.0,
            total_units: 0.0,
            ticks: 0,
            max: 0.0,
            min: 0.0,
            min_change: 0.0,
            max_change: 0.0,
            change: 0.0,
            first_candle: None,
            opening_strategy,
            stopped: false,
            finalized: false,
            finalized_quantity: 0.0,
            actual_profit_loss: 0.0,
            actual_profit_loss_pct: 0.0,
            side,
            strategy_monitors: Vec::new(),
        };

        // trade.init_trade();

        trade
    }

    pub fn create_placeholder(
        id: u64,
        trader: &'a Trader,
        side: OrderSide,
    ) -> Trade {
        Trade::new(
            id,
            trader,
            side,
            None,
            Vec::new(),
            true,
        )
    }

    pub fn init_trade(&mut self) {
        //self.first_candle = Some(self.trader.latest_candle());
        //self.max = self.first_candle.unwrap().close;
        //self.min = self.first_candle.unwrap().close;

        self.finalized = false;
    }
}