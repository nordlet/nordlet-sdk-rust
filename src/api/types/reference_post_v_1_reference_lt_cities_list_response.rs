pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtCitiesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceLtCitiesListResponseRowsItem>,
}

impl PostV1ReferenceLtCitiesListResponse {
    pub fn builder() -> PostV1ReferenceLtCitiesListResponseBuilder {
        <PostV1ReferenceLtCitiesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtCitiesListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceLtCitiesListResponseRowsItem>>,
}

impl PostV1ReferenceLtCitiesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReferenceLtCitiesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceLtCitiesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceLtCitiesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceLtCitiesListResponse, BuildError> {
        Ok(PostV1ReferenceLtCitiesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
