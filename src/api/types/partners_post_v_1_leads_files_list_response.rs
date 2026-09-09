pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsFilesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1LeadsFilesListResponseRowsItem>,
}

impl PostV1LeadsFilesListResponse {
    pub fn builder() -> PostV1LeadsFilesListResponseBuilder {
        <PostV1LeadsFilesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsFilesListResponseBuilder {
    rows: Option<Vec<PostV1LeadsFilesListResponseRowsItem>>,
}

impl PostV1LeadsFilesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1LeadsFilesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsFilesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1LeadsFilesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1LeadsFilesListResponse, BuildError> {
        Ok(PostV1LeadsFilesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
