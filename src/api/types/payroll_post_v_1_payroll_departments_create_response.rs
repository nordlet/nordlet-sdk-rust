pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollDepartmentsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1PayrollDepartmentsCreateResponse {
    pub fn builder() -> PostV1PayrollDepartmentsCreateResponseBuilder {
        <PostV1PayrollDepartmentsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollDepartmentsCreateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl PostV1PayrollDepartmentsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollDepartmentsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollDepartmentsCreateResponseBuilder::id)
    /// - [`code`](PostV1PayrollDepartmentsCreateResponseBuilder::code)
    /// - [`name`](PostV1PayrollDepartmentsCreateResponseBuilder::name)
    pub fn build(self) -> Result<PostV1PayrollDepartmentsCreateResponse, BuildError> {
        Ok(PostV1PayrollDepartmentsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
