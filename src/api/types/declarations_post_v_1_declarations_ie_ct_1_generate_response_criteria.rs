pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1DeclarationsIeCt1GenerateResponseCriteria {
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

impl PostV1DeclarationsIeCt1GenerateResponseCriteria {
    pub fn builder() -> PostV1DeclarationsIeCt1GenerateResponseCriteriaBuilder {
        <PostV1DeclarationsIeCt1GenerateResponseCriteriaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeCt1GenerateResponseCriteriaBuilder {
    balance_sheet_total: Option<String>,
    turnover: Option<String>,
    average_employees: Option<f64>,
}

impl PostV1DeclarationsIeCt1GenerateResponseCriteriaBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeCt1GenerateResponseCriteria`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance_sheet_total`](PostV1DeclarationsIeCt1GenerateResponseCriteriaBuilder::balance_sheet_total)
    /// - [`turnover`](PostV1DeclarationsIeCt1GenerateResponseCriteriaBuilder::turnover)
    /// - [`average_employees`](PostV1DeclarationsIeCt1GenerateResponseCriteriaBuilder::average_employees)
    pub fn build(self) -> Result<PostV1DeclarationsIeCt1GenerateResponseCriteria, BuildError> {
        Ok(PostV1DeclarationsIeCt1GenerateResponseCriteria {
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
