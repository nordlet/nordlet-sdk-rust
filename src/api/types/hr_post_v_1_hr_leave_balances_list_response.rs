pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrLeaveBalancesListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1HrLeaveBalancesListResponseRowsItem>,
}

impl PostV1HrLeaveBalancesListResponse {
    pub fn builder() -> PostV1HrLeaveBalancesListResponseBuilder {
        <PostV1HrLeaveBalancesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrLeaveBalancesListResponseBuilder {
    rows: Option<Vec<PostV1HrLeaveBalancesListResponseRowsItem>>,
}

impl PostV1HrLeaveBalancesListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1HrLeaveBalancesListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrLeaveBalancesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1HrLeaveBalancesListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1HrLeaveBalancesListResponse, BuildError> {
        Ok(PostV1HrLeaveBalancesListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
