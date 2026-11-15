pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FxRevaluationRunLedgerResponse {
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(rename = "reversalDate")]
    #[serde(default)]
    pub reversal_date: NaiveDate,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(rename = "reversalJournalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_journal_transaction_id: Option<String>,
    #[serde(default)]
    pub gain: String,
    #[serde(default)]
    pub loss: String,
    #[serde(default)]
    pub lines: Vec<FxRevaluationRunLedgerResponseLinesItem>,
}

impl FxRevaluationRunLedgerResponse {
    pub fn builder() -> FxRevaluationRunLedgerResponseBuilder {
        <FxRevaluationRunLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FxRevaluationRunLedgerResponseBuilder {
    run_id: Option<String>,
    date: Option<NaiveDate>,
    reversal_date: Option<NaiveDate>,
    journal_transaction_id: Option<String>,
    reversal_journal_transaction_id: Option<String>,
    gain: Option<String>,
    loss: Option<String>,
    lines: Option<Vec<FxRevaluationRunLedgerResponseLinesItem>>,
}

impl FxRevaluationRunLedgerResponseBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn reversal_date(mut self, value: NaiveDate) -> Self {
        self.reversal_date = Some(value);
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn reversal_journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.reversal_journal_transaction_id = Some(value.into());
        self
    }

    pub fn gain(mut self, value: impl Into<String>) -> Self {
        self.gain = Some(value.into());
        self
    }

    pub fn loss(mut self, value: impl Into<String>) -> Self {
        self.loss = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<FxRevaluationRunLedgerResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FxRevaluationRunLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](FxRevaluationRunLedgerResponseBuilder::date)
    /// - [`reversal_date`](FxRevaluationRunLedgerResponseBuilder::reversal_date)
    /// - [`gain`](FxRevaluationRunLedgerResponseBuilder::gain)
    /// - [`loss`](FxRevaluationRunLedgerResponseBuilder::loss)
    /// - [`lines`](FxRevaluationRunLedgerResponseBuilder::lines)
    pub fn build(self) -> Result<FxRevaluationRunLedgerResponse, BuildError> {
        Ok(FxRevaluationRunLedgerResponse {
            run_id: self.run_id,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            reversal_date: self
                .reversal_date
                .ok_or_else(|| BuildError::missing_field("reversal_date"))?,
            journal_transaction_id: self.journal_transaction_id,
            reversal_journal_transaction_id: self.reversal_journal_transaction_id,
            gain: self.gain.ok_or_else(|| BuildError::missing_field("gain"))?,
            loss: self.loss.ok_or_else(|| BuildError::missing_field("loss"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
