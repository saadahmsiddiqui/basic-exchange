use std::fmt::{self, Display};
use std::ops::{Add, Sub};

use serde::de::{Visitor, Deserialize};
use serde::Serialize;

use crate::engine::constants::PRECISION;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Amount(pub u64);

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        
        struct AmountVisitor;
        impl<'de> Visitor<'de> for AmountVisitor {
            type Value = Amount;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a status")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                
                let val = v.parse::<f64>().unwrap();
                let upscaled = (val * PRECISION as f64) as u64;

                Ok(Amount(upscaled))
            }
        }

        deserializer.deserialize_str(AmountVisitor)


    }
}

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let val = self.0;
        let original_amount = (val as f64) / (PRECISION as f64);
        let in_str = original_amount.to_string();
        serializer.serialize_str(&in_str)
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let in_str = self.0.to_string();
        f.write_str(&in_str)
    }
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, rhs: Self) -> Self::Output {
        let amount = self.0 + rhs.0;
        Self(amount)
    }
}

impl Sub for Amount {
    type Output = Amount;

    fn sub(self, rhs: Self) -> Self::Output {
        let amount = self.0 - rhs.0;
        Self(amount)
    }
}