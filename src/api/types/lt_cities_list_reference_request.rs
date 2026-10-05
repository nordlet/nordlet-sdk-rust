pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtCitiesListReferenceRequest {
    #[serde(rename = "municipalityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl LtCitiesListReferenceRequest {
    pub fn builder() -> LtCitiesListReferenceRequestBuilder {
        <LtCitiesListReferenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtCitiesListReferenceRequestBuilder {
    municipality_code: Option<String>,
    q: Option<String>,
}

impl LtCitiesListReferenceRequestBuilder {
    pub fn municipality_code(mut self, value: impl Into<String>) -> Self {
        self.municipality_code = Some(value.into());
        self
    }

    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtCitiesListReferenceRequest`].
    pub fn build(self) -> Result<LtCitiesListReferenceRequest, BuildError> {
        Ok(LtCitiesListReferenceRequest {
            municipality_code: self.municipality_code,
            q: self.q,
        })
    }
}
