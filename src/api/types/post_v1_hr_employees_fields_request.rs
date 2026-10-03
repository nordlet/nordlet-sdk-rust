pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesFieldsRequest {}

impl PostV1HrEmployeesFieldsRequest {
    pub fn builder() -> PostV1HrEmployeesFieldsRequestBuilder {
        <PostV1HrEmployeesFieldsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesFieldsRequestBuilder {}

impl PostV1HrEmployeesFieldsRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1HrEmployeesFieldsRequest`].
    pub fn build(self) -> Result<PostV1HrEmployeesFieldsRequest, BuildError> {
        Ok(PostV1HrEmployeesFieldsRequest {})
    }
}
