pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesDeleteHrRequest {
    #[serde(default)]
    pub id: String,
}

impl EmployeesDeleteHrRequest {
    pub fn builder() -> EmployeesDeleteHrRequestBuilder {
        <EmployeesDeleteHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesDeleteHrRequestBuilder {
    id: Option<String>,
}

impl EmployeesDeleteHrRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmployeesDeleteHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EmployeesDeleteHrRequestBuilder::id)
    pub fn build(self) -> Result<EmployeesDeleteHrRequest, BuildError> {
        Ok(EmployeesDeleteHrRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
