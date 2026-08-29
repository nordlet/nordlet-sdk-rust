pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryReorderRulesCheckRequest {}

impl PostV1InventoryReorderRulesCheckRequest {
    pub fn builder() -> PostV1InventoryReorderRulesCheckRequestBuilder {
        <PostV1InventoryReorderRulesCheckRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryReorderRulesCheckRequestBuilder {}

impl PostV1InventoryReorderRulesCheckRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1InventoryReorderRulesCheckRequest`].
    pub fn build(self) -> Result<PostV1InventoryReorderRulesCheckRequest, BuildError> {
        Ok(PostV1InventoryReorderRulesCheckRequest {})
    }
}
