use serde::{Deserialize, Serialize};

use crate::amount::Amount;
use crate::price::Price;

#[derive(Serialize, Deserialize)]
pub struct Trade {
    fill_price: Price,
    filled_amount: Amount,
    trade_id: u64,
    sell_order_id: u64,
    buy_order_id: u64,
}

impl Trade {
    pub fn new(
        fill_price: Price,
        filled_amount: Amount,
        trade_id: u64,
        buy_order_id: u64,
        sell_order_id: u64,
    ) -> Trade {
        Trade {
            fill_price: fill_price,
            filled_amount: filled_amount,
            trade_id: trade_id,
            sell_order_id: sell_order_id,
            buy_order_id: buy_order_id,
        }
    }
}
