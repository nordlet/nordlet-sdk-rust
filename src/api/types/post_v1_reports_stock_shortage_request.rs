pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsStockShortageRequest {
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl PostV1ReportsStockShortageRequest {
    pub fn builder() -> PostV1ReportsStockShortageRequestBuilder {
        <PostV1ReportsStockShortageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsStockShortageRequestBuilder {
    warehouse_id: Option<String>,
}

impl PostV1ReportsStockShortageRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsStockShortageRequest`].
    pub fn build(self) -> Result<PostV1ReportsStockShortageRequest, BuildError> {
        Ok(PostV1ReportsStockShortageRequest {
            warehouse_id: self.warehouse_id,
        })
    }
}
