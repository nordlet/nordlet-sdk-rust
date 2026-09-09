pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtCitiesListRequest {
    #[serde(rename = "municipalityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

impl PostV1ReferenceLtCitiesListRequest {
    pub fn builder() -> PostV1ReferenceLtCitiesListRequestBuilder {
        <PostV1ReferenceLtCitiesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtCitiesListRequestBuilder {
    municipality_code: Option<String>,
    q: Option<String>,
}

impl PostV1ReferenceLtCitiesListRequestBuilder {
    pub fn municipality_code(mut self, value: impl Into<String>) -> Self {
        self.municipality_code = Some(value.into());
        self
    }

    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceLtCitiesListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceLtCitiesListRequest, BuildError> {
        Ok(PostV1ReferenceLtCitiesListRequest {
            municipality_code: self.municipality_code,
            q: self.q,
        })
    }
}
