pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsItSdiPurchaseSendResponseTipoDocumento {
    Td16,
    Td17,
    Td18,
    Td19,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsItSdiPurchaseSendResponseTipoDocumento {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Td16 => serializer.serialize_str("TD16"),
            Self::Td17 => serializer.serialize_str("TD17"),
            Self::Td18 => serializer.serialize_str("TD18"),
            Self::Td19 => serializer.serialize_str("TD19"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsItSdiPurchaseSendResponseTipoDocumento {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TD16" => Ok(Self::Td16),
            "TD17" => Ok(Self::Td17),
            "TD18" => Ok(Self::Td18),
            "TD19" => Ok(Self::Td19),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsItSdiPurchaseSendResponseTipoDocumento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Td16 => write!(f, "TD16"),
            Self::Td17 => write!(f, "TD17"),
            Self::Td18 => write!(f, "TD18"),
            Self::Td19 => write!(f, "TD19"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
