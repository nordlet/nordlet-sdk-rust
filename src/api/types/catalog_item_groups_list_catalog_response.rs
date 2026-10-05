pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemGroupsListCatalogResponse {
    #[serde(default)]
    pub rows: Vec<ItemGroupsListCatalogResponseRowsItem>,
}

impl ItemGroupsListCatalogResponse {
    pub fn builder() -> ItemGroupsListCatalogResponseBuilder {
        <ItemGroupsListCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemGroupsListCatalogResponseBuilder {
    rows: Option<Vec<ItemGroupsListCatalogResponseRowsItem>>,
}

impl ItemGroupsListCatalogResponseBuilder {
    pub fn rows(mut self, value: Vec<ItemGroupsListCatalogResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ItemGroupsListCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](ItemGroupsListCatalogResponseBuilder::rows)
    pub fn build(self) -> Result<ItemGroupsListCatalogResponse, BuildError> {
        Ok(ItemGroupsListCatalogResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
