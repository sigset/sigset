use crate::account::order_request::OrderRequest;
use crate::account::order::Order;
use crate::account::balance::Balance;
use std::collections::HashMap;

pub trait ClientAccount {
    fn execute_order(order_details: OrderRequest) -> Order;

    fn update_balances(force: bool) -> HashMap<String, Balance>;

    //fn get_order_book(symbol: String, depth: u64) -> OrderBook;

    fn update_order_status(order: Order);

    fn cancel(order: Order);

    fn is_simulated() -> bool {
        false
    }

    fn margin_reserve_percentage() -> u64 {
        150
    }
}
