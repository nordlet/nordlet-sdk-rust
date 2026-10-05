pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReorderRulesDeleteInventoryRequest {
    #[serde(default)]
    pub id: String,
}

impl ReorderRulesDeleteInventoryRequest {
    pub fn builder() -> ReorderRulesDeleteInventoryRequestBuilder {
        <ReorderRulesDeleteInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReorderRulesDeleteInventoryRequestBuilder {
    id: Option<String>,
}

impl ReorderRulesDeleteInventoryRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReorderRulesDeleteInventoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReorderRulesDeleteInventoryRequestBuilder::id)
    pub fn build(self) -> Result<ReorderRulesDeleteInventoryRequest, BuildError> {
        Ok(ReorderRulesDeleteInventoryRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
