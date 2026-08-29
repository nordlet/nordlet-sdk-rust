pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1InventoryReorderRulesCheckResponse {
    #[serde(default)]
    pub rows: Vec<PostV1InventoryReorderRulesCheckResponseRowsItem>,
}

impl PostV1InventoryReorderRulesCheckResponse {
    pub fn builder() -> PostV1InventoryReorderRulesCheckResponseBuilder {
        <PostV1InventoryReorderRulesCheckResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1InventoryReorderRulesCheckResponseBuilder {
    rows: Option<Vec<PostV1InventoryReorderRulesCheckResponseRowsItem>>,
}

impl PostV1InventoryReorderRulesCheckResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1InventoryReorderRulesCheckResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1InventoryReorderRulesCheckResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1InventoryReorderRulesCheckResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1InventoryReorderRulesCheckResponse, BuildError> {
        Ok(PostV1InventoryReorderRulesCheckResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
