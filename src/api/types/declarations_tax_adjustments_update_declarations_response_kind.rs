pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaxAdjustmentsUpdateDeclarationsResponseKind {
    NonDeductible,
    IncomeIncrease,
    NonTaxableIncome,
    ExcludedIncome,
    DeductibleAdjustment,
    Donation,
    LossCarriedForward,
    InvestmentRelief,
    ForeignTaxCredit,
    TaxReduction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaxAdjustmentsUpdateDeclarationsResponseKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NonDeductible => serializer.serialize_str("non_deductible"),
            Self::IncomeIncrease => serializer.serialize_str("income_increase"),
            Self::NonTaxableIncome => serializer.serialize_str("non_taxable_income"),
            Self::ExcludedIncome => serializer.serialize_str("excluded_income"),
            Self::DeductibleAdjustment => serializer.serialize_str("deductible_adjustment"),
            Self::Donation => serializer.serialize_str("donation"),
            Self::LossCarriedForward => serializer.serialize_str("loss_carried_forward"),
            Self::InvestmentRelief => serializer.serialize_str("investment_relief"),
            Self::ForeignTaxCredit => serializer.serialize_str("foreign_tax_credit"),
            Self::TaxReduction => serializer.serialize_str("tax_reduction"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaxAdjustmentsUpdateDeclarationsResponseKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "non_deductible" => Ok(Self::NonDeductible),
            "income_increase" => Ok(Self::IncomeIncrease),
            "non_taxable_income" => Ok(Self::NonTaxableIncome),
            "excluded_income" => Ok(Self::ExcludedIncome),
            "deductible_adjustment" => Ok(Self::DeductibleAdjustment),
            "donation" => Ok(Self::Donation),
            "loss_carried_forward" => Ok(Self::LossCarriedForward),
            "investment_relief" => Ok(Self::InvestmentRelief),
            "foreign_tax_credit" => Ok(Self::ForeignTaxCredit),
            "tax_reduction" => Ok(Self::TaxReduction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaxAdjustmentsUpdateDeclarationsResponseKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonDeductible => write!(f, "non_deductible"),
            Self::IncomeIncrease => write!(f, "income_increase"),
            Self::NonTaxableIncome => write!(f, "non_taxable_income"),
            Self::ExcludedIncome => write!(f, "excluded_income"),
            Self::DeductibleAdjustment => write!(f, "deductible_adjustment"),
            Self::Donation => write!(f, "donation"),
            Self::LossCarriedForward => write!(f, "loss_carried_forward"),
            Self::InvestmentRelief => write!(f, "investment_relief"),
            Self::ForeignTaxCredit => write!(f, "foreign_tax_credit"),
            Self::TaxReduction => write!(f, "tax_reduction"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
