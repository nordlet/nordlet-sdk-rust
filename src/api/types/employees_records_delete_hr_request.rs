pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesRecordsDeleteHrRequest {
    #[serde(default)]
    pub id: String,
}

impl EmployeesRecordsDeleteHrRequest {
    pub fn builder() -> EmployeesRecordsDeleteHrRequestBuilder {
        <EmployeesRecordsDeleteHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesRecordsDeleteHrRequestBuilder {
    id: Option<String>,
}

impl EmployeesRecordsDeleteHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesRecordsDeleteHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmployeesRecordsDeleteHrRequestBuilder::id)
    pub fn build(self) -> Result<EmployeesRecordsDeleteHrRequest, BuildError> {
        Ok(EmployeesRecordsDeleteHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
