pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockShortageReportsRequest {
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl StockShortageReportsRequest {
    pub fn builder() -> StockShortageReportsRequestBuilder {
        <StockShortageReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockShortageReportsRequestBuilder {
    warehouse_id: Option<String>,
}

impl StockShortageReportsRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockShortageReportsRequest`].
    pub fn build(self) -> Result<StockShortageReportsRequest, BuildError> {
        Ok(StockShortageReportsRequest {
            warehouse_id: self.warehouse_id,
        })
    }
}
