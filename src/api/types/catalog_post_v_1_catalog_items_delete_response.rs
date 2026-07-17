pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogItemsDeleteResponse {
    pub fn builder() -> PostV1CatalogItemsDeleteResponseBuilder {
        <PostV1CatalogItemsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1CatalogItemsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemsDeleteResponse, BuildError> {
        Ok(PostV1CatalogItemsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
