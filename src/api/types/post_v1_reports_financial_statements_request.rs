pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsFinancialStatementsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<PostV1ReportsFinancialStatementsRequestCategory>,
}

impl PostV1ReportsFinancialStatementsRequest {
    pub fn builder() -> PostV1ReportsFinancialStatementsRequestBuilder {
        <PostV1ReportsFinancialStatementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsFinancialStatementsRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    category: Option<PostV1ReportsFinancialStatementsRequestCategory>,
}

impl PostV1ReportsFinancialStatementsRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn category(mut self, value: PostV1ReportsFinancialStatementsRequestCategory) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsFinancialStatementsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsFinancialStatementsRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsFinancialStatementsRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsFinancialStatementsRequest, BuildError> {
        Ok(PostV1ReportsFinancialStatementsRequest {
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
