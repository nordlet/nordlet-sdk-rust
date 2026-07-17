pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceLtRegionsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceLtRegionsListResponseRowsItem>,
}

impl PostV1ReferenceLtRegionsListResponse {
    pub fn builder() -> PostV1ReferenceLtRegionsListResponseBuilder {
        <PostV1ReferenceLtRegionsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceLtRegionsListResponseBuilder {
    rows: Option<Vec<PostV1ReferenceLtRegionsListResponseRowsItem>>,
}

impl PostV1ReferenceLtRegionsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReferenceLtRegionsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceLtRegionsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceLtRegionsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceLtRegionsListResponse, BuildError> {
        Ok(PostV1ReferenceLtRegionsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
