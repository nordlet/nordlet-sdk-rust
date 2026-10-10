pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VatResolveReferenceRequestGoodsKind {
    Installed,
    EnergyNetwork,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VatResolveReferenceRequestGoodsKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Installed => serializer.serialize_str("installed"),
            Self::EnergyNetwork => serializer.serialize_str("energy_network"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VatResolveReferenceRequestGoodsKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "installed" => Ok(Self::Installed),
            "energy_network" => Ok(Self::EnergyNetwork),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VatResolveReferenceRequestGoodsKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Installed => write!(f, "installed"),
            Self::EnergyNetwork => write!(f, "energy_network"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
