pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrTimesheetsGetRequest {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PostV1HrTimesheetsGetRequest {
    pub fn builder() -> PostV1HrTimesheetsGetRequestBuilder {
        <PostV1HrTimesheetsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrTimesheetsGetRequestBuilder {
    employee_id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
}

impl PostV1HrTimesheetsGetRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1HrTimesheetsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1HrTimesheetsGetRequestBuilder::employee_id)
    /// - [`year`](PostV1HrTimesheetsGetRequestBuilder::year)
    /// - [`month`](PostV1HrTimesheetsGetRequestBuilder::month)
    pub fn build(self) -> Result<PostV1HrTimesheetsGetRequest, BuildError> {
        Ok(PostV1HrTimesheetsGetRequest {
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
