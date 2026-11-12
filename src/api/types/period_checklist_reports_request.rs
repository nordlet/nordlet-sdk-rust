pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PeriodChecklistReportsRequest {
    #[serde(default)]
    pub period: String,
}

impl PeriodChecklistReportsRequest {
    pub fn builder() -> PeriodChecklistReportsRequestBuilder {
        <PeriodChecklistReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeriodChecklistReportsRequestBuilder {
    period: Option<String>,
}

impl PeriodChecklistReportsRequestBuilder {
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PeriodChecklistReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period`](PeriodChecklistReportsRequestBuilder::period)
    pub fn build(self) -> Result<PeriodChecklistReportsRequest, BuildError> {
        Ok(PeriodChecklistReportsRequest {
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
        })
    }
}
