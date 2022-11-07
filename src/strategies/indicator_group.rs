use crate::candles::aggregator::Aggregator;
use crate::candles::candle::Candle;
use crate::strategies::indicator::Indicator;

pub trait IndicatorGroup {
    fn is_initialized(&self) -> bool;

    fn initialize(&self, parent: Aggregator) {
        if self.is_initialized() {
            return;
        }

        let indicators = self.get_indicators();

        if indicators.is_empty() {
            return;
        }

        for indicator in indicators.iter() {
            indicator.initialize(&parent);
        }
    }

    fn accumulate(&self, candle: &Candle) {
        let indicators = self.get_indicators();

        for indicator in indicators.iter() {
            indicator.accumulate(
                candle,
            );
        }

        self.candle_accumulated(
            candle,
        );
    }

    fn get_indicators(&self) -> Vec<Box<dyn Indicator>>;

    fn candle_accumulated(&self, candle: &Candle);
}
