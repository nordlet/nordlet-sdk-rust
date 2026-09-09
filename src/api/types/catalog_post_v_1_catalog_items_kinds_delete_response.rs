pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsKindsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogItemsKindsDeleteResponse {
    pub fn builder() -> PostV1CatalogItemsKindsDeleteResponseBuilder {
        <PostV1CatalogItemsKindsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsKindsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1CatalogItemsKindsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsKindsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsKindsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemsKindsDeleteResponse, BuildError> {
        Ok(PostV1CatalogItemsKindsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
