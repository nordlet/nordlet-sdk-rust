pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1BillingTransactionsListResponseRowsItemType {
    TrialGrant,
    Topup,
    Usage,
    Activation,
    Adjustment,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1BillingTransactionsListResponseRowsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TrialGrant => serializer.serialize_str("trial_grant"),
            Self::Topup => serializer.serialize_str("topup"),
            Self::Usage => serializer.serialize_str("usage"),
            Self::Activation => serializer.serialize_str("activation"),
            Self::Adjustment => serializer.serialize_str("adjustment"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1BillingTransactionsListResponseRowsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "trial_grant" => Ok(Self::TrialGrant),
            "topup" => Ok(Self::Topup),
            "usage" => Ok(Self::Usage),
            "activation" => Ok(Self::Activation),
            "adjustment" => Ok(Self::Adjustment),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1BillingTransactionsListResponseRowsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TrialGrant => write!(f, "trial_grant"),
            Self::Topup => write!(f, "topup"),
            Self::Usage => write!(f, "usage"),
            Self::Activation => write!(f, "activation"),
            Self::Adjustment => write!(f, "adjustment"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
