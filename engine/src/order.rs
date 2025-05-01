
use serde_with::{serde_as, DisplayFromStr};
use serde::{Serialize, Deserialize};
use crate::operation_type::OperationType;
use crate::price::Price;
use crate::side::Side;

pub type OrderId = u64;
pub type AccountId = u64;
pub type Amount = f64;
pub type Pair = String;


#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub type_op: OperationType,
    #[serde_as(as = "DisplayFromStr")]
    pub account_id: AccountId,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: Amount,
    pub pair: Pair,
    #[serde_as(as = "DisplayFromStr")]
    pub order_id: OrderId,
    pub limit_price: Price,
    pub side: Side
}


impl Order {
    pub fn new(
        op: OperationType,
        account_id: AccountId,
        amount: Amount,
        pair: String,
        order_id: OrderId,
        limit_price: Price,
        side: Side
    ) -> Order {
        Order {
            type_op: op,
            account_id: account_id,
            amount: amount,
            pair: pair,
            order_id: order_id,
            limit_price: limit_price,
            side: side
        }
    }
}