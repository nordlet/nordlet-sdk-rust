pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountSessionsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1AccountSessionsListResponseRowsItem>,
}

impl PostV1AccountSessionsListResponse {
    pub fn builder() -> PostV1AccountSessionsListResponseBuilder {
        <PostV1AccountSessionsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountSessionsListResponseBuilder {
    rows: Option<Vec<PostV1AccountSessionsListResponseRowsItem>>,
}

impl PostV1AccountSessionsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1AccountSessionsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountSessionsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1AccountSessionsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1AccountSessionsListResponse, BuildError> {
        Ok(PostV1AccountSessionsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
