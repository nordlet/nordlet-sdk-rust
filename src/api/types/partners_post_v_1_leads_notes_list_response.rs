pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsNotesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1LeadsNotesListResponseRowsItem>,
}

impl PostV1LeadsNotesListResponse {
    pub fn builder() -> PostV1LeadsNotesListResponseBuilder {
        <PostV1LeadsNotesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsNotesListResponseBuilder {
    rows: Option<Vec<PostV1LeadsNotesListResponseRowsItem>>,
}

impl PostV1LeadsNotesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1LeadsNotesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsNotesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1LeadsNotesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1LeadsNotesListResponse, BuildError> {
        Ok(PostV1LeadsNotesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
