pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FxRevaluationReverseLedgerResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "undoJournalTransactionIds")]
    #[serde(default)]
    pub undo_journal_transaction_ids: Vec<String>,
}

impl FxRevaluationReverseLedgerResponse {
    pub fn builder() -> FxRevaluationReverseLedgerResponseBuilder {
        <FxRevaluationReverseLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FxRevaluationReverseLedgerResponseBuilder {
    id: Option<String>,
    undo_journal_transaction_ids: Option<Vec<String>>,
}

impl FxRevaluationReverseLedgerResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn undo_journal_transaction_ids(mut self, value: Vec<String>) -> Self {
        self.undo_journal_transaction_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FxRevaluationReverseLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FxRevaluationReverseLedgerResponseBuilder::id)
    /// - [`undo_journal_transaction_ids`](FxRevaluationReverseLedgerResponseBuilder::undo_journal_transaction_ids)
    pub fn build(self) -> Result<FxRevaluationReverseLedgerResponse, BuildError> {
        Ok(FxRevaluationReverseLedgerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            undo_journal_transaction_ids: self
                .undo_journal_transaction_ids
                .ok_or_else(|| BuildError::missing_field("undo_journal_transaction_ids"))?,
        })
    }
}
