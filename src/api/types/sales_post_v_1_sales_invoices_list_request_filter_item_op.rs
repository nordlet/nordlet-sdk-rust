pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1SalesInvoicesListRequestFilterItemOp {
    Eq,
    Ne,
    Contains,
    Gte,
    Lte,
    In,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1SalesInvoicesListRequestFilterItemOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eq => serializer.serialize_str("eq"),
            Self::Ne => serializer.serialize_str("ne"),
            Self::Contains => serializer.serialize_str("contains"),
            Self::Gte => serializer.serialize_str("gte"),
            Self::Lte => serializer.serialize_str("lte"),
            Self::In => serializer.serialize_str("in"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1SalesInvoicesListRequestFilterItemOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eq" => Ok(Self::Eq),
            "ne" => Ok(Self::Ne),
            "contains" => Ok(Self::Contains),
            "gte" => Ok(Self::Gte),
            "lte" => Ok(Self::Lte),
            "in" => Ok(Self::In),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1SalesInvoicesListRequestFilterItemOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::Ne => write!(f, "ne"),
            Self::Contains => write!(f, "contains"),
            Self::Gte => write!(f, "gte"),
            Self::Lte => write!(f, "lte"),
            Self::In => write!(f, "in"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
