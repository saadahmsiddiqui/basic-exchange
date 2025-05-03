
use std::cmp::Ordering;
use std::hash::Hash;

use serde_with::{serde_as, DisplayFromStr};
use serde::{Serialize, Deserialize};
use crate::engine::amount::Amount;
use crate::engine::operation_type::OperationType;
use crate::engine::price::Price;
use crate::engine::side::Side;

pub type OrderId = u64;
pub type AccountId = u64;
pub type Pair = String;


#[serde_as]
#[derive(Serialize, Deserialize, Clone, Eq, PartialOrd)]
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


impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.account_id == other.account_id && 
        self.amount == other.amount &&
        self.pair == other.pair &&
        self.side == other.side &&
        self.limit_price == other.limit_price &&
        self.type_op == other.type_op
    }
}

impl Hash for Order {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.account_id.hash(state);
        self.amount.hash(state);
        self.pair.hash(state);
        self.side.hash(state);
        self.limit_price.hash(state);
        self.type_op.hash(state);
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.account_id > other.account_id {
            Ordering::Greater
        } else if self.account_id < other.account_id {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}