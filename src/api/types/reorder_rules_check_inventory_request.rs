pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReorderRulesCheckInventoryRequest {}

impl ReorderRulesCheckInventoryRequest {
    pub fn builder() -> ReorderRulesCheckInventoryRequestBuilder {
        <ReorderRulesCheckInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReorderRulesCheckInventoryRequestBuilder {}

impl ReorderRulesCheckInventoryRequestBuilder {
    /// Consumes the builder and constructs a [`ReorderRulesCheckInventoryRequest`].
    pub fn build(self) -> Result<ReorderRulesCheckInventoryRequest, BuildError> {
        Ok(ReorderRulesCheckInventoryRequest {})
    }
}
