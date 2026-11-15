pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompaniesContextAccountResponsePayroll {
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub engine: bool,
    #[serde(rename = "employeeFields")]
    #[serde(default)]
    pub employee_fields: Vec<CompaniesContextAccountResponsePayrollEmployeeFieldsItem>,
}

impl CompaniesContextAccountResponsePayroll {
    pub fn builder() -> CompaniesContextAccountResponsePayrollBuilder {
        <CompaniesContextAccountResponsePayrollBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompaniesContextAccountResponsePayrollBuilder {
    country: Option<String>,
    engine: Option<bool>,
    employee_fields: Option<Vec<CompaniesContextAccountResponsePayrollEmployeeFieldsItem>>,
}

impl CompaniesContextAccountResponsePayrollBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn engine(mut self, value: bool) -> Self {
        self.engine = Some(value);
        self
    }

    pub fn employee_fields(
        mut self,
        value: Vec<CompaniesContextAccountResponsePayrollEmployeeFieldsItem>,
    ) -> Self {
        self.employee_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompaniesContextAccountResponsePayroll`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](CompaniesContextAccountResponsePayrollBuilder::country)
    /// - [`engine`](CompaniesContextAccountResponsePayrollBuilder::engine)
    /// - [`employee_fields`](CompaniesContextAccountResponsePayrollBuilder::employee_fields)
    pub fn build(self) -> Result<CompaniesContextAccountResponsePayroll, BuildError> {
        Ok(CompaniesContextAccountResponsePayroll {
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            engine: self
                .engine
                .ok_or_else(|| BuildError::missing_field("engine"))?,
            employee_fields: self
                .employee_fields
                .ok_or_else(|| BuildError::missing_field("employee_fields"))?,
        })
    }
}
