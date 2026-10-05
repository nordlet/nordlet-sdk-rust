pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimesheetsUpsertHrRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub days: Vec<TimesheetsUpsertHrRequestDaysItem>,
}

impl TimesheetsUpsertHrRequest {
    pub fn builder() -> TimesheetsUpsertHrRequestBuilder {
        <TimesheetsUpsertHrRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsUpsertHrRequestBuilder {
    employee_id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    days: Option<Vec<TimesheetsUpsertHrRequestDaysItem>>,
}

impl TimesheetsUpsertHrRequestBuilder {
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

    pub fn days(mut self, value: Vec<TimesheetsUpsertHrRequestDaysItem>) -> Self {
        self.days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsUpsertHrRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](TimesheetsUpsertHrRequestBuilder::employee_id)
    /// - [`year`](TimesheetsUpsertHrRequestBuilder::year)
    /// - [`month`](TimesheetsUpsertHrRequestBuilder::month)
    /// - [`days`](TimesheetsUpsertHrRequestBuilder::days)
    pub fn build(self) -> Result<TimesheetsUpsertHrRequest, BuildError> {
        Ok(TimesheetsUpsertHrRequest {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            days: self.days.ok_or_else(|| BuildError::missing_field("days"))?,
        })
    }
}
