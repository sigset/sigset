use crate::account::order_request::OrderRequest;
use crate::account::trade::{Trade, TradeSide};
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum OrderType {
    #[default]
    Limit,
    Market,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum OrderStatus {
    #[default]
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriggerCondition {
    None,
    StopLoss, // SL
    StopGain, // SG
}

impl TriggerCondition {
    pub fn from_str(s: &str) -> Option<TriggerCondition> {
        match s {
            "SL" => Some(TriggerCondition::StopLoss),
            "SG" => Some(TriggerCondition::StopGain),
            _ => None,
        }
    }

    pub fn short_name(&self) -> &str {
        match self {
            TriggerCondition::StopLoss => "SL",
            TriggerCondition::StopGain => "SG",
            TriggerCondition::None => "",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Order {
    pub order_request: OrderRequest,

    pub id: u64,
    pub order_id: String,

    pub processed: bool,

    pub status: OrderStatus,

    pub executed_quantity: f64,
    pub fees_paid: f64,
    pub average_price: f64,

    pub attachments: Vec<Order>,

    pub parent: Option<Order>,

    pub partial_fill_price: f64,
    pub partial_fill_quantity: f64,

    pub trade: Option<Trade>,
}

impl Order {
    pub fn new(
        id: u64,
        asset_symbol: impl Into<String>,
        funds_symbol: impl Into<String>,
        side: OrderSide,
        trade_side: TradeSide,
        time: u64,
    ) -> Order {
        let order_request = OrderRequest::new(
            asset_symbol.into(),
            funds_symbol.into(),
            side,
            trade_side,
            time,
            None,
        );

        Order {
            // order

            id,

            order_request,
            order_id: format!("{}", id),
            processed: false,
            status: OrderStatus::New,
            executed_quantity: 0.0,
            fees_paid: 0.0,
            average_price: 0.0,
            attachments: vec![],
            parent: None,
            partial_fill_price: 0.0,
            partial_fill_quantity: 0.0,
            trade: None,
        }
    }

    pub fn new_from_order_request(
        id: u64,
        order_request: OrderRequest,
    ) -> Order {
        Order::new(
            id,
            order_request.assets_symbol(),
            order_request.funds_symbol(),
            order_request.side().clone(),
            order_request.trade_side().clone(),
            order_request.time(),
        )
    }

    pub fn order_id(&self) -> String {
        self.order_id.clone()
    }

    pub fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }

    pub fn executed_quantity(&self) -> f64 {
        self.executed_quantity.clone()
    }

    pub fn set_executed_quantity(&mut self, executed_quantity: f64) {
        self.executed_quantity = executed_quantity;
    }

    pub fn total_order_amount_at_average_price(&self) -> f64 {
        if self.average_price {
            self.get_price() * self.get_quantity()
        } else {
            self.averagePrice * self.get_quantity()
        }
    }

    pub fn set_price(&mut self, price: f64) {
        self.order_request
            .set_price(
                price,
            );
    }

    pub fn status(&self) -> OrderStatus {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: OrderStatus) {
        self.status = status;
    }

    pub fn cancel(&mut self) {
        if self.status == OrderStatus::Filled {
            return;
        }

        self.status = OrderStatus::Cancelled;
    }

    pub fn is_cancelled(&self) -> bool {
        self.status == OrderStatus::Cancelled
    }

    pub fn fees_paid(&self) -> f64 {
        self.fees_paid
    }

    pub fn set_fees_paid(&mut self, fees_paid: f64) {
        self.fees_paid = fees_paid;
    }

    pub fn average_price(&self) -> f64 {
        self.average_price
    }

    pub fn set_average_price(&mut self, average_price: f64) {
        self.average_price = average_price;
    }

    pub fn attachments(&self) -> &Vec<Order> {
        &self.attachments
    }

    pub fn parent(&self) -> &Option<Order> {
        &self.parent
    }

    pub fn parent_order_id(&self) -> Option<String> {
        match &self.parent {
            Some(parent) => Some(parent.order_id()),
            _ => None,
        }
    }

    pub fn quantity(&self) -> f64 {
        let out = self.order_request.quantity();

        if self.parent.is_some() {
            let parent = self.parent.unwrap();

            if parent.is_finalized() {
                let p = self.executed_quantity();

                if out > p || p == 0.0 {
                    return p;
                }
            }
        }

        out
    }

    pub fn set_quantity(&mut self, quantity: f64) {
        self.order_request.set_quantity(quantity);
    }

    pub fn has_partial_fill_details(&self) -> bool {
        self.partial_fill_quantity != 0.0
            && self.partial_fill_quantity > 0.0
    }

    pub fn clear_partial_fill_details(&mut self) {
        self.partial_fill_quantity = 0.0;
        self.partial_fill_price = 0.0;
    }

    pub fn get_partial_fill_total_price(&self) -> f64 {
        self.partial_fill_quantity
            * self.partial_fill_price
    }

    pub fn get_partial_fill_price(&self) -> f64 {
        self.partial_fill_price
    }

    pub fn get_partial_fill_quantity(&self) -> f64 {
        self.partial_fill_quantity
    }

    pub fn set_partial_fill_details(&mut self, fill_price: f64, filled_quantity: f64) {
        self.partial_fill_price = fill_price;
        self.partial_fill_quantity = filled_quantity;
    }

    pub fn internal_id(&self) -> u64 {
        self.id
    }

    pub fn set_trade(&mut self, trade: &Trade) {
        self.trade = Some(trade.clone());
    }

    pub fn trade(&self) -> Option<&Trade> {
        self.trade.as_ref()
    }

    pub fn get_time_elapsed(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - self.order_request.time()
    }

    pub fn get_time_elapsed_since_close(
        &self,
        close_time: u64,
    ) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now - close_time
    }

    pub fn get_remaining_quantity(&self) -> f64 {
        self.quantity() - self.executed_quantity()
    }

    pub fn get_total_traded_amount(&self) -> f64 {
        self.executed_quantity() * self.get_price()
    }

    pub fn is_finalized(&self) -> bool {
        self.status == OrderStatus::Filled
            || self.status == OrderStatus::Cancelled
    }

    pub fn get_filled_ratio(&self) -> f64 {
        if self.quantity() == 0.0 {
            return 0.0;
        }

        self.executed_quantity() / self.quantity()
    }

    pub fn side_description(&self) -> String {
        if self.order_request.is_short() {
            if self.order_request.is_buy() {
                "Short".to_string()
            } else {
                "Cover".to_string()
            }
        } else {
            format!("{:?}", self.order_request.side())
        }
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(
            self.id
                .cmp(&other.id),
        )
    }
}
