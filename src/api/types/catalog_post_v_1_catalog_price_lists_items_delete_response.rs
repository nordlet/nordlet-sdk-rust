pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsItemsDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1CatalogPriceListsItemsDeleteResponse {
    pub fn builder() -> PostV1CatalogPriceListsItemsDeleteResponseBuilder {
        <PostV1CatalogPriceListsItemsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsItemsDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1CatalogPriceListsItemsDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsItemsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1CatalogPriceListsItemsDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1CatalogPriceListsItemsDeleteResponse, BuildError> {
        Ok(PostV1CatalogPriceListsItemsDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
