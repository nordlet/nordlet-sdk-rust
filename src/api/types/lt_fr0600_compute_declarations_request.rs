pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtFr0600ComputeDeclarationsRequest {
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

impl LtFr0600ComputeDeclarationsRequest {
    pub fn builder() -> LtFr0600ComputeDeclarationsRequestBuilder {
        <LtFr0600ComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtFr0600ComputeDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
    months: Option<i64>,
    deduction_percent: Option<i64>,
}

impl LtFr0600ComputeDeclarationsRequestBuilder {
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

    /// Consumes the builder and constructs a [`LtFr0600ComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtFr0600ComputeDeclarationsRequestBuilder::year)
    /// - [`month`](LtFr0600ComputeDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<LtFr0600ComputeDeclarationsRequest, BuildError> {
        Ok(LtFr0600ComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            months: self.months,
            deduction_percent: self.deduction_percent,
        })
    }
}
