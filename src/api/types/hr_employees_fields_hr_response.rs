pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmployeesFieldsHrResponse {
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub fields: Vec<EmployeesFieldsHrResponseFieldsItem>,
}

impl EmployeesFieldsHrResponse {
    pub fn builder() -> EmployeesFieldsHrResponseBuilder {
        <EmployeesFieldsHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmployeesFieldsHrResponseBuilder {
    country: Option<String>,
    fields: Option<Vec<EmployeesFieldsHrResponseFieldsItem>>,
}

impl EmployeesFieldsHrResponseBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<EmployeesFieldsHrResponseFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmployeesFieldsHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](EmployeesFieldsHrResponseBuilder::country)
    /// - [`fields`](EmployeesFieldsHrResponseBuilder::fields)
    pub fn build(self) -> Result<EmployeesFieldsHrResponse, BuildError> {
        Ok(EmployeesFieldsHrResponse {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
