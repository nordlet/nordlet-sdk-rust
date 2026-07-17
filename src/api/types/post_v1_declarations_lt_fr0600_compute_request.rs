pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtFr0600ComputeRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<i64>,
    #[serde(rename = "deductionPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduction_percent: Option<i64>,
}

impl PostV1DeclarationsLtFr0600ComputeRequest {
    pub fn builder() -> PostV1DeclarationsLtFr0600ComputeRequestBuilder {
        <PostV1DeclarationsLtFr0600ComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtFr0600ComputeRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    months: Option<i64>,
    deduction_percent: Option<i64>,
}

impl PostV1DeclarationsLtFr0600ComputeRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn months(mut self, value: i64) -> Self {
        self.months = Some(value);
        self
    }

    pub fn deduction_percent(mut self, value: i64) -> Self {
        self.deduction_percent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtFr0600ComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtFr0600ComputeRequestBuilder::year)
    /// - [`month`](PostV1DeclarationsLtFr0600ComputeRequestBuilder::month)
    pub fn build(self) -> Result<PostV1DeclarationsLtFr0600ComputeRequest, BuildError> {
        Ok(PostV1DeclarationsLtFr0600ComputeRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            months: self.months,
            deduction_percent: self.deduction_percent,
        })
    }
}
