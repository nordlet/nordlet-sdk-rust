pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogItemsDeleteRequest {
    pub fn builder() -> PostV1CatalogItemsDeleteRequestBuilder {
        <PostV1CatalogItemsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1CatalogItemsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemsDeleteRequest, BuildError> {
        Ok(PostV1CatalogItemsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
