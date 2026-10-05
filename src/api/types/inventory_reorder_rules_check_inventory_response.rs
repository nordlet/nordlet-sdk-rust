pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReorderRulesCheckInventoryResponse {
    #[serde(default)]
    pub rows: Vec<ReorderRulesCheckInventoryResponseRowsItem>,
}

impl ReorderRulesCheckInventoryResponse {
    pub fn builder() -> ReorderRulesCheckInventoryResponseBuilder {
        <ReorderRulesCheckInventoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReorderRulesCheckInventoryResponseBuilder {
    rows: Option<Vec<ReorderRulesCheckInventoryResponseRowsItem>>,
}

impl ReorderRulesCheckInventoryResponseBuilder {
    pub fn rows(mut self, value: Vec<ReorderRulesCheckInventoryResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReorderRulesCheckInventoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ReorderRulesCheckInventoryResponseBuilder::rows)
    pub fn build(self) -> Result<ReorderRulesCheckInventoryResponse, BuildError> {
        Ok(ReorderRulesCheckInventoryResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
