use std::collections::{HashMap, HashSet};

pub struct Balance {
    symbol: String,

    pub free: f64,
    pub locked: f64,
    pub shorted: f64,

    pub trading_locked: bool,

    pub margin_reserves: HashMap<String, f64>,
    pub shorted_asset_symbols: HashSet<String>,

    balance_update_counts: u64,
}

impl Balance {
    fn new(symbol: String) -> Balance {
        Balance {
            symbol,

            free: 0.0,
            locked: 0.0,
            shorted: 0.0,

            trading_locked: false,

            margin_reserves: HashMap::new(),
            shorted_asset_symbols: HashSet::new(),

            balance_update_counts: 0,
        }
    }

    fn free(
        &mut self,
        free: f64,
    ) {
        self.free = free;
    }

    fn locked(
        &mut self,
        locked: f64,
    ) {
        self.locked = locked;
    }

    fn shorted(
        &mut self,
        shorted: f64,
    ) {
        self.shorted = shorted;
    }

    fn total(
        &self,
    ) -> f64 {
        self.free + self.locked
    }

    fn margin_reserve(
        &mut self,
        asset_symbol: String,
        margin_reserve: f64,
    ) {
        if margin_reserve <= 0.0 {
            self.margin_reserves.remove(
                &asset_symbol,
            );
        } else {
            self.margin_reserves.insert(
                asset_symbol,
                margin_reserve,
            );
        }
    }

    fn lock_trading(
        &mut self,
    ) {
        self.trading_locked = true;
    }

    fn unlock_trading(
        &mut self,
    ) {
        self.trading_locked = true;
    }
}
