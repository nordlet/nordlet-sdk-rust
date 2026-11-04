pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUnitsListCatalogResponse {
    #[serde(default)]
    pub rows: Vec<ItemsUnitsListCatalogResponseRowsItem>,
}

impl ItemsUnitsListCatalogResponse {
    pub fn builder() -> ItemsUnitsListCatalogResponseBuilder {
        <ItemsUnitsListCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUnitsListCatalogResponseBuilder {
    rows: Option<Vec<ItemsUnitsListCatalogResponseRowsItem>>,
}

impl ItemsUnitsListCatalogResponseBuilder {
    pub fn rows(mut self, value: Vec<ItemsUnitsListCatalogResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsUnitsListCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ItemsUnitsListCatalogResponseBuilder::rows)
    pub fn build(self) -> Result<ItemsUnitsListCatalogResponse, BuildError> {
        Ok(ItemsUnitsListCatalogResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
