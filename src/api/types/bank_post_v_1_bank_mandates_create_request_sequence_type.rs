pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BankMandatesCreateRequestSequenceType {
    Recurrent,
    OneOff,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BankMandatesCreateRequestSequenceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Recurrent => serializer.serialize_str("recurrent"),
            Self::OneOff => serializer.serialize_str("one_off"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BankMandatesCreateRequestSequenceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "recurrent" => Ok(Self::Recurrent),
            "one_off" => Ok(Self::OneOff),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BankMandatesCreateRequestSequenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Recurrent => write!(f, "recurrent"),
            Self::OneOff => write!(f, "one_off"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
