pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimesheetsUpsertHrResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "employeeName")]
    #[serde(default)]
    pub employee_name: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub days: Vec<TimesheetsUpsertHrResponseDaysItem>,
    #[serde(rename = "workedDays")]
    #[serde(default)]
    pub worked_days: String,
    #[serde(rename = "workedHours")]
    #[serde(default)]
    pub worked_hours: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl TimesheetsUpsertHrResponse {
    pub fn builder() -> TimesheetsUpsertHrResponseBuilder {
        <TimesheetsUpsertHrResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimesheetsUpsertHrResponseBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    employee_name: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    days: Option<Vec<TimesheetsUpsertHrResponseDaysItem>>,
    worked_days: Option<String>,
    worked_hours: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl TimesheetsUpsertHrResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn employee_name(mut self, value: impl Into<String>) -> Self {
        self.employee_name = Some(value.into());
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

    pub fn days(mut self, value: Vec<TimesheetsUpsertHrResponseDaysItem>) -> Self {
        self.days = Some(value);
        self
    }

    pub fn worked_days(mut self, value: impl Into<String>) -> Self {
        self.worked_days = Some(value.into());
        self
    }

    pub fn worked_hours(mut self, value: impl Into<String>) -> Self {
        self.worked_hours = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimesheetsUpsertHrResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimesheetsUpsertHrResponseBuilder::id)
    /// - [`employee_id`](TimesheetsUpsertHrResponseBuilder::employee_id)
    /// - [`employee_name`](TimesheetsUpsertHrResponseBuilder::employee_name)
    /// - [`year`](TimesheetsUpsertHrResponseBuilder::year)
    /// - [`month`](TimesheetsUpsertHrResponseBuilder::month)
    /// - [`days`](TimesheetsUpsertHrResponseBuilder::days)
    /// - [`worked_days`](TimesheetsUpsertHrResponseBuilder::worked_days)
    /// - [`worked_hours`](TimesheetsUpsertHrResponseBuilder::worked_hours)
    /// - [`updated_at`](TimesheetsUpsertHrResponseBuilder::updated_at)
    pub fn build(self) -> Result<TimesheetsUpsertHrResponse, BuildError> {
        Ok(TimesheetsUpsertHrResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            employee_name: self
                .employee_name
                .ok_or_else(|| BuildError::missing_field("employee_name"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            days: self.days.ok_or_else(|| BuildError::missing_field("days"))?,
            worked_days: self
                .worked_days
                .ok_or_else(|| BuildError::missing_field("worked_days"))?,
            worked_hours: self
                .worked_hours
                .ok_or_else(|| BuildError::missing_field("worked_hours"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
