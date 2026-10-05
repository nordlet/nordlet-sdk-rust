pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesGetHrRequest {
    #[serde(default)]
    pub id: String,
}

impl EmployeesGetHrRequest {
    pub fn builder() -> EmployeesGetHrRequestBuilder {
        <EmployeesGetHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesGetHrRequestBuilder {
    id: Option<String>,
}

impl EmployeesGetHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesGetHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmployeesGetHrRequestBuilder::id)
    pub fn build(self) -> Result<EmployeesGetHrRequest, BuildError> {
        Ok(EmployeesGetHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
