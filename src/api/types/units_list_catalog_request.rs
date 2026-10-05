pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsListCatalogRequest {}

impl UnitsListCatalogRequest {
    pub fn builder() -> UnitsListCatalogRequestBuilder {
        <UnitsListCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsListCatalogRequestBuilder {}

impl UnitsListCatalogRequestBuilder {
    /// Consumes the builder and constructs a [`UnitsListCatalogRequest`].
    pub fn build(self) -> Result<UnitsListCatalogRequest, BuildError> {
        Ok(UnitsListCatalogRequest {})
    }
}
