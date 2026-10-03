pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostV1DeclarationsTaxPaymentsListRequestTax {
    CorporateIncomeTax,
    PayrollWithholding,
    Vat,
    SocialInsurance,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostV1DeclarationsTaxPaymentsListRequestTax {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CorporateIncomeTax => serializer.serialize_str("corporate_income_tax"),
            Self::PayrollWithholding => serializer.serialize_str("payroll_withholding"),
            Self::Vat => serializer.serialize_str("vat"),
            Self::SocialInsurance => serializer.serialize_str("social_insurance"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostV1DeclarationsTaxPaymentsListRequestTax {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "corporate_income_tax" => Ok(Self::CorporateIncomeTax),
            "payroll_withholding" => Ok(Self::PayrollWithholding),
            "vat" => Ok(Self::Vat),
            "social_insurance" => Ok(Self::SocialInsurance),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostV1DeclarationsTaxPaymentsListRequestTax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CorporateIncomeTax => write!(f, "corporate_income_tax"),
            Self::PayrollWithholding => write!(f, "payroll_withholding"),
            Self::Vat => write!(f, "vat"),
            Self::SocialInsurance => write!(f, "social_insurance"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
