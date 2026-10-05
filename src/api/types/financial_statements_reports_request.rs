pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinancialStatementsReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<FinancialStatementsReportsRequestCategory>,
}

impl FinancialStatementsReportsRequest {
    pub fn builder() -> FinancialStatementsReportsRequestBuilder {
        <FinancialStatementsReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancialStatementsReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    category: Option<FinancialStatementsReportsRequestCategory>,
}

impl FinancialStatementsReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn category(mut self, value: FinancialStatementsReportsRequestCategory) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinancialStatementsReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](FinancialStatementsReportsRequestBuilder::from_date)
    /// - [`to_date`](FinancialStatementsReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<FinancialStatementsReportsRequest, BuildError> {
        Ok(FinancialStatementsReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            category: self.category,
        })
    }
}
