pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1HrEmployeesDeleteRequest {
    pub fn builder() -> PostV1HrEmployeesDeleteRequestBuilder {
        <PostV1HrEmployeesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1HrEmployeesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1HrEmployeesDeleteRequest, BuildError> {
        Ok(PostV1HrEmployeesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
