pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReorderRulesDeleteInventoryResponse {
    #[serde(default)]
    pub id: String,
}

impl ReorderRulesDeleteInventoryResponse {
    pub fn builder() -> ReorderRulesDeleteInventoryResponseBuilder {
        <ReorderRulesDeleteInventoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReorderRulesDeleteInventoryResponseBuilder {
    id: Option<String>,
}

impl ReorderRulesDeleteInventoryResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReorderRulesDeleteInventoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReorderRulesDeleteInventoryResponseBuilder::id)
    pub fn build(self) -> Result<ReorderRulesDeleteInventoryResponse, BuildError> {
        Ok(ReorderRulesDeleteInventoryResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
