use std::collections::HashMap;
use crate::client_account::ClientAccount;

pub struct OrderBook {
    symbol: String,

    depth: u64,

    bids: HashMap<f64, f64>,
    asks: HashMap<f64, f64>,

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

            bids: HashMap::new(),
            asks: HashMap::new(),

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

        while self.bids.size() > depth {
            self.bids.remove(self.bids.lastKey());
        }
    }

    fn add_ask(
        &mut self,
        price: f64,
        quantity: f64,
    ) {
        self.add
    }
}
