pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsDeFranchiseTaxComputeDeclarationsResponseMethod {
    AuthorizedShares,
    AssumedParValueCapital,
    LargeCorporateFiler,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UsDeFranchiseTaxComputeDeclarationsResponseMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AuthorizedShares => serializer.serialize_str("authorized-shares"),
            Self::AssumedParValueCapital => serializer.serialize_str("assumed-par-value-capital"),
            Self::LargeCorporateFiler => serializer.serialize_str("large-corporate-filer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UsDeFranchiseTaxComputeDeclarationsResponseMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "authorized-shares" => Ok(Self::AuthorizedShares),
            "assumed-par-value-capital" => Ok(Self::AssumedParValueCapital),
            "large-corporate-filer" => Ok(Self::LargeCorporateFiler),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UsDeFranchiseTaxComputeDeclarationsResponseMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthorizedShares => write!(f, "authorized-shares"),
            Self::AssumedParValueCapital => write!(f, "assumed-par-value-capital"),
            Self::LargeCorporateFiler => write!(f, "large-corporate-filer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
