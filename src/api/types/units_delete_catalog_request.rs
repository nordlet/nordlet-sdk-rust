pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsDeleteCatalogRequest {
    #[serde(default)]
    pub id: String,
}

impl UnitsDeleteCatalogRequest {
    pub fn builder() -> UnitsDeleteCatalogRequestBuilder {
        <UnitsDeleteCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsDeleteCatalogRequestBuilder {
    id: Option<String>,
}

impl UnitsDeleteCatalogRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitsDeleteCatalogRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UnitsDeleteCatalogRequestBuilder::id)
    pub fn build(self) -> Result<UnitsDeleteCatalogRequest, BuildError> {
        Ok(UnitsDeleteCatalogRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
