pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersGroupsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1PartnersGroupsListResponseRowsItem>,
}

impl PostV1PartnersGroupsListResponse {
    pub fn builder() -> PostV1PartnersGroupsListResponseBuilder {
        <PostV1PartnersGroupsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersGroupsListResponseBuilder {
    rows: Option<Vec<PostV1PartnersGroupsListResponseRowsItem>>,
}

impl PostV1PartnersGroupsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1PartnersGroupsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersGroupsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1PartnersGroupsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1PartnersGroupsListResponse, BuildError> {
        Ok(PostV1PartnersGroupsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
