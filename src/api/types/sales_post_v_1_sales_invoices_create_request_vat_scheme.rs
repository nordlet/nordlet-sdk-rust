pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1SalesInvoicesCreateRequestVatScheme {
    Domestic,
    IntraEuB2B,
    ReverseCharge,
    OssUnion,
    Ioss,
    MarketplaceDeemed,
    Export,
    OutOfScope,
    SmeExempt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1SalesInvoicesCreateRequestVatScheme {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Domestic => serializer.serialize_str("domestic"),
            Self::IntraEuB2B => serializer.serialize_str("intra_eu_b2b"),
            Self::ReverseCharge => serializer.serialize_str("reverse_charge"),
            Self::OssUnion => serializer.serialize_str("oss_union"),
            Self::Ioss => serializer.serialize_str("ioss"),
            Self::MarketplaceDeemed => serializer.serialize_str("marketplace_deemed"),
            Self::Export => serializer.serialize_str("export"),
            Self::OutOfScope => serializer.serialize_str("out_of_scope"),
            Self::SmeExempt => serializer.serialize_str("sme_exempt"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1SalesInvoicesCreateRequestVatScheme {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "domestic" => Ok(Self::Domestic),
            "intra_eu_b2b" => Ok(Self::IntraEuB2B),
            "reverse_charge" => Ok(Self::ReverseCharge),
            "oss_union" => Ok(Self::OssUnion),
            "ioss" => Ok(Self::Ioss),
            "marketplace_deemed" => Ok(Self::MarketplaceDeemed),
            "export" => Ok(Self::Export),
            "out_of_scope" => Ok(Self::OutOfScope),
            "sme_exempt" => Ok(Self::SmeExempt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1SalesInvoicesCreateRequestVatScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domestic => write!(f, "domestic"),
            Self::IntraEuB2B => write!(f, "intra_eu_b2b"),
            Self::ReverseCharge => write!(f, "reverse_charge"),
            Self::OssUnion => write!(f, "oss_union"),
            Self::Ioss => write!(f, "ioss"),
            Self::MarketplaceDeemed => write!(f, "marketplace_deemed"),
            Self::Export => write!(f, "export"),
            Self::OutOfScope => write!(f, "out_of_scope"),
            Self::SmeExempt => write!(f, "sme_exempt"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
