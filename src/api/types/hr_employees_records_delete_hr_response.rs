pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesRecordsDeleteHrResponse {
    #[serde(default)]
    pub id: String,
}

impl EmployeesRecordsDeleteHrResponse {
    pub fn builder() -> EmployeesRecordsDeleteHrResponseBuilder {
        <EmployeesRecordsDeleteHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesRecordsDeleteHrResponseBuilder {
    id: Option<String>,
}

impl EmployeesRecordsDeleteHrResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesRecordsDeleteHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmployeesRecordsDeleteHrResponseBuilder::id)
    pub fn build(self) -> Result<EmployeesRecordsDeleteHrResponse, BuildError> {
        Ok(EmployeesRecordsDeleteHrResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
