pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtMunicipalitiesListReferenceRequest {
    #[serde(rename = "countyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county_code: Option<String>,
}

impl LtMunicipalitiesListReferenceRequest {
    pub fn builder() -> LtMunicipalitiesListReferenceRequestBuilder {
        <LtMunicipalitiesListReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtMunicipalitiesListReferenceRequestBuilder {
    county_code: Option<String>,
}

impl LtMunicipalitiesListReferenceRequestBuilder {
    pub fn county_code(mut self, value: impl Into<String>) -> Self {
        self.county_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtMunicipalitiesListReferenceRequest`].
    pub fn build(self) -> Result<LtMunicipalitiesListReferenceRequest, BuildError> {
        Ok(LtMunicipalitiesListReferenceRequest {
            county_code: self.county_code,
        })
    }
}
