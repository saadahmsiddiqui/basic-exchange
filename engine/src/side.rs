use std::fmt::{self, Display};

use serde::de::{Visitor, Deserialize};
use serde::Serialize;

#[derive(Copy, Clone)]
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
