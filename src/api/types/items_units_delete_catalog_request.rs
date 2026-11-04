pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsUnitsDeleteCatalogRequest {
    #[serde(default)]
    pub id: String,
}

impl ItemsUnitsDeleteCatalogRequest {
    pub fn builder() -> ItemsUnitsDeleteCatalogRequestBuilder {
        <ItemsUnitsDeleteCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsUnitsDeleteCatalogRequestBuilder {
    id: Option<String>,
}

impl ItemsUnitsDeleteCatalogRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsUnitsDeleteCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsUnitsDeleteCatalogRequestBuilder::id)
    pub fn build(self) -> Result<ItemsUnitsDeleteCatalogRequest, BuildError> {
        Ok(ItemsUnitsDeleteCatalogRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
