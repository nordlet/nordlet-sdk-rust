pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DepartmentsCreatePayrollRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl DepartmentsCreatePayrollRequest {
    pub fn builder() -> DepartmentsCreatePayrollRequestBuilder {
        <DepartmentsCreatePayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepartmentsCreatePayrollRequestBuilder {
    code: Option<String>,
    name: Option<String>,
}

impl DepartmentsCreatePayrollRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DepartmentsCreatePayrollRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](DepartmentsCreatePayrollRequestBuilder::code)
    /// - [`name`](DepartmentsCreatePayrollRequestBuilder::name)
    pub fn build(self) -> Result<DepartmentsCreatePayrollRequest, BuildError> {
        Ok(DepartmentsCreatePayrollRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
