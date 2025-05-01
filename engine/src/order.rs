
use serde_with::{serde_as, DisplayFromStr};
use serde::{Serialize, Deserialize};
use crate::amount::Amount;
use crate::operation_type::OperationType;
use crate::price::Price;
use crate::side::Side;

pub type OrderId = u64;
pub type AccountId = u64;
pub type Pair = String;


#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub type_op: OperationType,
    #[serde_as(as = "DisplayFromStr")]
    pub account_id: AccountId,
    pub amount: Amount,
    pub pair: Pair,
    #[serde_as(as = "DisplayFromStr")]
    pub order_id: OrderId,
    pub limit_price: Price,
    pub side: Side
}
