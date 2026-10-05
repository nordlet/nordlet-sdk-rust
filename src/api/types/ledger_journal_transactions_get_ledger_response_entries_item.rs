pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JournalTransactionsGetLedgerResponseEntriesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "accountName")]
    #[serde(default)]
    pub account_name: String,
    #[serde(rename = "costCenterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(default)]
    pub debit: String,
    #[serde(default)]
    pub credit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl JournalTransactionsGetLedgerResponseEntriesItem {
    pub fn builder() -> JournalTransactionsGetLedgerResponseEntriesItemBuilder {
        <JournalTransactionsGetLedgerResponseEntriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JournalTransactionsGetLedgerResponseEntriesItemBuilder {
    id: Option<String>,
    account_id: Option<String>,
    account_code: Option<String>,
    account_name: Option<String>,
    cost_center_id: Option<String>,
    project_id: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
    description: Option<String>,
}

impl JournalTransactionsGetLedgerResponseEntriesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn account_name(mut self, value: impl Into<String>) -> Self {
        self.account_name = Some(value.into());
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

    /// Consumes the builder and constructs a [`JournalTransactionsGetLedgerResponseEntriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](JournalTransactionsGetLedgerResponseEntriesItemBuilder::id)
    /// - [`account_id`](JournalTransactionsGetLedgerResponseEntriesItemBuilder::account_id)
    /// - [`account_code`](JournalTransactionsGetLedgerResponseEntriesItemBuilder::account_code)
    /// - [`account_name`](JournalTransactionsGetLedgerResponseEntriesItemBuilder::account_name)
    /// - [`debit`](JournalTransactionsGetLedgerResponseEntriesItemBuilder::debit)
    /// - [`credit`](JournalTransactionsGetLedgerResponseEntriesItemBuilder::credit)
    pub fn build(self) -> Result<JournalTransactionsGetLedgerResponseEntriesItem, BuildError> {
        Ok(JournalTransactionsGetLedgerResponseEntriesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            account_name: self
                .account_name
                .ok_or_else(|| BuildError::missing_field("account_name"))?,
            cost_center_id: self.cost_center_id,
            project_id: self.project_id,
            debit: self
                .debit
                .ok_or_else(|| BuildError::missing_field("debit"))?,
            credit: self
                .credit
                .ok_or_else(|| BuildError::missing_field("credit"))?,
            description: self.description,
        })
    }
}
