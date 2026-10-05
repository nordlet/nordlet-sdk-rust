pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaxPaymentsCreateDeclarationsResponseKind {
    Advance,
    Withholding,
    Final,
    Refund,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaxPaymentsCreateDeclarationsResponseKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Advance => serializer.serialize_str("advance"),
            Self::Withholding => serializer.serialize_str("withholding"),
            Self::Final => serializer.serialize_str("final"),
            Self::Refund => serializer.serialize_str("refund"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaxPaymentsCreateDeclarationsResponseKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "advance" => Ok(Self::Advance),
            "withholding" => Ok(Self::Withholding),
            "final" => Ok(Self::Final),
            "refund" => Ok(Self::Refund),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaxPaymentsCreateDeclarationsResponseKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Advance => write!(f, "advance"),
            Self::Withholding => write!(f, "withholding"),
            Self::Final => write!(f, "final"),
            Self::Refund => write!(f, "refund"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
