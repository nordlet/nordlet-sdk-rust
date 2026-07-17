pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesRecordsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1HrEmployeesRecordsDeleteResponse {
    pub fn builder() -> PostV1HrEmployeesRecordsDeleteResponseBuilder {
        <PostV1HrEmployeesRecordsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesRecordsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1HrEmployeesRecordsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesRecordsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesRecordsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1HrEmployeesRecordsDeleteResponse, BuildError> {
        Ok(PostV1HrEmployeesRecordsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
