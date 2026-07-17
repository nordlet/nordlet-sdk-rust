pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollDepartmentsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1PayrollDepartmentsListResponseRowsItem>,
}

impl PostV1PayrollDepartmentsListResponse {
    pub fn builder() -> PostV1PayrollDepartmentsListResponseBuilder {
        <PostV1PayrollDepartmentsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollDepartmentsListResponseBuilder {
    rows: Option<Vec<PostV1PayrollDepartmentsListResponseRowsItem>>,
}

impl PostV1PayrollDepartmentsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1PayrollDepartmentsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollDepartmentsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1PayrollDepartmentsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1PayrollDepartmentsListResponse, BuildError> {
        Ok(PostV1PayrollDepartmentsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
