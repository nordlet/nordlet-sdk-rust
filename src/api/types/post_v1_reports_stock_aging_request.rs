pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsStockAgingRequest {
    #[serde(rename = "asOf")]
    #[serde(default)]
    pub as_of: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl PostV1ReportsStockAgingRequest {
    pub fn builder() -> PostV1ReportsStockAgingRequestBuilder {
        <PostV1ReportsStockAgingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsStockAgingRequestBuilder {
    as_of: Option<String>,
    warehouse_id: Option<String>,
}

impl PostV1ReportsStockAgingRequestBuilder {
    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.as_of = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsStockAgingRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of`](PostV1ReportsStockAgingRequestBuilder::as_of)
    pub fn build(self) -> Result<PostV1ReportsStockAgingRequest, BuildError> {
        Ok(PostV1ReportsStockAgingRequest {
            as_of: self
                .as_of
                .ok_or_else(|| BuildError::missing_field("as_of"))?,
            warehouse_id: self.warehouse_id,
        })
    }
}
