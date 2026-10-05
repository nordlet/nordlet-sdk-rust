pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StockListEcommerceRequest {
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl StockListEcommerceRequest {
    pub fn builder() -> StockListEcommerceRequestBuilder {
        <StockListEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StockListEcommerceRequestBuilder {
    warehouse_id: Option<String>,
}

impl StockListEcommerceRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StockListEcommerceRequest`].
    pub fn build(self) -> Result<StockListEcommerceRequest, BuildError> {
        Ok(StockListEcommerceRequest {
            warehouse_id: self.warehouse_id,
        })
    }
}
