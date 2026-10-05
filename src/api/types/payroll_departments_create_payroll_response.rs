pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DepartmentsCreatePayrollResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl DepartmentsCreatePayrollResponse {
    pub fn builder() -> DepartmentsCreatePayrollResponseBuilder {
        <DepartmentsCreatePayrollResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepartmentsCreatePayrollResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl DepartmentsCreatePayrollResponseBuilder {
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

    /// Consumes the builder and constructs a [`DepartmentsCreatePayrollResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DepartmentsCreatePayrollResponseBuilder::id)
    /// - [`code`](DepartmentsCreatePayrollResponseBuilder::code)
    /// - [`name`](DepartmentsCreatePayrollResponseBuilder::name)
    pub fn build(self) -> Result<DepartmentsCreatePayrollResponse, BuildError> {
        Ok(DepartmentsCreatePayrollResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
