use crate::account::trade::Trade;
use crate::strategies::strategy::Strategy;
use crate::strategies::indicator_group::IndicatorGroup;
use crate::account::order::Order;
use crate::candles::candle::Candle;
use crate::account::context::Context;

pub trait StrategyMonitor: IndicatorGroup {
    //protected Context context;
    //protected Trader trader;

    fn handle_stop(&self, trade: Trade) -> Option<String> {
        None
    }

    fn discard_buy(&self, strategy: impl Strategy) -> boolean {
        false
    }

    fn discard_short_sell(&self, strategy: impl Strategy) -> boolean {
        false
    }

    fn allow_mixed_strategies(&self) -> boolean {
        true
    }

    fn highest_profit(&self, trade: Trade, change: f64) {}

    fn worst_loss(&self, trade: Trade, change: f64) {}

    fn bought(&self, trade: Trade, order: Order) {}

    fn sold(&self, trade: Trade, order: Order) {}

    fn allow_exit(&self, trade: Trade) -> boolean {
        true
    }

    fn allow_trade_switch(
        &self,
        trade: Trade,
        exit_symbol: String,
        candle: Candle,
        candle_ticker: String,
    ) -> boolean {
        false
    }

    fn set_context(&self, context: Context);
}
