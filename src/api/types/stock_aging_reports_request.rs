pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockAgingReportsRequest {
    #[serde(rename = "asOf")]
    #[serde(default)]
    pub as_of: NaiveDate,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl StockAgingReportsRequest {
    pub fn builder() -> StockAgingReportsRequestBuilder {
        <StockAgingReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockAgingReportsRequestBuilder {
    as_of: Option<NaiveDate>,
    warehouse_id: Option<String>,
}

impl StockAgingReportsRequestBuilder {
    pub fn as_of(mut self, value: NaiveDate) -> Self {
        self.as_of = Some(value);
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockAgingReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of`](StockAgingReportsRequestBuilder::as_of)
    pub fn build(self) -> Result<StockAgingReportsRequest, BuildError> {
        Ok(StockAgingReportsRequest {
            as_of: self
                .as_of
                .ok_or_else(|| BuildError::missing_field("as_of"))?,
            warehouse_id: self.warehouse_id,
        })
    }
}
