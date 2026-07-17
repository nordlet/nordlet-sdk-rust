pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsSuppliersDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogItemsSuppliersDeleteRequest {
    pub fn builder() -> PostV1CatalogItemsSuppliersDeleteRequestBuilder {
        <PostV1CatalogItemsSuppliersDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsSuppliersDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1CatalogItemsSuppliersDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsSuppliersDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsSuppliersDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemsSuppliersDeleteRequest, BuildError> {
        Ok(PostV1CatalogItemsSuppliersDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
