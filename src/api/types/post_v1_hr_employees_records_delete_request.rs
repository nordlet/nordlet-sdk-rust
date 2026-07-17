pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesRecordsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1HrEmployeesRecordsDeleteRequest {
    pub fn builder() -> PostV1HrEmployeesRecordsDeleteRequestBuilder {
        <PostV1HrEmployeesRecordsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesRecordsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1HrEmployeesRecordsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesRecordsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesRecordsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1HrEmployeesRecordsDeleteRequest, BuildError> {
        Ok(PostV1HrEmployeesRecordsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
