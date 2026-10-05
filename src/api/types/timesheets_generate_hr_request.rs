pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimesheetsGenerateHrRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
}

impl TimesheetsGenerateHrRequest {
    pub fn builder() -> TimesheetsGenerateHrRequestBuilder {
        <TimesheetsGenerateHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsGenerateHrRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    employee_id: Option<String>,
}

impl TimesheetsGenerateHrRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsGenerateHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](TimesheetsGenerateHrRequestBuilder::year)
    /// - [`month`](TimesheetsGenerateHrRequestBuilder::month)
    pub fn build(self) -> Result<TimesheetsGenerateHrRequest, BuildError> {
        Ok(TimesheetsGenerateHrRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            employee_id: self.employee_id,
        })
    }
}
