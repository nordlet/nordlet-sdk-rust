pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceOrdersReserveRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl PostV1EcommerceOrdersReserveRequest {
    pub fn builder() -> PostV1EcommerceOrdersReserveRequestBuilder {
        <PostV1EcommerceOrdersReserveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceOrdersReserveRequestBuilder {
    id: Option<String>,
    warehouse_id: Option<String>,
}

impl PostV1EcommerceOrdersReserveRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceOrdersReserveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1EcommerceOrdersReserveRequestBuilder::id)
    pub fn build(self) -> Result<PostV1EcommerceOrdersReserveRequest, BuildError> {
        Ok(PostV1EcommerceOrdersReserveRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            warehouse_id: self.warehouse_id,
        })
    }
}
