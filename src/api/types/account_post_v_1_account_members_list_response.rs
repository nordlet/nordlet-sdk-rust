pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountMembersListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1AccountMembersListResponseRowsItem>,
}

impl PostV1AccountMembersListResponse {
    pub fn builder() -> PostV1AccountMembersListResponseBuilder {
        <PostV1AccountMembersListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountMembersListResponseBuilder {
    rows: Option<Vec<PostV1AccountMembersListResponseRowsItem>>,
}

impl PostV1AccountMembersListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1AccountMembersListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountMembersListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1AccountMembersListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1AccountMembersListResponse, BuildError> {
        Ok(PostV1AccountMembersListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
