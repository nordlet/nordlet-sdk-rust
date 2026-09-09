pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsKindsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogItemsKindsDeleteRequest {
    pub fn builder() -> PostV1CatalogItemsKindsDeleteRequestBuilder {
        <PostV1CatalogItemsKindsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsKindsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1CatalogItemsKindsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsKindsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsKindsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemsKindsDeleteRequest, BuildError> {
        Ok(PostV1CatalogItemsKindsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
