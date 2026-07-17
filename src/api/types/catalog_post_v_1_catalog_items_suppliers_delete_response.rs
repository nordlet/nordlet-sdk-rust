pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsSuppliersDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogItemsSuppliersDeleteResponse {
    pub fn builder() -> PostV1CatalogItemsSuppliersDeleteResponseBuilder {
        <PostV1CatalogItemsSuppliersDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsSuppliersDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1CatalogItemsSuppliersDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsSuppliersDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsSuppliersDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemsSuppliersDeleteResponse, BuildError> {
        Ok(PostV1CatalogItemsSuppliersDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
