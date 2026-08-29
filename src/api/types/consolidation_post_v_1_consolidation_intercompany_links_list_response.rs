pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyLinksListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1ConsolidationIntercompanyLinksListResponseRowsItem>,
}

impl PostV1ConsolidationIntercompanyLinksListResponse {
    pub fn builder() -> PostV1ConsolidationIntercompanyLinksListResponseBuilder {
        <PostV1ConsolidationIntercompanyLinksListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyLinksListResponseBuilder {
    rows: Option<Vec<PostV1ConsolidationIntercompanyLinksListResponseRowsItem>>,
}

impl PostV1ConsolidationIntercompanyLinksListResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1ConsolidationIntercompanyLinksListResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyLinksListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ConsolidationIntercompanyLinksListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1ConsolidationIntercompanyLinksListResponse, BuildError> {
        Ok(PostV1ConsolidationIntercompanyLinksListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
