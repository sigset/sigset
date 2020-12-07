use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub struct Candle {
    pub open_time: u64,
    pub close_time: u64,

    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,

    pub volume: f64,
}

impl Candle {
    fn new(
        open_time: u64,
        close_time: u64,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Candle {
        Candle {
            open_time,
            close_time,

            open,
            high,
            low,
            close,
            volume,
        }
    }

    fn get_change(&self) -> f64 {
        (self.close / self.open) - 1.0
    }

    fn set_merged(&mut self, merged: bool) {
        self.merged = merged;
    }

    pub fn merge(&self, other: &Candle) -> Candle {
        Candle::new(
            self.open_time,
            other.close_time,
            self.open,
            self.high.max(other.high),
            self.low.min(other.low),
            other.close,
            self.volume + other.volume,
        )
    }

    fn is_close_positive(&self) -> bool {
        self.close > self.open
    }

    fn is_green(&self) -> bool {
        self.open <= self.close
    }

    fn is_red(&self) -> bool {
        self.open > self.close
    }

    fn is_tick(&self) -> bool {
        self.open_time == self.close_time
        && self.open == self.close
        && self.close == self.high
        && self.high == self.low
    }
}

impl PartialOrd for Candle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        if self.close_time == other.close_time {
            if self.open_time == other.open_time {
                return Some(Ordering::Equal);
            }

            if self.open_time < other.open_time {
                return Some(Ordering::Less);
            }

            return Some(Ordering::Greater);
        }

        if self.close_time < other.close_time {
            return Some(Ordering::Less);
        }

        return Some(Ordering::Greater);
    }
}
