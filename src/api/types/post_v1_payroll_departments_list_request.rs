pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollDepartmentsListRequest {}

impl PostV1PayrollDepartmentsListRequest {
    pub fn builder() -> PostV1PayrollDepartmentsListRequestBuilder {
        <PostV1PayrollDepartmentsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollDepartmentsListRequestBuilder {}

impl PostV1PayrollDepartmentsListRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1PayrollDepartmentsListRequest`].
    pub fn build(self) -> Result<PostV1PayrollDepartmentsListRequest, BuildError> {
        Ok(PostV1PayrollDepartmentsListRequest {})
    }
}
