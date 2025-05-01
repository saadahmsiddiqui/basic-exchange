use std::fmt::{self, Display};
use serde::de::{Deserialize, Visitor};
use serde::{Serialize, Deserialize as Des};
use serde_with::{serde_as, DisplayFromStr};

type OrderId = u64;
type AccountId = u64;
type Price = f64;
type Amount = f64;
type Pair = String;

pub enum Side {
    BUY,
    SELL
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Side::BUY => "BUY",
            Side::SELL => "SELL"
        };

        write!(f, "{}", message)
    }
}

impl Serialize for Side {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        let message = match self {
            Side::BUY => "BUY",
            Side::SELL => "SELL"
        };

        serializer.serialize_str(message)
    }
}

impl<'de> Deserialize<'de> for Side {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        
        struct SideVisitor;
        impl<'de> Visitor<'de> for SideVisitor {
            type Value = Side;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a Side")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                let value = match v {
                    "BUY" => Result::Ok(Side::BUY),
                    "SELL" => Result::Ok(Side::SELL),
                    _ => Err(E::custom(""))
                };

                value
            }
        }

        deserializer.deserialize_str(SideVisitor)
    }

}

pub enum OperationType {
    CREATE,
    DELETE
}


impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            OperationType::CREATE => "CREATE",
            OperationType::DELETE => "DELETE"
        };

        write!(f, "{}", message)
    }
}

impl<'de> Deserialize<'de> for OperationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {

        struct OperationTypeVisitor;
        impl<'de> Visitor<'de> for OperationTypeVisitor {
            type Value = OperationType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a status")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                
                let value = match v {
                    "CREATE" => Ok(OperationType::CREATE),
                    "DELETE" => Ok(OperationType::DELETE),
                    _ => Err(E::custom("unknown value, unable to deserialize"))
                };

                value
            }
        }

        deserializer.deserialize_str(OperationTypeVisitor)
    }
}

impl Serialize for OperationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {

        let message = match self {
            OperationType::CREATE => "CREATE",
            OperationType::DELETE => "DELETE"
        };

        serializer.serialize_str(message)
    }
}

#[serde_as]
#[derive(Serialize, Des)]
pub struct Order {
    pub type_op: OperationType,
    #[serde_as(as = "DisplayFromStr")]
    pub account_id: AccountId,
    #[serde_as(as = "DisplayFromStr")]
    pub amount: Amount,
    pub pair: Pair,
    #[serde_as(as = "DisplayFromStr")]
    pub order_id: OrderId,
    #[serde_as(as = "DisplayFromStr")]
    pub limit_price: Price,
    pub side: Side
}


impl Order {

    pub fn new() -> Order {
        Order {
            type_op: OperationType::CREATE,
            account_id: 1,
            amount: 0.0,
            pair: String::from("Hello Order 1"),
            order_id: 1,
            limit_price: 0.0,
            side: Side::BUY
        }

    }
}