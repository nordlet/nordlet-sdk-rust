pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnnualAccountsSignaturesUpdateDeclarationsResponseDirectorType {
    ManagingCurrent,
    ManagingFormer,
    SupervisoryCurrent,
    SupervisoryFormer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AnnualAccountsSignaturesUpdateDeclarationsResponseDirectorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ManagingCurrent => serializer.serialize_str("managing_current"),
            Self::ManagingFormer => serializer.serialize_str("managing_former"),
            Self::SupervisoryCurrent => serializer.serialize_str("supervisory_current"),
            Self::SupervisoryFormer => serializer.serialize_str("supervisory_former"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AnnualAccountsSignaturesUpdateDeclarationsResponseDirectorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "managing_current" => Ok(Self::ManagingCurrent),
            "managing_former" => Ok(Self::ManagingFormer),
            "supervisory_current" => Ok(Self::SupervisoryCurrent),
            "supervisory_former" => Ok(Self::SupervisoryFormer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AnnualAccountsSignaturesUpdateDeclarationsResponseDirectorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManagingCurrent => write!(f, "managing_current"),
            Self::ManagingFormer => write!(f, "managing_former"),
            Self::SupervisoryCurrent => write!(f, "supervisory_current"),
            Self::SupervisoryFormer => write!(f, "supervisory_former"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
