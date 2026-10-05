pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SettingsUpdateInventoryRequest {
    #[serde(rename = "negativeStockPolicy")]
    pub negative_stock_policy: SettingsUpdateInventoryRequestNegativeStockPolicy,
}

impl SettingsUpdateInventoryRequest {
    pub fn builder() -> SettingsUpdateInventoryRequestBuilder {
        <SettingsUpdateInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsUpdateInventoryRequestBuilder {
    negative_stock_policy: Option<SettingsUpdateInventoryRequestNegativeStockPolicy>,
}

impl SettingsUpdateInventoryRequestBuilder {
    pub fn negative_stock_policy(
        mut self,
        value: SettingsUpdateInventoryRequestNegativeStockPolicy,
    ) -> Self {
        self.negative_stock_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsUpdateInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`negative_stock_policy`](SettingsUpdateInventoryRequestBuilder::negative_stock_policy)
    pub fn build(self) -> Result<SettingsUpdateInventoryRequest, BuildError> {
        Ok(SettingsUpdateInventoryRequest {
            negative_stock_policy: self
                .negative_stock_policy
                .ok_or_else(|| BuildError::missing_field("negative_stock_policy"))?,
        })
    }
}
