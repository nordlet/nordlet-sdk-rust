pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IeCt1GenerateDeclarationsResponseCriteria {
    #[serde(rename = "balanceSheetTotal")]
    #[serde(default)]
    pub balance_sheet_total: String,
    #[serde(default)]
    pub turnover: String,
    #[serde(rename = "averageEmployees")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub average_employees: f64,
}

impl IeCt1GenerateDeclarationsResponseCriteria {
    pub fn builder() -> IeCt1GenerateDeclarationsResponseCriteriaBuilder {
        <IeCt1GenerateDeclarationsResponseCriteriaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeCt1GenerateDeclarationsResponseCriteriaBuilder {
    balance_sheet_total: Option<String>,
    turnover: Option<String>,
    average_employees: Option<f64>,
}

impl IeCt1GenerateDeclarationsResponseCriteriaBuilder {
    pub fn balance_sheet_total(mut self, value: impl Into<String>) -> Self {
        self.balance_sheet_total = Some(value.into());
        self
    }

    pub fn turnover(mut self, value: impl Into<String>) -> Self {
        self.turnover = Some(value.into());
        self
    }

    pub fn average_employees(mut self, value: f64) -> Self {
        self.average_employees = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IeCt1GenerateDeclarationsResponseCriteria`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance_sheet_total`](IeCt1GenerateDeclarationsResponseCriteriaBuilder::balance_sheet_total)
    /// - [`turnover`](IeCt1GenerateDeclarationsResponseCriteriaBuilder::turnover)
    /// - [`average_employees`](IeCt1GenerateDeclarationsResponseCriteriaBuilder::average_employees)
    pub fn build(self) -> Result<IeCt1GenerateDeclarationsResponseCriteria, BuildError> {
        Ok(IeCt1GenerateDeclarationsResponseCriteria {
            balance_sheet_total: self
                .balance_sheet_total
                .ok_or_else(|| BuildError::missing_field("balance_sheet_total"))?,
            turnover: self
                .turnover
                .ok_or_else(|| BuildError::missing_field("turnover"))?,
            average_employees: self
                .average_employees
                .ok_or_else(|| BuildError::missing_field("average_employees"))?,
        })
    }
}
