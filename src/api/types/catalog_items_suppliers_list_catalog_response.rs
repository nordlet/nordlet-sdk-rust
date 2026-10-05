pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsSuppliersListCatalogResponse {
    #[serde(default)]
    pub rows: Vec<ItemsSuppliersListCatalogResponseRowsItem>,
}

impl ItemsSuppliersListCatalogResponse {
    pub fn builder() -> ItemsSuppliersListCatalogResponseBuilder {
        <ItemsSuppliersListCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsSuppliersListCatalogResponseBuilder {
    rows: Option<Vec<ItemsSuppliersListCatalogResponseRowsItem>>,
}

impl ItemsSuppliersListCatalogResponseBuilder {
    pub fn rows(mut self, value: Vec<ItemsSuppliersListCatalogResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsSuppliersListCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ItemsSuppliersListCatalogResponseBuilder::rows)
    pub fn build(self) -> Result<ItemsSuppliersListCatalogResponse, BuildError> {
        Ok(ItemsSuppliersListCatalogResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
