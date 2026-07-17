pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1InventorySettingsGetResponse {
    #[serde(rename = "negativeStockPolicy")]
    pub negative_stock_policy: PostV1InventorySettingsGetResponseNegativeStockPolicy,
}

impl PostV1InventorySettingsGetResponse {
    pub fn builder() -> PostV1InventorySettingsGetResponseBuilder {
        <PostV1InventorySettingsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventorySettingsGetResponseBuilder {
    negative_stock_policy: Option<PostV1InventorySettingsGetResponseNegativeStockPolicy>,
}

impl PostV1InventorySettingsGetResponseBuilder {
    pub fn negative_stock_policy(
        mut self,
        value: PostV1InventorySettingsGetResponseNegativeStockPolicy,
    ) -> Self {
        self.negative_stock_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventorySettingsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`negative_stock_policy`](PostV1InventorySettingsGetResponseBuilder::negative_stock_policy)
    pub fn build(self) -> Result<PostV1InventorySettingsGetResponse, BuildError> {
        Ok(PostV1InventorySettingsGetResponse {
            negative_stock_policy: self
                .negative_stock_policy
                .ok_or_else(|| BuildError::missing_field("negative_stock_policy"))?,
        })
    }
}
