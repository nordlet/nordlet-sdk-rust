pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JournalTransactionsCreateLedgerRequestEntriesItem {
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "costCenterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl JournalTransactionsCreateLedgerRequestEntriesItem {
    pub fn builder() -> JournalTransactionsCreateLedgerRequestEntriesItemBuilder {
        <JournalTransactionsCreateLedgerRequestEntriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JournalTransactionsCreateLedgerRequestEntriesItemBuilder {
    account_code: Option<String>,
    cost_center_id: Option<String>,
    project_id: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
    description: Option<String>,
}

impl JournalTransactionsCreateLedgerRequestEntriesItemBuilder {
    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn cost_center_id(mut self, value: impl Into<String>) -> Self {
        self.cost_center_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`JournalTransactionsCreateLedgerRequestEntriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_code`](JournalTransactionsCreateLedgerRequestEntriesItemBuilder::account_code)
    pub fn build(self) -> Result<JournalTransactionsCreateLedgerRequestEntriesItem, BuildError> {
        Ok(JournalTransactionsCreateLedgerRequestEntriesItem {
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            cost_center_id: self.cost_center_id,
            project_id: self.project_id,
            debit: self.debit,
            credit: self.credit,
            description: self.description,
        })
    }
}
