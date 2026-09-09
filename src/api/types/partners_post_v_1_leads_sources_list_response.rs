pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsSourcesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1LeadsSourcesListResponseRowsItem>,
}

impl PostV1LeadsSourcesListResponse {
    pub fn builder() -> PostV1LeadsSourcesListResponseBuilder {
        <PostV1LeadsSourcesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsSourcesListResponseBuilder {
    rows: Option<Vec<PostV1LeadsSourcesListResponseRowsItem>>,
}

impl PostV1LeadsSourcesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1LeadsSourcesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsSourcesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1LeadsSourcesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1LeadsSourcesListResponse, BuildError> {
        Ok(PostV1LeadsSourcesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
