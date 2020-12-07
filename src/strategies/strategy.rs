use std::cmp::Ordering;
use crate::candles::candle::Candle;
use crate::indicators::signal::Signal;
use crate::account::trade::TradeSide;

pub trait Strategy {
    fn get_signal(&self, candle: Candle) -> Signal;

    fn trade_side(&self) -> Option<TradeSide> {
        None
    }

    fn exit_on_opposite_signal(&self) -> bool {
        return true;
    }
}
