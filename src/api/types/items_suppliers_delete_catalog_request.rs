pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsSuppliersDeleteCatalogRequest {
    #[serde(default)]
    pub id: String,
}

impl ItemsSuppliersDeleteCatalogRequest {
    pub fn builder() -> ItemsSuppliersDeleteCatalogRequestBuilder {
        <ItemsSuppliersDeleteCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsSuppliersDeleteCatalogRequestBuilder {
    id: Option<String>,
}

impl ItemsSuppliersDeleteCatalogRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsSuppliersDeleteCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsSuppliersDeleteCatalogRequestBuilder::id)
    pub fn build(self) -> Result<ItemsSuppliersDeleteCatalogRequest, BuildError> {
        Ok(ItemsSuppliersDeleteCatalogRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
