pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SettingsUpdateInventoryResponse {
    #[serde(rename = "negativeStockPolicy")]
    pub negative_stock_policy: SettingsUpdateInventoryResponseNegativeStockPolicy,
}

impl SettingsUpdateInventoryResponse {
    pub fn builder() -> SettingsUpdateInventoryResponseBuilder {
        <SettingsUpdateInventoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsUpdateInventoryResponseBuilder {
    negative_stock_policy: Option<SettingsUpdateInventoryResponseNegativeStockPolicy>,
}

impl SettingsUpdateInventoryResponseBuilder {
    pub fn negative_stock_policy(
        mut self,
        value: SettingsUpdateInventoryResponseNegativeStockPolicy,
    ) -> Self {
        self.negative_stock_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsUpdateInventoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`negative_stock_policy`](SettingsUpdateInventoryResponseBuilder::negative_stock_policy)
    pub fn build(self) -> Result<SettingsUpdateInventoryResponse, BuildError> {
        Ok(SettingsUpdateInventoryResponse {
            negative_stock_policy: self
                .negative_stock_policy
                .ok_or_else(|| BuildError::missing_field("negative_stock_policy"))?,
        })
    }
}
