pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetsUpdateAssetsResponseInputVatUseChangesItemReason {
    UseChange,
    Sale,
    Withdrawal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AssetsUpdateAssetsResponseInputVatUseChangesItemReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UseChange => serializer.serialize_str("use_change"),
            Self::Sale => serializer.serialize_str("sale"),
            Self::Withdrawal => serializer.serialize_str("withdrawal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AssetsUpdateAssetsResponseInputVatUseChangesItemReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "use_change" => Ok(Self::UseChange),
            "sale" => Ok(Self::Sale),
            "withdrawal" => Ok(Self::Withdrawal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AssetsUpdateAssetsResponseInputVatUseChangesItemReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UseChange => write!(f, "use_change"),
            Self::Sale => write!(f, "sale"),
            Self::Withdrawal => write!(f, "withdrawal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
