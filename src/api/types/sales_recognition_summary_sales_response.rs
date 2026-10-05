pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionSummarySalesResponse {
    #[serde(default)]
    pub rows: Vec<RecognitionSummarySalesResponseRowsItem>,
    #[serde(default)]
    pub totals: RecognitionSummarySalesResponseTotals,
}

impl RecognitionSummarySalesResponse {
    pub fn builder() -> RecognitionSummarySalesResponseBuilder {
        <RecognitionSummarySalesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionSummarySalesResponseBuilder {
    rows: Option<Vec<RecognitionSummarySalesResponseRowsItem>>,
    totals: Option<RecognitionSummarySalesResponseTotals>,
}

impl RecognitionSummarySalesResponseBuilder {
    pub fn rows(mut self, value: Vec<RecognitionSummarySalesResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn totals(mut self, value: RecognitionSummarySalesResponseTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionSummarySalesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](RecognitionSummarySalesResponseBuilder::rows)
    /// - [`totals`](RecognitionSummarySalesResponseBuilder::totals)
    pub fn build(self) -> Result<RecognitionSummarySalesResponse, BuildError> {
        Ok(RecognitionSummarySalesResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
