pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyCandidatesResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ConsolidationIntercompanyCandidatesResponseRowsItem>,
}

impl PostV1ConsolidationIntercompanyCandidatesResponse {
    pub fn builder() -> PostV1ConsolidationIntercompanyCandidatesResponseBuilder {
        <PostV1ConsolidationIntercompanyCandidatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyCandidatesResponseBuilder {
    rows: Option<Vec<PostV1ConsolidationIntercompanyCandidatesResponseRowsItem>>,
}

impl PostV1ConsolidationIntercompanyCandidatesResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1ConsolidationIntercompanyCandidatesResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyCandidatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ConsolidationIntercompanyCandidatesResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ConsolidationIntercompanyCandidatesResponse, BuildError> {
        Ok(PostV1ConsolidationIntercompanyCandidatesResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
