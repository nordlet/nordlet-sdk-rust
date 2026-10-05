pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockBalanceReportsRequest {
    #[serde(rename = "asOf")]
    #[serde(default)]
    pub as_of: NaiveDate,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl StockBalanceReportsRequest {
    pub fn builder() -> StockBalanceReportsRequestBuilder {
        <StockBalanceReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockBalanceReportsRequestBuilder {
    as_of: Option<NaiveDate>,
    warehouse_id: Option<String>,
}

impl StockBalanceReportsRequestBuilder {
    pub fn as_of(mut self, value: NaiveDate) -> Self {
        self.as_of = Some(value);
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockBalanceReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of`](StockBalanceReportsRequestBuilder::as_of)
    pub fn build(self) -> Result<StockBalanceReportsRequest, BuildError> {
        Ok(StockBalanceReportsRequest {
            as_of: self
                .as_of
                .ok_or_else(|| BuildError::missing_field("as_of"))?,
            warehouse_id: self.warehouse_id,
        })
    }
}
