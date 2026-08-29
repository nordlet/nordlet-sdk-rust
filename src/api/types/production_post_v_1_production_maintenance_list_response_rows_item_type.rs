pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1ProductionMaintenanceListResponseRowsItemType {
    Preventive,
    Corrective,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1ProductionMaintenanceListResponseRowsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Preventive => serializer.serialize_str("preventive"),
            Self::Corrective => serializer.serialize_str("corrective"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1ProductionMaintenanceListResponseRowsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "preventive" => Ok(Self::Preventive),
            "corrective" => Ok(Self::Corrective),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1ProductionMaintenanceListResponseRowsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preventive => write!(f, "preventive"),
            Self::Corrective => write!(f, "corrective"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
