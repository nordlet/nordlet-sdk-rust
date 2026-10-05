pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsOptionsCatalogRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<UnitsOptionsCatalogRequestLocale>,
}

impl UnitsOptionsCatalogRequest {
    pub fn builder() -> UnitsOptionsCatalogRequestBuilder {
        <UnitsOptionsCatalogRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsOptionsCatalogRequestBuilder {
    locale: Option<UnitsOptionsCatalogRequestLocale>,
}

impl UnitsOptionsCatalogRequestBuilder {
    pub fn locale(mut self, value: UnitsOptionsCatalogRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitsOptionsCatalogRequest`].
    pub fn build(self) -> Result<UnitsOptionsCatalogRequest, BuildError> {
        Ok(UnitsOptionsCatalogRequest {
            locale: self.locale,
        })
    }
}
