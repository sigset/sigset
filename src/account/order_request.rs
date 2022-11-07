use std::collections::{HashMap, HashSet};

use super::order::{OrderSide, TriggerCondition};
use super::trade::TradeSide;
use crate::account::order::{OrderType, Order};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderRequest {
    pub assets_symbol: String,
    pub funds_symbol: String,

    pub side: OrderSide,
    pub trade_side: TradeSide,

    pub cancelled: bool,
    pub active: bool,

    pub time: u64,

    pub price: f64,
    pub quantity: f64,

    pub order_type: OrderType,

    pub resubmitted_from: Option<Order>,

    pub trigger_price: f64,
    pub trigger_condition: TriggerCondition,

    pub attached_requests: Vec<OrderRequest>,
}

impl OrderRequest {
    pub fn new(
        assets_symbol: String,
        funds_symbol: String,
        side: OrderSide,
        trade_side: TradeSide,
        time: u64,
        resubmitted_from: Option<Order>,
    ) -> OrderRequest {
        OrderRequest {
            assets_symbol,
            funds_symbol,

            side,
            trade_side,

            time,

            active: true,
            cancelled: false,

            price: 0.0,
            quantity: 0.0,

            order_type: OrderType::Limit,

            trigger_price: 0.0,
            trigger_condition: TriggerCondition::None,

            attached_requests: vec![],

            resubmitted_from,
        }
    }

    pub fn assets_symbol(&self) -> &str {
        &self.assets_symbol
    }

    pub fn funds_symbol(&self) -> &str {
        &self.funds_symbol
    }

    pub fn symbol(&self) -> &str {
        self.assets_symbol() + self.funds_symbol()
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    pub fn set_quantity(&mut self, quantity: f64) {
        self.quantity = quantity;
    }

    pub fn side(&self) -> &OrderSide {
        &self.side
    }

    pub fn trade_side(&self) -> &TradeSide {
        &self.trade_side
    }

    pub fn order_type(&self) -> &OrderType {
        &self.order_type
    }

    pub fn set_order_type(&mut self, order_type: OrderType) {
        &self.order_type = *order_type
    }

    pub fn total_order_amount(&self) -> f64 {
        self.price * self.quantity
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled == true
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn is_resubmission(&self) -> bool {
        self.resubmitted_from.is_some()
    }

    pub fn orginal_order(&self) -> &Option<Order>{
        &self.resubmitted_from
    }

    pub fn is_short(&self) -> bool {
        self.trade_side == TradeSide::SHORT
    }

    pub fn is_long(&self) -> bool {
        self.trade_side == TradeSide::LONG
    }

    pub fn is_buy(&self) -> bool {
        self.side == OrderSide::Buy
    }

    pub fn is_sell(&self) -> bool {
        self.side == OrderSide::Sell
    }

    pub fn is_long_buy(&self) -> bool {
        self.is_long() && self.is_buy()
    }

    pub fn is_long_sell(&self) -> bool {
        self.is_long() && self.is_sell()
    }

    pub fn is_short_sell(&self) -> bool {
        self.is_short() && self.is_sell()
    }

    pub fn is_short_cover(&self) -> bool {
        self.is_short() && self.is_buy()
    }

    pub fn update_time(&mut self, time: u64) {
        self.time = time;
    }

    pub fn attached_order_requests(&self) -> Vec<OrderRequest> {
        self.attached_requests
            .iter()
            .cloned()
            .collect()
    }

    pub fn trigger_condition(&self) -> &TriggerCondition {
        &self.trigger_condition
    }

    pub fn trigger_price(&self) -> f64 {
        self.trigger_price
    }

    pub fn set_trigger_condition(
        &mut self,
        trigger_condition: TriggerCondition,
        trigger_price: f64,
    ) {
        self.trigger_condition = trigger_condition;
        self.trigger_price = trigger_price;

        self.active =
            self.trigger_condition == TriggerCondition::None
                || self.trigger_price == 0.0;

        if self.price == 0.0 && self.trigger_price != 0.0 {
            self.price = trigger_price;
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn is_active(&self) -> bool {
        self.active && !self.is_cancelled()
    }

    pub fn attach_order_request(
        &mut self,
        order_type: OrderType,
        price: f64,
    ) {
        let mut attachment = OrderRequest::new(
            self.assets_symbol.clone(),
            self.funds_symbol.clone(),
            match self.side {
                OrderSide::Buy => OrderSide::Sell,
                OrderSide::Sell => OrderSide::Buy,
            },
            self.trade_side.clone(),
            self.time,
            None,
        );

        attachment.set_quantity(
            self.quantity,
        );

        let diff = price - self.price;

        attachment.set_price(
            price,
        );

        attachment.set_order_type(
            order_type,
        );

        if diff < 0.0 {
            attachment.set_trigger_condition(
                TriggerCondition::StopLoss,
                attachment.price(),
            );
        }

        if diff >= 0.0 {
            attachment.set_trigger_condition(
                TriggerCondition::StopGain,
                attachment.price(),
            );
        }

        self.attached_requests.push(
            attachment,
        );
    }

    pub fn attach_to_percentage_change(
        &mut self,
        order_type: OrderType,
        percentage_change: f64,
    ) {
        self.attach_order_request(
            order_type,
            self.price * (1.0 + (percentage_change / 100)),
        );
    }

    pub fn attach_to_price_change(
        &mut self,
        order_type: OrderType,
        price_change: f64,
    ) {
        self.attach_order_request(
            order_type,
            self.price + price_change,
        );
    }

    pub fn set_attached_order_requests(
        &mut self,
        order_requests: Vec<OrderRequest>,
    ) {
        self.attached_requests = order_requests;
    }
}
