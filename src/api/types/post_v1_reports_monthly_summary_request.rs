pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsMonthlySummaryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<i64>,
}

impl PostV1ReportsMonthlySummaryRequest {
    pub fn builder() -> PostV1ReportsMonthlySummaryRequestBuilder {
        <PostV1ReportsMonthlySummaryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsMonthlySummaryRequestBuilder {
    months: Option<i64>,
}

impl PostV1ReportsMonthlySummaryRequestBuilder {
    pub fn months(mut self, value: i64) -> Self {
        self.months = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsMonthlySummaryRequest`].
    pub fn build(self) -> Result<PostV1ReportsMonthlySummaryRequest, BuildError> {
        Ok(PostV1ReportsMonthlySummaryRequest {
            months: self.months,
        })
    }
}
