use crate::account::order_request::OrderRequest;
use crate::account::order::{Order, OrderType, OrderSide};
use crate::account::balance::Balance;
use std::collections::HashMap;

pub trait TradingFees {
    fn take_fee(amount: f64, order_type: OrderType, order_side: OrderSide) -> f64;

    fn fees_on_order(&self, order: Order) -> f64 {
        // final double amount = order.getTotalOrderAmount();
        // return feesOnAmount(amount, order.getType(), order.getSide());

        1.0
    }

    fn fees_on_order_request(&self, order_request: OrderRequest) -> f64 {
        // final double amount = order.getTotalOrderAmount();
        // return feesOnAmount(amount, order.getType(), order.getSide());

        1.0
    }

    fn fees_on_traded_amount(&self, order: Order) -> f64 {
        // final double amount = order.getTotalTraded();
        // if (amount == 0.0) {
        //     return 0.0;
        // }
        // return feesOnAmount(amount, order.getType(), order.getSide());

        1.0
    }

    fn fees_on_partial_fill(&self, order: Order) -> f64 {
        // final double amount = order.get_partial_fill_total_price();
        // if (amount == 0.0) {
        //     return 0.0;
        // }
        // return feesOnAmount(amount, order.getType(), order.getSide());

        1.0
    }

    fn fees_on_amount(&self, order: Order) -> f64 {
        return order.get_partial_fill_total_price() - self.take_fee(order.get_partial_fill_total_price(), orderType, side);
    }

    fn fees_on_total_order_amount(&self, order: Order) -> f64 {
        // double amount = order.getTotalOrderAmount();
        // return feesOnAmount(amount, order.getType(), order.getSide());

        1.0
    }

    fn get_break_even_amount(&self, order: Order) -> f64 {
        // double out = takeFee(amount, Order.Type.LIMIT, Order.Side.BUY);
        // out = takeFee(out, Order.Type.LIMIT, Order.Side.BUY);
        // return amount + (amount - out);

        1.0
    }

    fn get_break_even_change(&self, order: Order) -> f64 {
        // if (amount == 0.0) {
        //     return 0.0;
        // }
        // double breakEvenAmount = getBreakEvenAmount(amount);
        // return ((breakEvenAmount / amount) - 1) * 100.0;

        1.0
    }
}
