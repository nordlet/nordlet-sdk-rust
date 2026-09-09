pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtMunicipalitiesListRequest {
    #[serde(rename = "countyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county_code: Option<String>,
}

impl PostV1ReferenceLtMunicipalitiesListRequest {
    pub fn builder() -> PostV1ReferenceLtMunicipalitiesListRequestBuilder {
        <PostV1ReferenceLtMunicipalitiesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtMunicipalitiesListRequestBuilder {
    county_code: Option<String>,
}

impl PostV1ReferenceLtMunicipalitiesListRequestBuilder {
    pub fn county_code(mut self, value: impl Into<String>) -> Self {
        self.county_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceLtMunicipalitiesListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceLtMunicipalitiesListRequest, BuildError> {
        Ok(PostV1ReferenceLtMunicipalitiesListRequest {
            county_code: self.county_code,
        })
    }
}
