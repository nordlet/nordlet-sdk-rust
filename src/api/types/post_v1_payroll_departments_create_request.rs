pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollDepartmentsCreateRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1PayrollDepartmentsCreateRequest {
    pub fn builder() -> PostV1PayrollDepartmentsCreateRequestBuilder {
        <PostV1PayrollDepartmentsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollDepartmentsCreateRequestBuilder {
    code: Option<String>,
    name: Option<String>,
}

impl PostV1PayrollDepartmentsCreateRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollDepartmentsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PostV1PayrollDepartmentsCreateRequestBuilder::code)
    /// - [`name`](PostV1PayrollDepartmentsCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1PayrollDepartmentsCreateRequest, BuildError> {
        Ok(PostV1PayrollDepartmentsCreateRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
