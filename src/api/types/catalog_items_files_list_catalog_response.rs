pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsFilesListCatalogResponse {
    #[serde(default)]
    pub rows: Vec<ItemsFilesListCatalogResponseRowsItem>,
}

impl ItemsFilesListCatalogResponse {
    pub fn builder() -> ItemsFilesListCatalogResponseBuilder {
        <ItemsFilesListCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsFilesListCatalogResponseBuilder {
    rows: Option<Vec<ItemsFilesListCatalogResponseRowsItem>>,
}

impl ItemsFilesListCatalogResponseBuilder {
    pub fn rows(mut self, value: Vec<ItemsFilesListCatalogResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemsFilesListCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ItemsFilesListCatalogResponseBuilder::rows)
    pub fn build(self) -> Result<ItemsFilesListCatalogResponse, BuildError> {
        Ok(ItemsFilesListCatalogResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
