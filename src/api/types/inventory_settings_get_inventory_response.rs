pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SettingsGetInventoryResponse {
    #[serde(rename = "negativeStockPolicy")]
    pub negative_stock_policy: SettingsGetInventoryResponseNegativeStockPolicy,
}

impl SettingsGetInventoryResponse {
    pub fn builder() -> SettingsGetInventoryResponseBuilder {
        <SettingsGetInventoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsGetInventoryResponseBuilder {
    negative_stock_policy: Option<SettingsGetInventoryResponseNegativeStockPolicy>,
}

impl SettingsGetInventoryResponseBuilder {
    pub fn negative_stock_policy(
        mut self,
        value: SettingsGetInventoryResponseNegativeStockPolicy,
    ) -> Self {
        self.negative_stock_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsGetInventoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`negative_stock_policy`](SettingsGetInventoryResponseBuilder::negative_stock_policy)
    pub fn build(self) -> Result<SettingsGetInventoryResponse, BuildError> {
        Ok(SettingsGetInventoryResponse {
            negative_stock_policy: self
                .negative_stock_policy
                .ok_or_else(|| BuildError::missing_field("negative_stock_policy"))?,
        })
    }
}
