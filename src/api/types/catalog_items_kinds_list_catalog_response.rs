pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsKindsListCatalogResponse {
    #[serde(default)]
    pub rows: Vec<ItemsKindsListCatalogResponseRowsItem>,
}

impl ItemsKindsListCatalogResponse {
    pub fn builder() -> ItemsKindsListCatalogResponseBuilder {
        <ItemsKindsListCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsKindsListCatalogResponseBuilder {
    rows: Option<Vec<ItemsKindsListCatalogResponseRowsItem>>,
}

impl ItemsKindsListCatalogResponseBuilder {
    pub fn rows(mut self, value: Vec<ItemsKindsListCatalogResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsKindsListCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ItemsKindsListCatalogResponseBuilder::rows)
    pub fn build(self) -> Result<ItemsKindsListCatalogResponse, BuildError> {
        Ok(ItemsKindsListCatalogResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
