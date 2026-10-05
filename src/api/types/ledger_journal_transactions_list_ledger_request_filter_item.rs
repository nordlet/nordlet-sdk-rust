pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JournalTransactionsListLedgerRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: JournalTransactionsListLedgerRequestFilterItemOp,
    pub value: JournalTransactionsListLedgerRequestFilterItemValue,
}

impl JournalTransactionsListLedgerRequestFilterItem {
    pub fn builder() -> JournalTransactionsListLedgerRequestFilterItemBuilder {
        <JournalTransactionsListLedgerRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JournalTransactionsListLedgerRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<JournalTransactionsListLedgerRequestFilterItemOp>,
    value: Option<JournalTransactionsListLedgerRequestFilterItemValue>,
}

impl JournalTransactionsListLedgerRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: JournalTransactionsListLedgerRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: JournalTransactionsListLedgerRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JournalTransactionsListLedgerRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](JournalTransactionsListLedgerRequestFilterItemBuilder::field)
    /// - [`op`](JournalTransactionsListLedgerRequestFilterItemBuilder::op)
    /// - [`value`](JournalTransactionsListLedgerRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<JournalTransactionsListLedgerRequestFilterItem, BuildError> {
        Ok(JournalTransactionsListLedgerRequestFilterItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
