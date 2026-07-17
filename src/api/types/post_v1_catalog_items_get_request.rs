pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogItemsGetRequest {
    pub fn builder() -> PostV1CatalogItemsGetRequestBuilder {
        <PostV1CatalogItemsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1CatalogItemsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemsGetRequest, BuildError> {
        Ok(PostV1CatalogItemsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
