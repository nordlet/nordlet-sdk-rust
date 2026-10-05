pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CountriesListReferenceRequest {}

impl CountriesListReferenceRequest {
    pub fn builder() -> CountriesListReferenceRequestBuilder {
        <CountriesListReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CountriesListReferenceRequestBuilder {}

impl CountriesListReferenceRequestBuilder {
    /// Consumes the builder and constructs a [`CountriesListReferenceRequest`].
    pub fn build(self) -> Result<CountriesListReferenceRequest, BuildError> {
        Ok(CountriesListReferenceRequest {})
    }
}
