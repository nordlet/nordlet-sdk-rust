pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MonthlySummaryReportsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months: Option<i64>,
}

impl MonthlySummaryReportsRequest {
    pub fn builder() -> MonthlySummaryReportsRequestBuilder {
        <MonthlySummaryReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MonthlySummaryReportsRequestBuilder {
    months: Option<i64>,
}

impl MonthlySummaryReportsRequestBuilder {
    pub fn months(mut self, value: i64) -> Self {
        self.months = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MonthlySummaryReportsRequest`].
    pub fn build(self) -> Result<MonthlySummaryReportsRequest, BuildError> {
        Ok(MonthlySummaryReportsRequest {
            months: self.months,
        })
    }
}
