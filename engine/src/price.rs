use std::fmt::{self, Display};

use serde::de::{Visitor, Deserialize};
use serde::Serialize;
use crate::constants::PRECISION;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Price(pub u64);

impl<'de> Deserialize<'de> for Price {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        
        struct PriceVisitor;
        impl<'de> Visitor<'de> for PriceVisitor {
            type Value = Price;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a status")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                
                let val = v.parse::<f64>().unwrap();
                let upscaled = (val * PRECISION as f64) as u64;

                Ok(Price(upscaled))
            }
        }

        deserializer.deserialize_str(PriceVisitor)


    }
}


impl Serialize for Price {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let val = self.0;
        let original_price = (val as f64) / (PRECISION as f64);
        let in_str = original_price.to_string();
        serializer.serialize_str(&in_str)
    }
}

impl Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let in_str = self.0.to_string();
        f.write_str(&in_str)
    }
}