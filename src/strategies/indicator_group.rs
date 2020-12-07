use crate::strategies::indicator::Indicator;
use crate::candles::aggregator::Aggregator;
use crate::candles::candle::Candle;

pub trait IndicatorGroup {
    fn initialize(&self, parent: Aggregator) {
        let indicators = self.get_indicators();

        for indicator in indicators.iter() {
            indicator.initialize(
                candle,
            );
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
