pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrEmployeesAnonymizeRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1HrEmployeesAnonymizeRequest {
    pub fn builder() -> PostV1HrEmployeesAnonymizeRequestBuilder {
        <PostV1HrEmployeesAnonymizeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrEmployeesAnonymizeRequestBuilder {
    id: Option<String>,
}

impl PostV1HrEmployeesAnonymizeRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrEmployeesAnonymizeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrEmployeesAnonymizeRequestBuilder::id)
    pub fn build(self) -> Result<PostV1HrEmployeesAnonymizeRequest, BuildError> {
        Ok(PostV1HrEmployeesAnonymizeRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
