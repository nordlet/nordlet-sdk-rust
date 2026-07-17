pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationGroupsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ConsolidationGroupsListResponseRowsItem>,
}

impl PostV1ConsolidationGroupsListResponse {
    pub fn builder() -> PostV1ConsolidationGroupsListResponseBuilder {
        <PostV1ConsolidationGroupsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationGroupsListResponseBuilder {
    rows: Option<Vec<PostV1ConsolidationGroupsListResponseRowsItem>>,
}

impl PostV1ConsolidationGroupsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1ConsolidationGroupsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationGroupsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ConsolidationGroupsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ConsolidationGroupsListResponse, BuildError> {
        Ok(PostV1ConsolidationGroupsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
