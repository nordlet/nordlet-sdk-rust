pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AccountInvitesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1AccountInvitesListResponseRowsItem>,
}

impl PostV1AccountInvitesListResponse {
    pub fn builder() -> PostV1AccountInvitesListResponseBuilder {
        <PostV1AccountInvitesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AccountInvitesListResponseBuilder {
    rows: Option<Vec<PostV1AccountInvitesListResponseRowsItem>>,
}

impl PostV1AccountInvitesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1AccountInvitesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AccountInvitesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1AccountInvitesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1AccountInvitesListResponse, BuildError> {
        Ok(PostV1AccountInvitesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
