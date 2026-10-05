pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompaniesUpdateAccountResponseVatPeriod {
    Monthly,
    Bimonthly,
    Quarterly,
    Semiannual,
    Annual,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CompaniesUpdateAccountResponseVatPeriod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::Bimonthly => serializer.serialize_str("bimonthly"),
            Self::Quarterly => serializer.serialize_str("quarterly"),
            Self::Semiannual => serializer.serialize_str("semiannual"),
            Self::Annual => serializer.serialize_str("annual"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CompaniesUpdateAccountResponseVatPeriod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "monthly" => Ok(Self::Monthly),
            "bimonthly" => Ok(Self::Bimonthly),
            "quarterly" => Ok(Self::Quarterly),
            "semiannual" => Ok(Self::Semiannual),
            "annual" => Ok(Self::Annual),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CompaniesUpdateAccountResponseVatPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Monthly => write!(f, "monthly"),
            Self::Bimonthly => write!(f, "bimonthly"),
            Self::Quarterly => write!(f, "quarterly"),
            Self::Semiannual => write!(f, "semiannual"),
            Self::Annual => write!(f, "annual"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
