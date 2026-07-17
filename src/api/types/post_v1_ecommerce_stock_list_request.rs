pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceStockListRequest {
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl PostV1EcommerceStockListRequest {
    pub fn builder() -> PostV1EcommerceStockListRequestBuilder {
        <PostV1EcommerceStockListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceStockListRequestBuilder {
    warehouse_id: Option<String>,
}

impl PostV1EcommerceStockListRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceStockListRequest`].
    pub fn build(self) -> Result<PostV1EcommerceStockListRequest, BuildError> {
        Ok(PostV1EcommerceStockListRequest {
            warehouse_id: self.warehouse_id,
        })
    }
}
