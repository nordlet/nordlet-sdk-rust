pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1InventorySettingsUpdateResponse {
    #[serde(rename = "negativeStockPolicy")]
    pub negative_stock_policy: PostV1InventorySettingsUpdateResponseNegativeStockPolicy,
}

impl PostV1InventorySettingsUpdateResponse {
    pub fn builder() -> PostV1InventorySettingsUpdateResponseBuilder {
        <PostV1InventorySettingsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventorySettingsUpdateResponseBuilder {
    negative_stock_policy: Option<PostV1InventorySettingsUpdateResponseNegativeStockPolicy>,
}

impl PostV1InventorySettingsUpdateResponseBuilder {
    pub fn negative_stock_policy(
        mut self,
        value: PostV1InventorySettingsUpdateResponseNegativeStockPolicy,
    ) -> Self {
        self.negative_stock_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventorySettingsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`negative_stock_policy`](PostV1InventorySettingsUpdateResponseBuilder::negative_stock_policy)
    pub fn build(self) -> Result<PostV1InventorySettingsUpdateResponse, BuildError> {
        Ok(PostV1InventorySettingsUpdateResponse {
            negative_stock_policy: self
                .negative_stock_policy
                .ok_or_else(|| BuildError::missing_field("negative_stock_policy"))?,
        })
    }
}
