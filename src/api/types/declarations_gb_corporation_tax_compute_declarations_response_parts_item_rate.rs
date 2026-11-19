pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GbCorporationTaxComputeDeclarationsResponsePartsItemRate {
    Main,
    SmallProfits,
    MarginalRelief,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GbCorporationTaxComputeDeclarationsResponsePartsItemRate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Main => serializer.serialize_str("main"),
            Self::SmallProfits => serializer.serialize_str("small_profits"),
            Self::MarginalRelief => serializer.serialize_str("marginal_relief"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GbCorporationTaxComputeDeclarationsResponsePartsItemRate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "main" => Ok(Self::Main),
            "small_profits" => Ok(Self::SmallProfits),
            "marginal_relief" => Ok(Self::MarginalRelief),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GbCorporationTaxComputeDeclarationsResponsePartsItemRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Main => write!(f, "main"),
            Self::SmallProfits => write!(f, "small_profits"),
            Self::MarginalRelief => write!(f, "marginal_relief"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
