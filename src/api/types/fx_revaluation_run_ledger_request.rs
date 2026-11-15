pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FxRevaluationRunLedgerRequest {
    #[serde(default)]
    pub date: NaiveDate,
}

impl FxRevaluationRunLedgerRequest {
    pub fn builder() -> FxRevaluationRunLedgerRequestBuilder {
        <FxRevaluationRunLedgerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FxRevaluationRunLedgerRequestBuilder {
    date: Option<NaiveDate>,
}

impl FxRevaluationRunLedgerRequestBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FxRevaluationRunLedgerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](FxRevaluationRunLedgerRequestBuilder::date)
    pub fn build(self) -> Result<FxRevaluationRunLedgerRequest, BuildError> {
        Ok(FxRevaluationRunLedgerRequest {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
        })
    }
}
