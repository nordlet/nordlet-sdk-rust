pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1HrEmployeesRecordsCreateRequestType {
    Education,
    Qualification,
    Certificate,
    Training,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1HrEmployeesRecordsCreateRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Education => serializer.serialize_str("education"),
            Self::Qualification => serializer.serialize_str("qualification"),
            Self::Certificate => serializer.serialize_str("certificate"),
            Self::Training => serializer.serialize_str("training"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1HrEmployeesRecordsCreateRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "education" => Ok(Self::Education),
            "qualification" => Ok(Self::Qualification),
            "certificate" => Ok(Self::Certificate),
            "training" => Ok(Self::Training),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1HrEmployeesRecordsCreateRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Education => write!(f, "education"),
            Self::Qualification => write!(f, "qualification"),
            Self::Certificate => write!(f, "certificate"),
            Self::Training => write!(f, "training"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
