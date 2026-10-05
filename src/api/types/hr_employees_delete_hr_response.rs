pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesDeleteHrResponse {
    #[serde(default)]
    pub id: String,
}

impl EmployeesDeleteHrResponse {
    pub fn builder() -> EmployeesDeleteHrResponseBuilder {
        <EmployeesDeleteHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesDeleteHrResponseBuilder {
    id: Option<String>,
}

impl EmployeesDeleteHrResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesDeleteHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmployeesDeleteHrResponseBuilder::id)
    pub fn build(self) -> Result<EmployeesDeleteHrResponse, BuildError> {
        Ok(EmployeesDeleteHrResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
