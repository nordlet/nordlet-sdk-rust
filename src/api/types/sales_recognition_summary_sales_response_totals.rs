pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionSummarySalesResponseTotals {
    #[serde(rename = "deferredTotal")]
    #[serde(default)]
    pub deferred_total: String,
    #[serde(rename = "recognizedToDate")]
    #[serde(default)]
    pub recognized_to_date: NaiveDate,
    #[serde(default)]
    pub remaining: String,
}

impl RecognitionSummarySalesResponseTotals {
    pub fn builder() -> RecognitionSummarySalesResponseTotalsBuilder {
        <RecognitionSummarySalesResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionSummarySalesResponseTotalsBuilder {
    deferred_total: Option<String>,
    recognized_to_date: Option<NaiveDate>,
    remaining: Option<String>,
}

impl RecognitionSummarySalesResponseTotalsBuilder {
    pub fn deferred_total(mut self, value: impl Into<String>) -> Self {
        self.deferred_total = Some(value.into());
        self
    }

    pub fn recognized_to_date(mut self, value: NaiveDate) -> Self {
        self.recognized_to_date = Some(value);
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecognitionSummarySalesResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deferred_total`](RecognitionSummarySalesResponseTotalsBuilder::deferred_total)
    /// - [`recognized_to_date`](RecognitionSummarySalesResponseTotalsBuilder::recognized_to_date)
    /// - [`remaining`](RecognitionSummarySalesResponseTotalsBuilder::remaining)
    pub fn build(self) -> Result<RecognitionSummarySalesResponseTotals, BuildError> {
        Ok(RecognitionSummarySalesResponseTotals {
            deferred_total: self
                .deferred_total
                .ok_or_else(|| BuildError::missing_field("deferred_total"))?,
            recognized_to_date: self
                .recognized_to_date
                .ok_or_else(|| BuildError::missing_field("recognized_to_date"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
        })
    }
}
