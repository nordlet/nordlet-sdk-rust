pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ReportsTrialBalanceResponseRowsItemType {
    Asset,
    Liability,
    Equity,
    Income,
    Expense,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ReportsTrialBalanceResponseRowsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Asset => serializer.serialize_str("asset"),
            Self::Liability => serializer.serialize_str("liability"),
            Self::Equity => serializer.serialize_str("equity"),
            Self::Income => serializer.serialize_str("income"),
            Self::Expense => serializer.serialize_str("expense"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1ReportsTrialBalanceResponseRowsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "asset" => Ok(Self::Asset),
            "liability" => Ok(Self::Liability),
            "equity" => Ok(Self::Equity),
            "income" => Ok(Self::Income),
            "expense" => Ok(Self::Expense),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1ReportsTrialBalanceResponseRowsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asset => write!(f, "asset"),
            Self::Liability => write!(f, "liability"),
            Self::Equity => write!(f, "equity"),
            Self::Income => write!(f, "income"),
            Self::Expense => write!(f, "expense"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
