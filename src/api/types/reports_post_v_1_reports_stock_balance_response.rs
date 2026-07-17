pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsStockBalanceResponse {
    #[serde(rename = "asOf")]
    #[serde(default)]
    pub as_of: String,
    #[serde(default)]
    pub rows: Vec<PostV1ReportsStockBalanceResponseRowsItem>,
    #[serde(rename = "totalValue")]
    #[serde(default)]
    pub total_value: String,
}

impl PostV1ReportsStockBalanceResponse {
    pub fn builder() -> PostV1ReportsStockBalanceResponseBuilder {
        <PostV1ReportsStockBalanceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsStockBalanceResponseBuilder {
    as_of: Option<String>,
    rows: Option<Vec<PostV1ReportsStockBalanceResponseRowsItem>>,
    total_value: Option<String>,
}

impl PostV1ReportsStockBalanceResponseBuilder {
    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.as_of = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1ReportsStockBalanceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn total_value(mut self, value: impl Into<String>) -> Self {
        self.total_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsStockBalanceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of`](PostV1ReportsStockBalanceResponseBuilder::as_of)
    /// - [`rows`](PostV1ReportsStockBalanceResponseBuilder::rows)
    /// - [`total_value`](PostV1ReportsStockBalanceResponseBuilder::total_value)
    pub fn build(self) -> Result<PostV1ReportsStockBalanceResponse, BuildError> {
        Ok(PostV1ReportsStockBalanceResponse {
            as_of: self
                .as_of
                .ok_or_else(|| BuildError::missing_field("as_of"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            total_value: self
                .total_value
                .ok_or_else(|| BuildError::missing_field("total_value"))?,
        })
    }
}
