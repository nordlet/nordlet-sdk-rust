pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SchedulesCreatePayrollRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "hoursPerWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_per_week: Option<String>,
}

impl SchedulesCreatePayrollRequest {
    pub fn builder() -> SchedulesCreatePayrollRequestBuilder {
        <SchedulesCreatePayrollRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SchedulesCreatePayrollRequestBuilder {
    code: Option<String>,
    name: Option<String>,
    hours_per_week: Option<String>,
}

impl SchedulesCreatePayrollRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn hours_per_week(mut self, value: impl Into<String>) -> Self {
        self.hours_per_week = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SchedulesCreatePayrollRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](SchedulesCreatePayrollRequestBuilder::code)
    /// - [`name`](SchedulesCreatePayrollRequestBuilder::name)
    pub fn build(self) -> Result<SchedulesCreatePayrollRequest, BuildError> {
        Ok(SchedulesCreatePayrollRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            hours_per_week: self.hours_per_week,
        })
    }
}
