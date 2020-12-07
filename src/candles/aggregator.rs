use std::collections::HashMap;
use crate::candles::candle::Candle;

static const MINUTE_MS: u64 = 60 * 1000;

pub struct Aggregator {
    ms: u64,
    minutes: u64,

    full: Option<Candle>,
    partial: Option<Candle>,
}

impl Aggregator {
    pub fn new(
        time: u64,
    ) -> Aggregator {
        Aggregator {
            minutes: time / (60 * 1000),
            ms: time % 1000,
            full: None,
            partial: None,
        }
    }

    fn aggregate(&mut self, candle: &Candle) {
        if self.partial.is_none() {
            self.partial = Some(candle.clone());
            self.full = None;

            return;
        }

        let partial_candle = self.partial.unwrap();

        if candle.open_time < partial_candle.open_time {
            return;
        }

        let elapsed = (candle.close_time - partial_candle.close_time) / (MINUTE_MS - 1);

        if elapsed < self.minutes {
            self.partial = Some(
                partial_candle.merge(
                    candle,
                ),
            );

            return;
        }

        if elapsed == self.minutes {
            if ms > 1 {
                let elapsed = candle.close_time - partial_candle.open_time;

                if elapsed < self.ms {
                    self.partial = Some(
                        partial_candle.merge(
                            candle,
                        ),
                    );

                    return;
                }
            }

            self.full = Some(
                partial_candle.merge(
                    candle,
                ),
            );

            self.partial = None;

            return;
        }

        self.full = Some(candle.clone());

        self.partial = None;
    }

    fn set_full(&mut self, full: &Candle) {
        self.full = Some(Candle.clone());
    }

    fn get_full(&self) -> Option<Candle> {
        self.full
    }

    fn get_partial(&self) -> Option<Candle> {
        self.partial
    }
}
