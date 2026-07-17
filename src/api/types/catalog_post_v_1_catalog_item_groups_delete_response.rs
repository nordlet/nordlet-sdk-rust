pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemGroupsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogItemGroupsDeleteResponse {
    pub fn builder() -> PostV1CatalogItemGroupsDeleteResponseBuilder {
        <PostV1CatalogItemGroupsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemGroupsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1CatalogItemGroupsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemGroupsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemGroupsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemGroupsDeleteResponse, BuildError> {
        Ok(PostV1CatalogItemGroupsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
