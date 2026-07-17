pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersStatusesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1PartnersStatusesListResponseRowsItem>,
}

impl PostV1PartnersStatusesListResponse {
    pub fn builder() -> PostV1PartnersStatusesListResponseBuilder {
        <PostV1PartnersStatusesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersStatusesListResponseBuilder {
    rows: Option<Vec<PostV1PartnersStatusesListResponseRowsItem>>,
}

impl PostV1PartnersStatusesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1PartnersStatusesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersStatusesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1PartnersStatusesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1PartnersStatusesListResponse, BuildError> {
        Ok(PostV1PartnersStatusesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
