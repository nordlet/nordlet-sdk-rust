pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesAnonymizeHrRequest {
    #[serde(default)]
    pub id: String,
}

impl EmployeesAnonymizeHrRequest {
    pub fn builder() -> EmployeesAnonymizeHrRequestBuilder {
        <EmployeesAnonymizeHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesAnonymizeHrRequestBuilder {
    id: Option<String>,
}

impl EmployeesAnonymizeHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesAnonymizeHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmployeesAnonymizeHrRequestBuilder::id)
    pub fn build(self) -> Result<EmployeesAnonymizeHrRequest, BuildError> {
        Ok(EmployeesAnonymizeHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
