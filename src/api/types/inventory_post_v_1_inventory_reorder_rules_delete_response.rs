pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryReorderRulesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1InventoryReorderRulesDeleteResponse {
    pub fn builder() -> PostV1InventoryReorderRulesDeleteResponseBuilder {
        <PostV1InventoryReorderRulesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryReorderRulesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1InventoryReorderRulesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryReorderRulesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1InventoryReorderRulesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1InventoryReorderRulesDeleteResponse, BuildError> {
        Ok(PostV1InventoryReorderRulesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
