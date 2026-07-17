pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsGeneralJournalResponseRowsItemEntriesItem {
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "accountName")]
    #[serde(default)]
    pub account_name: String,
    #[serde(default)]
    pub debit: String,
    #[serde(default)]
    pub credit: String,
}

impl PostV1ReportsGeneralJournalResponseRowsItemEntriesItem {
    pub fn builder() -> PostV1ReportsGeneralJournalResponseRowsItemEntriesItemBuilder {
        <PostV1ReportsGeneralJournalResponseRowsItemEntriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsGeneralJournalResponseRowsItemEntriesItemBuilder {
    account_code: Option<String>,
    account_name: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
}

impl PostV1ReportsGeneralJournalResponseRowsItemEntriesItemBuilder {
    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn account_name(mut self, value: impl Into<String>) -> Self {
        self.account_name = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ReportsGeneralJournalResponseRowsItemEntriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_code`](PostV1ReportsGeneralJournalResponseRowsItemEntriesItemBuilder::account_code)
    /// - [`account_name`](PostV1ReportsGeneralJournalResponseRowsItemEntriesItemBuilder::account_name)
    /// - [`debit`](PostV1ReportsGeneralJournalResponseRowsItemEntriesItemBuilder::debit)
    /// - [`credit`](PostV1ReportsGeneralJournalResponseRowsItemEntriesItemBuilder::credit)
    pub fn build(
        self,
    ) -> Result<PostV1ReportsGeneralJournalResponseRowsItemEntriesItem, BuildError> {
        Ok(PostV1ReportsGeneralJournalResponseRowsItemEntriesItem {
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            account_name: self
                .account_name
                .ok_or_else(|| BuildError::missing_field("account_name"))?,
            debit: self
                .debit
                .ok_or_else(|| BuildError::missing_field("debit"))?,
            credit: self
                .credit
                .ok_or_else(|| BuildError::missing_field("credit"))?,
        })
    }
}
