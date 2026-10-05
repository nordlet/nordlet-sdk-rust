pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PriceListsItemsSetCatalogResponse {
    #[serde(default)]
    pub updated: i64,
}

impl PriceListsItemsSetCatalogResponse {
    pub fn builder() -> PriceListsItemsSetCatalogResponseBuilder {
        <PriceListsItemsSetCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceListsItemsSetCatalogResponseBuilder {
    updated: Option<i64>,
}

impl PriceListsItemsSetCatalogResponseBuilder {
    pub fn updated(mut self, value: i64) -> Self {
        self.updated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceListsItemsSetCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`updated`](PriceListsItemsSetCatalogResponseBuilder::updated)
    pub fn build(self) -> Result<PriceListsItemsSetCatalogResponse, BuildError> {
        Ok(PriceListsItemsSetCatalogResponse {
            updated: self
                .updated
                .ok_or_else(|| BuildError::missing_field("updated"))?,
        })
    }
}
