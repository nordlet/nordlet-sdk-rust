pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeReturnsGenerateDeclarationsRequestRuleKey {
    DeEBilanz,
    DeCitReturn,
    DeTradeTax,
    DeTradeTaxApportionment,
    DeAnnualVatReturn,
    DePayrollWithholding,
    DePayrollStatements,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DeReturnsGenerateDeclarationsRequestRuleKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DeEBilanz => serializer.serialize_str("de-e-bilanz"),
            Self::DeCitReturn => serializer.serialize_str("de-cit-return"),
            Self::DeTradeTax => serializer.serialize_str("de-trade-tax"),
            Self::DeTradeTaxApportionment => serializer.serialize_str("de-trade-tax-apportionment"),
            Self::DeAnnualVatReturn => serializer.serialize_str("de-annual-vat-return"),
            Self::DePayrollWithholding => serializer.serialize_str("de-payroll-withholding"),
            Self::DePayrollStatements => serializer.serialize_str("de-payroll-statements"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DeReturnsGenerateDeclarationsRequestRuleKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "de-e-bilanz" => Ok(Self::DeEBilanz),
            "de-cit-return" => Ok(Self::DeCitReturn),
            "de-trade-tax" => Ok(Self::DeTradeTax),
            "de-trade-tax-apportionment" => Ok(Self::DeTradeTaxApportionment),
            "de-annual-vat-return" => Ok(Self::DeAnnualVatReturn),
            "de-payroll-withholding" => Ok(Self::DePayrollWithholding),
            "de-payroll-statements" => Ok(Self::DePayrollStatements),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DeReturnsGenerateDeclarationsRequestRuleKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeEBilanz => write!(f, "de-e-bilanz"),
            Self::DeCitReturn => write!(f, "de-cit-return"),
            Self::DeTradeTax => write!(f, "de-trade-tax"),
            Self::DeTradeTaxApportionment => write!(f, "de-trade-tax-apportionment"),
            Self::DeAnnualVatReturn => write!(f, "de-annual-vat-return"),
            Self::DePayrollWithholding => write!(f, "de-payroll-withholding"),
            Self::DePayrollStatements => write!(f, "de-payroll-statements"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
