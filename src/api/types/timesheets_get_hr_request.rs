pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimesheetsGetHrRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl TimesheetsGetHrRequest {
    pub fn builder() -> TimesheetsGetHrRequestBuilder {
        <TimesheetsGetHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsGetHrRequestBuilder {
    employee_id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
}

impl TimesheetsGetHrRequestBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsGetHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](TimesheetsGetHrRequestBuilder::employee_id)
    /// - [`year`](TimesheetsGetHrRequestBuilder::year)
    /// - [`month`](TimesheetsGetHrRequestBuilder::month)
    pub fn build(self) -> Result<TimesheetsGetHrRequest, BuildError> {
        Ok(TimesheetsGetHrRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
