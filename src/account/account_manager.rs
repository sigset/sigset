use std::collections::HashMap;
use crate::account::balance::Balance;
use crate::client_account::ClientAccount;
use crate::account::trading_manager::TradingManager;

pub struct AccountManager {
    trade_counter: u64,

    balance_update_counts: HashMap<String, u64>,

    // 10 * 60 * 1000
    balance_expiration_time: u64,

    // 15 * 60 * 1000
    frequent_balance_update_interval: u64,

    last_balance_sync: u64,

    balances: Vec<Balance>,

    account: Box<dyn ClientAccount>,
    account_hash: u64,

    margin_reserve_factor: f64,
    margin_reserve_factor_pct: f64,

    latest_prices: HashMap<String, Vec<f64>>,

    trading_managers: HashMap<String, Vec<TradingManager>>,
}
