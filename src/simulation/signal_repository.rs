use std::collections::HashMap;
use crate::candles::candle::Candle;
use crate::indicators::signal::Signal;

static DEFAULT_REPO_HEADERS: [&str; 8] = [
    "OPEN_TIME",
    "CLOSE_TIME",
    "OPEN",
    "HIGH",
    "LOW",
    "CLOSE",
    "VOLUME",
    "SIGNAL"
];

struct SignalRepository {
    headers: Vec<String>,
    signals: HashMap<String, HashMap<Candle, Signal>>,

    repository_dir: String,
}

impl SignalRepository {
    //pub fn new(repository_dir: &str) -> SignalRepository {
    pub fn new() -> SignalRepository {
        SignalRepository {
            headers: DEFAULT_REPO_HEADERS.iter().map(|s| s.to_string()).collect(),
            signals: HashMap::new(),
            //repository_dir: repository_dir.to_string(),
            repository_dir: String::new(),
        }
    }

    pub fn add(&mut self, symbol: &str, signal: Signal, candle: Candle) {
        self.signals
            .entry(
                symbol.to_string()
            )
            .or_insert(
                HashMap::new()
            )
            .insert(candle, signal);
    }

    pub fn save(&self) {
        unimplemented!()
    }

    pub fn load(&mut self, symbol: &str, input: &str) {
        unimplemented!()
    }
}