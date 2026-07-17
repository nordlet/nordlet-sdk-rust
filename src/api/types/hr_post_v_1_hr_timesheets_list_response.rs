pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrTimesheetsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1HrTimesheetsListResponseRowsItem>,
}

impl PostV1HrTimesheetsListResponse {
    pub fn builder() -> PostV1HrTimesheetsListResponseBuilder {
        <PostV1HrTimesheetsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrTimesheetsListResponseBuilder {
    rows: Option<Vec<PostV1HrTimesheetsListResponseRowsItem>>,
}

impl PostV1HrTimesheetsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1HrTimesheetsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrTimesheetsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1HrTimesheetsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1HrTimesheetsListResponse, BuildError> {
        Ok(PostV1HrTimesheetsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
