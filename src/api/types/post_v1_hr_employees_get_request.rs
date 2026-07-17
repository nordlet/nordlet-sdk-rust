pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1HrEmployeesGetRequest {
    pub fn builder() -> PostV1HrEmployeesGetRequestBuilder {
        <PostV1HrEmployeesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1HrEmployeesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1HrEmployeesGetRequest, BuildError> {
        Ok(PostV1HrEmployeesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
