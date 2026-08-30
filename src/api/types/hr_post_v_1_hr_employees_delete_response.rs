pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1HrEmployeesDeleteResponse {
    pub fn builder() -> PostV1HrEmployeesDeleteResponseBuilder {
        <PostV1HrEmployeesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1HrEmployeesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1HrEmployeesDeleteResponse, BuildError> {
        Ok(PostV1HrEmployeesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
