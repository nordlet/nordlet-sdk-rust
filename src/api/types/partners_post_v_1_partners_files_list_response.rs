pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersFilesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1PartnersFilesListResponseRowsItem>,
}

impl PostV1PartnersFilesListResponse {
    pub fn builder() -> PostV1PartnersFilesListResponseBuilder {
        <PostV1PartnersFilesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersFilesListResponseBuilder {
    rows: Option<Vec<PostV1PartnersFilesListResponseRowsItem>>,
}

impl PostV1PartnersFilesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1PartnersFilesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersFilesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1PartnersFilesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1PartnersFilesListResponse, BuildError> {
        Ok(PostV1PartnersFilesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
