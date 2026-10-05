pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubmissionsRetryDeclarationsResponseEnvironment {
    Test,
    Production,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubmissionsRetryDeclarationsResponseEnvironment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Test => serializer.serialize_str("test"),
            Self::Production => serializer.serialize_str("production"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubmissionsRetryDeclarationsResponseEnvironment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "test" => Ok(Self::Test),
            "production" => Ok(Self::Production),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubmissionsRetryDeclarationsResponseEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Test => write!(f, "test"),
            Self::Production => write!(f, "production"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
