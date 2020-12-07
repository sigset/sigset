use crate::candles::candle::Candle;
use crate::indicators::signal::Signal;
use crate::candles::aggregator::Aggregator;

pub trait Indicator {
    fn accumulate(&self, candle: &Candle);

    fn get_accumulation_count(&self) -> f64;

    fn get_value(&self) -> f64;

    fn get_interval(&self) -> u64;

    fn get_signal(&self, candle: &Candle) -> Signal;

    fn signal_description(&self) -> String {
        return String::new();
    }

    fn initialize(
        &self,
        aggregator: Aggregator,
    ) {
        // none
    }

    fn recalculate_every_tick(
        &self,
        recalculate_every_tick: bool,
    ) {
        // none
    }
}
