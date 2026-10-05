pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ItemsGetCatalogRequest {
    #[serde(default)]
    pub id: String,
}

impl ItemsGetCatalogRequest {
    pub fn builder() -> ItemsGetCatalogRequestBuilder {
        <ItemsGetCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ItemsGetCatalogRequestBuilder {
    id: Option<String>,
}

impl ItemsGetCatalogRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ItemsGetCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ItemsGetCatalogRequestBuilder::id)
    pub fn build(self) -> Result<ItemsGetCatalogRequest, BuildError> {
        Ok(ItemsGetCatalogRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
