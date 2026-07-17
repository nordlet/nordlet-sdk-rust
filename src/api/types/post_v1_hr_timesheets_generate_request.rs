pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrTimesheetsGenerateRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(rename = "employeeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
}

impl PostV1HrTimesheetsGenerateRequest {
    pub fn builder() -> PostV1HrTimesheetsGenerateRequestBuilder {
        <PostV1HrTimesheetsGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrTimesheetsGenerateRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    employee_id: Option<String>,
}

impl PostV1HrTimesheetsGenerateRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostV1HrTimesheetsGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1HrTimesheetsGenerateRequestBuilder::year)
    /// - [`month`](PostV1HrTimesheetsGenerateRequestBuilder::month)
    pub fn build(self) -> Result<PostV1HrTimesheetsGenerateRequest, BuildError> {
        Ok(PostV1HrTimesheetsGenerateRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            employee_id: self.employee_id,
        })
    }
}
