pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsDeReturnFactsGetResponseFactsContributionsItemKind {
    Cash,
    InKind,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsDeReturnFactsGetResponseFactsContributionsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Cash => serializer.serialize_str("cash"),
            Self::InKind => serializer.serialize_str("in_kind"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for PostV1DeclarationsDeReturnFactsGetResponseFactsContributionsItemKind
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cash" => Ok(Self::Cash),
            "in_kind" => Ok(Self::InKind),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsDeReturnFactsGetResponseFactsContributionsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cash => write!(f, "cash"),
            Self::InKind => write!(f, "in_kind"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
