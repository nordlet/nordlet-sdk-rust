pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerJournalTransactionsCreateRequestEntriesItem {
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "costCenterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1LedgerJournalTransactionsCreateRequestEntriesItem {
    pub fn builder() -> PostV1LedgerJournalTransactionsCreateRequestEntriesItemBuilder {
        <PostV1LedgerJournalTransactionsCreateRequestEntriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerJournalTransactionsCreateRequestEntriesItemBuilder {
    account_code: Option<String>,
    cost_center_id: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
    description: Option<String>,
}

impl PostV1LedgerJournalTransactionsCreateRequestEntriesItemBuilder {
    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn cost_center_id(mut self, value: impl Into<String>) -> Self {
        self.cost_center_id = Some(value.into());
        self
    }

    pub fn debit(mut self, value: impl Into<String>) -> Self {
        self.debit = Some(value.into());
        self
    }

    pub fn credit(mut self, value: impl Into<String>) -> Self {
        self.credit = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerJournalTransactionsCreateRequestEntriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_code`](PostV1LedgerJournalTransactionsCreateRequestEntriesItemBuilder::account_code)
    pub fn build(
        self,
    ) -> Result<PostV1LedgerJournalTransactionsCreateRequestEntriesItem, BuildError> {
        Ok(PostV1LedgerJournalTransactionsCreateRequestEntriesItem {
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            cost_center_id: self.cost_center_id,
            debit: self.debit,
            credit: self.credit,
            description: self.description,
        })
    }
}
