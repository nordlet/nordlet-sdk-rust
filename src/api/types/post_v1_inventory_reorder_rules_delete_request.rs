pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryReorderRulesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1InventoryReorderRulesDeleteRequest {
    pub fn builder() -> PostV1InventoryReorderRulesDeleteRequestBuilder {
        <PostV1InventoryReorderRulesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryReorderRulesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1InventoryReorderRulesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryReorderRulesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryReorderRulesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1InventoryReorderRulesDeleteRequest, BuildError> {
        Ok(PostV1InventoryReorderRulesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
