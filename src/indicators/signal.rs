pub enum Signal {
    UNDERVALUED, // ('u', 1)
    BUY, // ('B', 0.5)
    NEUTRAL, // ('-', 0)
    SELL, // ('S', -0.5)
    OVERVALUED, // ('o', -1)
}

impl Signal {
    pub fn get_value(&self) -> (char, f32) {
        match self {
            Signal::UNDERVALUED => ('u', 1.0),
            Signal::BUY => ('B', 0.5),
            Signal::NEUTRAL => ('-', 0.0),
            Signal::SELL => ('S', -0.5),
            Signal::OVERVALUED => ('o', -1.0),
        }
    }
}
