pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesAttachmentsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1HrEmployeesAttachmentsListResponseRowsItem>,
}

impl PostV1HrEmployeesAttachmentsListResponse {
    pub fn builder() -> PostV1HrEmployeesAttachmentsListResponseBuilder {
        <PostV1HrEmployeesAttachmentsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesAttachmentsListResponseBuilder {
    rows: Option<Vec<PostV1HrEmployeesAttachmentsListResponseRowsItem>>,
}

impl PostV1HrEmployeesAttachmentsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1HrEmployeesAttachmentsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesAttachmentsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1HrEmployeesAttachmentsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1HrEmployeesAttachmentsListResponse, BuildError> {
        Ok(PostV1HrEmployeesAttachmentsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
