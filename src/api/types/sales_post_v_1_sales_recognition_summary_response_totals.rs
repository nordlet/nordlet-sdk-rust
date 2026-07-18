pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1SalesRecognitionSummaryResponseTotals {
    #[serde(rename = "deferredTotal")]
    #[serde(default)]
    pub deferred_total: String,
    #[serde(rename = "recognizedToDate")]
    #[serde(default)]
    pub recognized_to_date: String,
    #[serde(default)]
    pub remaining: String,
}

impl PostV1SalesRecognitionSummaryResponseTotals {
    pub fn builder() -> PostV1SalesRecognitionSummaryResponseTotalsBuilder {
        <PostV1SalesRecognitionSummaryResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1SalesRecognitionSummaryResponseTotalsBuilder {
    deferred_total: Option<String>,
    recognized_to_date: Option<String>,
    remaining: Option<String>,
}

impl PostV1SalesRecognitionSummaryResponseTotalsBuilder {
    pub fn deferred_total(mut self, value: impl Into<String>) -> Self {
        self.deferred_total = Some(value.into());
        self
    }

    pub fn recognized_to_date(mut self, value: impl Into<String>) -> Self {
        self.recognized_to_date = Some(value.into());
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1SalesRecognitionSummaryResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deferred_total`](PostV1SalesRecognitionSummaryResponseTotalsBuilder::deferred_total)
    /// - [`recognized_to_date`](PostV1SalesRecognitionSummaryResponseTotalsBuilder::recognized_to_date)
    /// - [`remaining`](PostV1SalesRecognitionSummaryResponseTotalsBuilder::remaining)
    pub fn build(self) -> Result<PostV1SalesRecognitionSummaryResponseTotals, BuildError> {
        Ok(PostV1SalesRecognitionSummaryResponseTotals {
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
