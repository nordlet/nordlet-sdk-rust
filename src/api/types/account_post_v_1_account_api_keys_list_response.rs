pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountApiKeysListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1AccountApiKeysListResponseRowsItem>,
}

impl PostV1AccountApiKeysListResponse {
    pub fn builder() -> PostV1AccountApiKeysListResponseBuilder {
        <PostV1AccountApiKeysListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountApiKeysListResponseBuilder {
    rows: Option<Vec<PostV1AccountApiKeysListResponseRowsItem>>,
}

impl PostV1AccountApiKeysListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1AccountApiKeysListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountApiKeysListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1AccountApiKeysListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1AccountApiKeysListResponse, BuildError> {
        Ok(PostV1AccountApiKeysListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
