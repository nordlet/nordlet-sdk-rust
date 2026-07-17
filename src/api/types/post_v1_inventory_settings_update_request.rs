pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1InventorySettingsUpdateRequest {
    #[serde(rename = "negativeStockPolicy")]
    pub negative_stock_policy: PostV1InventorySettingsUpdateRequestNegativeStockPolicy,
}

impl PostV1InventorySettingsUpdateRequest {
    pub fn builder() -> PostV1InventorySettingsUpdateRequestBuilder {
        <PostV1InventorySettingsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventorySettingsUpdateRequestBuilder {
    negative_stock_policy: Option<PostV1InventorySettingsUpdateRequestNegativeStockPolicy>,
}

impl PostV1InventorySettingsUpdateRequestBuilder {
    pub fn negative_stock_policy(
        mut self,
        value: PostV1InventorySettingsUpdateRequestNegativeStockPolicy,
    ) -> Self {
        self.negative_stock_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventorySettingsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`negative_stock_policy`](PostV1InventorySettingsUpdateRequestBuilder::negative_stock_policy)
    pub fn build(self) -> Result<PostV1InventorySettingsUpdateRequest, BuildError> {
        Ok(PostV1InventorySettingsUpdateRequest {
            negative_stock_policy: self
                .negative_stock_policy
                .ok_or_else(|| BuildError::missing_field("negative_stock_policy"))?,
        })
    }
}
