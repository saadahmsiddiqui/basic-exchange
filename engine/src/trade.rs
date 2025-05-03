use serde::{Deserialize, Serialize};

use crate::engine::amount::Amount;
use crate::engine::price::Price;
use crate::engine::side::Side;

#[derive(Serialize, Deserialize)]
pub struct Trade {
    side: Side,
    fill_price: Price,
    filled_amount: Amount,
    trade_id: u64,
    sell_order_id: u64,
    buy_order_id: u64,
    trade_account_id: u64
}

impl Trade {
    pub fn new(
        fill_price: Price,
        filled_amount: Amount,
        trade_id: u64,
        buy_order_id: u64,
        sell_order_id: u64,
        trade_account_id: u64,
        side: Side,
    ) -> Trade {
        Trade {
            fill_price,
            filled_amount,
            trade_id,
            sell_order_id,
            buy_order_id,
            trade_account_id,
            side
        }
    }
}
