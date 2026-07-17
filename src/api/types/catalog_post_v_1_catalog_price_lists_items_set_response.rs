pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogPriceListsItemsSetResponse {
    #[serde(default)]
    pub updated: i64,
}

impl PostV1CatalogPriceListsItemsSetResponse {
    pub fn builder() -> PostV1CatalogPriceListsItemsSetResponseBuilder {
        <PostV1CatalogPriceListsItemsSetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogPriceListsItemsSetResponseBuilder {
    updated: Option<i64>,
}

impl PostV1CatalogPriceListsItemsSetResponseBuilder {
    pub fn updated(mut self, value: i64) -> Self {
        self.updated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogPriceListsItemsSetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`updated`](PostV1CatalogPriceListsItemsSetResponseBuilder::updated)
    pub fn build(self) -> Result<PostV1CatalogPriceListsItemsSetResponse, BuildError> {
        Ok(PostV1CatalogPriceListsItemsSetResponse {
            updated: self
                .updated
                .ok_or_else(|| BuildError::missing_field("updated"))?,
        })
    }
}
