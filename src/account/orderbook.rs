use std::collections::{BTreeMap, HashMap};
use crate::client_account::ClientAccount;

pub struct OrderBook {
    symbol: String,

    depth: u64,

    bids: BTreeMap<f64, f64>,
    asks: BTreeMap<f64, f64>,

    client_account: Box<dyn ClientAccount>,
}

impl OrderBook {
    fn new(
        client_account: impl ClientAccount,
        symbol: String,
        depth: u64,
    ) -> OrderBook {
        OrderBook {
            symbol,
            depth,

            bids: BTreeMap::new(),
            asks: BTreeMap::new(),

            client_account: Box::new(client_account)
        }
    }

    fn add_bid(
        &mut self,
        price: f64,
        quantity: f64,
    ) {
        self.bids.put(
            price,
            quantity,
        );

        while self.bids.size() > self.depth {
            self.bids.remove(self.bids.lastKey());
        }
    }

    fn add_ask(
        &mut self,
        price: f64,
        quantity: f64,
    ) {
        self.asks.put(
            price,
            quantity,
        );

        while self.asks.size() > self.depth {
            self.asks.remove(self.asks.firstKey());
        }
    }

    fn get_average_ask_amount_by_depth(&self, depth: u64) -> f64 {
        let mut total_amount = 0.0;
        let mut total_quantity = 0.0;

        let mut i = 0;

        for (price, quantity) in self.asks.iter() {
            if i >= depth {
                break;
            }

            total_amount += price * quantity;
            total_quantity += quantity;

            i += 1;
        }

        total_amount / total_quantity
    }

    fn get_average_bid_amount_by_depth(&self, depth: u64) -> f64 {
        let mut total_amount = 0.0;
        let mut total_quantity = 0.0;

        let mut i = 0;

        for (price, quantity) in self.bids.iter() {
            if i >= depth {
                break;
            }

            total_amount += price * quantity;
            total_quantity += quantity;

            i += 1;
        }

        total_amount / total_quantity
    }

    fn estimate_fill_price_asks(&self, quantity: f64) -> f64 {
        let mut filled_quantity = 0.0;
        let mut estimated_price = 0.0;

        for (level_price, level_quantity) in self.asks.iter() {
            let quantity_to_fill = quantity - filled_quantity;

            if quantity_to_fill > *level_quantity {
                filled_quantity += level_quantity;
                estimated_price += level_price * level_quantity;
            } else {
                filled_quantity += quantity_to_fill;
                estimated_price += level_price * quantity_to_fill;
                break;
            }
        }

        estimated_price
    }

    fn estimate_fill_price_bids(&self, quantity: f64) -> f64 {
        let mut filled_quantity = 0.0;
        let mut estimated_price = 0.0;

        for (level_price, level_quantity) in self.bids.iter() {
            let quantity_to_fill = quantity - filled_quantity;

            if quantity_to_fill > *level_quantity {
                filled_quantity += level_quantity;
                estimated_price += level_price * level_quantity;
            } else {
                filled_quantity += quantity_to_fill;
                estimated_price += level_price * quantity_to_fill;
                break;
            }
        }

        estimated_price
    }

    pub fn get_average_bid_amount_by_quantity(&self, quantity_to_fill: f64) -> f64 {
        self.estimate_fill_price_bids(quantity_to_fill)
    }

    pub fn get_average_ask_amount_by_quantity(&self, quantity_to_fill: f64) -> f64 {
        self.estimate_fill_price_asks(quantity_to_fill)
    }

    pub fn get_spread_by_quantity(&self, quantity_to_fill: f64) -> f64 {
        self.get_average_ask_amount(quantity_to_fill) - self.get_average_bid_amount(quantity_to_fill)
    }

    pub fn get_depth(&self) -> u64 {
        self.depth
    }

    pub fn bids(&self) -> &BTreeMap<f64, f64> {
        &self.bids
    }

    pub fn asks(&self) -> &BTreeMap<f64, f64> {
        &self.asks
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn is_empty(&self) -> bool {
        self.bids.is_empty() && self.asks.is_empty()
    }
}
