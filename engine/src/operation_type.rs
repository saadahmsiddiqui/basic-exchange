use std::fmt::{self, Display};
use serde::de::{Visitor, Deserialize};
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
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