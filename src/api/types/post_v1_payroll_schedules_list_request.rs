pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollSchedulesListRequest {}

impl PostV1PayrollSchedulesListRequest {
    pub fn builder() -> PostV1PayrollSchedulesListRequestBuilder {
        <PostV1PayrollSchedulesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollSchedulesListRequestBuilder {}

impl PostV1PayrollSchedulesListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1PayrollSchedulesListRequest`].
    pub fn build(self) -> Result<PostV1PayrollSchedulesListRequest, BuildError> {
        Ok(PostV1PayrollSchedulesListRequest {})
    }
}
