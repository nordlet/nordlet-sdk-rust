pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemGroupsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogItemGroupsDeleteRequest {
    pub fn builder() -> PostV1CatalogItemGroupsDeleteRequestBuilder {
        <PostV1CatalogItemGroupsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemGroupsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1CatalogItemGroupsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemGroupsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemGroupsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemGroupsDeleteRequest, BuildError> {
        Ok(PostV1CatalogItemGroupsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
