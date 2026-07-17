pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrTimesheetsUpsertRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(default)]
    pub days: Vec<PostV1HrTimesheetsUpsertRequestDaysItem>,
}

impl PostV1HrTimesheetsUpsertRequest {
    pub fn builder() -> PostV1HrTimesheetsUpsertRequestBuilder {
        <PostV1HrTimesheetsUpsertRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrTimesheetsUpsertRequestBuilder {
    employee_id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    days: Option<Vec<PostV1HrTimesheetsUpsertRequestDaysItem>>,
}

impl PostV1HrTimesheetsUpsertRequestBuilder {
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

    pub fn days(mut self, value: Vec<PostV1HrTimesheetsUpsertRequestDaysItem>) -> Self {
        self.days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrTimesheetsUpsertRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1HrTimesheetsUpsertRequestBuilder::employee_id)
    /// - [`year`](PostV1HrTimesheetsUpsertRequestBuilder::year)
    /// - [`month`](PostV1HrTimesheetsUpsertRequestBuilder::month)
    /// - [`days`](PostV1HrTimesheetsUpsertRequestBuilder::days)
    pub fn build(self) -> Result<PostV1HrTimesheetsUpsertRequest, BuildError> {
        Ok(PostV1HrTimesheetsUpsertRequest {
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
