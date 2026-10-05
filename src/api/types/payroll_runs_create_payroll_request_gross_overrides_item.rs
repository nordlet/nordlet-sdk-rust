pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsCreatePayrollRequestGrossOverridesItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub gross: String,
}

impl RunsCreatePayrollRequestGrossOverridesItem {
    pub fn builder() -> RunsCreatePayrollRequestGrossOverridesItemBuilder {
        <RunsCreatePayrollRequestGrossOverridesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsCreatePayrollRequestGrossOverridesItemBuilder {
    employee_id: Option<String>,
    gross: Option<String>,
}

impl RunsCreatePayrollRequestGrossOverridesItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsCreatePayrollRequestGrossOverridesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](RunsCreatePayrollRequestGrossOverridesItemBuilder::employee_id)
    /// - [`gross`](RunsCreatePayrollRequestGrossOverridesItemBuilder::gross)
    pub fn build(self) -> Result<RunsCreatePayrollRequestGrossOverridesItem, BuildError> {
        Ok(RunsCreatePayrollRequestGrossOverridesItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
        })
    }
}
