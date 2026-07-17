pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1EcommerceOrdersReserveResponseStatus {
    New,
    Reserved,
    Fulfilled,
    Cancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1EcommerceOrdersReserveResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::New => serializer.serialize_str("new"),
            Self::Reserved => serializer.serialize_str("reserved"),
            Self::Fulfilled => serializer.serialize_str("fulfilled"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1EcommerceOrdersReserveResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "new" => Ok(Self::New),
            "reserved" => Ok(Self::Reserved),
            "fulfilled" => Ok(Self::Fulfilled),
            "cancelled" => Ok(Self::Cancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1EcommerceOrdersReserveResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::New => write!(f, "new"),
            Self::Reserved => write!(f, "reserved"),
            Self::Fulfilled => write!(f, "fulfilled"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
