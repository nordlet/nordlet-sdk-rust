pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1OfficersListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1OfficersListResponseRowsItem>,
}

impl PostV1OfficersListResponse {
    pub fn builder() -> PostV1OfficersListResponseBuilder {
        <PostV1OfficersListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1OfficersListResponseBuilder {
    rows: Option<Vec<PostV1OfficersListResponseRowsItem>>,
}

impl PostV1OfficersListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1OfficersListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1OfficersListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1OfficersListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1OfficersListResponse, BuildError> {
        Ok(PostV1OfficersListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
