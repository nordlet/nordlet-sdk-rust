pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUnitsDeleteCatalogResponse {
    #[serde(default)]
    pub id: String,
}

impl ItemsUnitsDeleteCatalogResponse {
    pub fn builder() -> ItemsUnitsDeleteCatalogResponseBuilder {
        <ItemsUnitsDeleteCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUnitsDeleteCatalogResponseBuilder {
    id: Option<String>,
}

impl ItemsUnitsDeleteCatalogResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsUnitsDeleteCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsUnitsDeleteCatalogResponseBuilder::id)
    pub fn build(self) -> Result<ItemsUnitsDeleteCatalogResponse, BuildError> {
        Ok(ItemsUnitsDeleteCatalogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
