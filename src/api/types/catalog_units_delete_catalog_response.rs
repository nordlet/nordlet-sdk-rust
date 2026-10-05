pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsDeleteCatalogResponse {
    #[serde(default)]
    pub id: String,
}

impl UnitsDeleteCatalogResponse {
    pub fn builder() -> UnitsDeleteCatalogResponseBuilder {
        <UnitsDeleteCatalogResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsDeleteCatalogResponseBuilder {
    id: Option<String>,
}

impl UnitsDeleteCatalogResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitsDeleteCatalogResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UnitsDeleteCatalogResponseBuilder::id)
    pub fn build(self) -> Result<UnitsDeleteCatalogResponse, BuildError> {
        Ok(UnitsDeleteCatalogResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
