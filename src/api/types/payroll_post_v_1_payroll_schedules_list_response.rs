pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollSchedulesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1PayrollSchedulesListResponseRowsItem>,
}

impl PostV1PayrollSchedulesListResponse {
    pub fn builder() -> PostV1PayrollSchedulesListResponseBuilder {
        <PostV1PayrollSchedulesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollSchedulesListResponseBuilder {
    rows: Option<Vec<PostV1PayrollSchedulesListResponseRowsItem>>,
}

impl PostV1PayrollSchedulesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1PayrollSchedulesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollSchedulesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1PayrollSchedulesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1PayrollSchedulesListResponse, BuildError> {
        Ok(PostV1PayrollSchedulesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
