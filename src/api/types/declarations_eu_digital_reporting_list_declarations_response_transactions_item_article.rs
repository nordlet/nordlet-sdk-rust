pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EuDigitalReportingListDeclarationsResponseTransactionsItemArticle {
    TwoHundredSixtyTwo1A,
    TwoHundredSixtyTwo1B,
    TwoHundredSixtyTwo1C,
    TwoHundredSixtyTwo1D,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EuDigitalReportingListDeclarationsResponseTransactionsItemArticle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TwoHundredSixtyTwo1A => serializer.serialize_str("262(1)(a)"),
            Self::TwoHundredSixtyTwo1B => serializer.serialize_str("262(1)(b)"),
            Self::TwoHundredSixtyTwo1C => serializer.serialize_str("262(1)(c)"),
            Self::TwoHundredSixtyTwo1D => serializer.serialize_str("262(1)(d)"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EuDigitalReportingListDeclarationsResponseTransactionsItemArticle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "262(1)(a)" => Ok(Self::TwoHundredSixtyTwo1A),
            "262(1)(b)" => Ok(Self::TwoHundredSixtyTwo1B),
            "262(1)(c)" => Ok(Self::TwoHundredSixtyTwo1C),
            "262(1)(d)" => Ok(Self::TwoHundredSixtyTwo1D),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EuDigitalReportingListDeclarationsResponseTransactionsItemArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TwoHundredSixtyTwo1A => write!(f, "262(1)(a)"),
            Self::TwoHundredSixtyTwo1B => write!(f, "262(1)(b)"),
            Self::TwoHundredSixtyTwo1C => write!(f, "262(1)(c)"),
            Self::TwoHundredSixtyTwo1D => write!(f, "262(1)(d)"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
