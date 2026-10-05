pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsSuppliersDeleteCatalogResponse {
    #[serde(default)]
    pub id: String,
}

impl ItemsSuppliersDeleteCatalogResponse {
    pub fn builder() -> ItemsSuppliersDeleteCatalogResponseBuilder {
        <ItemsSuppliersDeleteCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsSuppliersDeleteCatalogResponseBuilder {
    id: Option<String>,
}

impl ItemsSuppliersDeleteCatalogResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsSuppliersDeleteCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsSuppliersDeleteCatalogResponseBuilder::id)
    pub fn build(self) -> Result<ItemsSuppliersDeleteCatalogResponse, BuildError> {
        Ok(ItemsSuppliersDeleteCatalogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
