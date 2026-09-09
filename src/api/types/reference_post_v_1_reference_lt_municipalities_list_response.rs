pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtMunicipalitiesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceLtMunicipalitiesListResponseRowsItem>,
}

impl PostV1ReferenceLtMunicipalitiesListResponse {
    pub fn builder() -> PostV1ReferenceLtMunicipalitiesListResponseBuilder {
        <PostV1ReferenceLtMunicipalitiesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtMunicipalitiesListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceLtMunicipalitiesListResponseRowsItem>>,
}

impl PostV1ReferenceLtMunicipalitiesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReferenceLtMunicipalitiesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceLtMunicipalitiesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceLtMunicipalitiesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceLtMunicipalitiesListResponse, BuildError> {
        Ok(PostV1ReferenceLtMunicipalitiesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
