use std::collections::HashMap;
use crate::account::balance::Balance;
use crate::client_account::ClientAccount;
use crate::account::trading_manager::TradingManager;

pub struct AccountManager<Account: ClientAccount> {
    trade_counter: u64,

    balance_update_counts: HashMap<String, u64>,

    // 10 * 60 * 1000
    balance_expiration_time: u64,

    // 15 * 60 * 1000
    frequent_balance_update_interval: u64,

    last_balance_sync: u64,

    balances: Vec<Balance>,

    account: Account,
    account_hash: u64,

    margin_reserve_factor: f64,
    margin_reserve_factor_pct: f64,

    latest_prices: HashMap<String, Vec<f64>>,

    trading_managers: HashMap<String, Vec<TradingManager>>,
}

impl <Account: ClientAccount> AccountManager<Account> {
    pub fn new(
        account: Account,
        balance_expiration_time: u64,
        frequent_balance_update_interval: u64,
        margin_reserve_factor: f64,
    ) -> AccountManager<Account> {
        AccountManager {
            trade_counter: 0,

            balance_update_counts: HashMap::new(),

            balance_expiration_time,
            frequent_balance_update_interval,

            last_balance_sync: 0,

            balances: Vec::new(),

            account,
            account_hash: 0,

            margin_reserve_factor,
            margin_reserve_factor_pct: margin_reserve_factor * 100.0,

            latest_prices: HashMap::new(),

            trading_managers: HashMap::new(),

            signal_repository: SignalRepository::new(),
        }
    }
}