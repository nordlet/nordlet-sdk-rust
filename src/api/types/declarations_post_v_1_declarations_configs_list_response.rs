pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1DeclarationsConfigsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsConfigsListResponseRowsItem>,
}

impl PostV1DeclarationsConfigsListResponse {
    pub fn builder() -> PostV1DeclarationsConfigsListResponseBuilder {
        <PostV1DeclarationsConfigsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsConfigsListResponseBuilder {
    rows: Option<Vec<PostV1DeclarationsConfigsListResponseRowsItem>>,
}

impl PostV1DeclarationsConfigsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1DeclarationsConfigsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsConfigsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1DeclarationsConfigsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsConfigsListResponse, BuildError> {
        Ok(PostV1DeclarationsConfigsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
