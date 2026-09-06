pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1AgreementsAgreementsListResponseRowsItemKind {
    Customer,
    Supplier,
    Employment,
    Bank,
    Lease,
    Insurance,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1AgreementsAgreementsListResponseRowsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Customer => serializer.serialize_str("customer"),
            Self::Supplier => serializer.serialize_str("supplier"),
            Self::Employment => serializer.serialize_str("employment"),
            Self::Bank => serializer.serialize_str("bank"),
            Self::Lease => serializer.serialize_str("lease"),
            Self::Insurance => serializer.serialize_str("insurance"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1AgreementsAgreementsListResponseRowsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "customer" => Ok(Self::Customer),
            "supplier" => Ok(Self::Supplier),
            "employment" => Ok(Self::Employment),
            "bank" => Ok(Self::Bank),
            "lease" => Ok(Self::Lease),
            "insurance" => Ok(Self::Insurance),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1AgreementsAgreementsListResponseRowsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Customer => write!(f, "customer"),
            Self::Supplier => write!(f, "supplier"),
            Self::Employment => write!(f, "employment"),
            Self::Bank => write!(f, "bank"),
            Self::Lease => write!(f, "lease"),
            Self::Insurance => write!(f, "insurance"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
