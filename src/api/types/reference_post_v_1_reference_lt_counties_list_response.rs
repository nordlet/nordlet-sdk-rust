pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtCountiesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceLtCountiesListResponseRowsItem>,
}

impl PostV1ReferenceLtCountiesListResponse {
    pub fn builder() -> PostV1ReferenceLtCountiesListResponseBuilder {
        <PostV1ReferenceLtCountiesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtCountiesListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceLtCountiesListResponseRowsItem>>,
}

impl PostV1ReferenceLtCountiesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReferenceLtCountiesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceLtCountiesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceLtCountiesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceLtCountiesListResponse, BuildError> {
        Ok(PostV1ReferenceLtCountiesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
