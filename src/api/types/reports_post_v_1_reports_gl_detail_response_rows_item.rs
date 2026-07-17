pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsGlDetailResponseRowsItem {
    #[serde(default)]
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "documentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "documentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "journalTransactionId")]
    #[serde(default)]
    pub journal_transaction_id: String,
    #[serde(default)]
    pub debit: String,
    #[serde(default)]
    pub credit: String,
    #[serde(default)]
    pub balance: String,
}

impl PostV1ReportsGlDetailResponseRowsItem {
    pub fn builder() -> PostV1ReportsGlDetailResponseRowsItemBuilder {
        <PostV1ReportsGlDetailResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsGlDetailResponseRowsItemBuilder {
    date: Option<String>,
    description: Option<String>,
    document_type: Option<String>,
    document_id: Option<String>,
    journal_transaction_id: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
    balance: Option<String>,
}

impl PostV1ReportsGlDetailResponseRowsItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn document_type(mut self, value: impl Into<String>) -> Self {
        self.document_type = Some(value.into());
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
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

    pub fn balance(mut self, value: impl Into<String>) -> Self {
        self.balance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsGlDetailResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](PostV1ReportsGlDetailResponseRowsItemBuilder::date)
    /// - [`journal_transaction_id`](PostV1ReportsGlDetailResponseRowsItemBuilder::journal_transaction_id)
    /// - [`debit`](PostV1ReportsGlDetailResponseRowsItemBuilder::debit)
    /// - [`credit`](PostV1ReportsGlDetailResponseRowsItemBuilder::credit)
    /// - [`balance`](PostV1ReportsGlDetailResponseRowsItemBuilder::balance)
    pub fn build(self) -> Result<PostV1ReportsGlDetailResponseRowsItem, BuildError> {
        Ok(PostV1ReportsGlDetailResponseRowsItem {
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            description: self.description,
            document_type: self.document_type,
            document_id: self.document_id,
            journal_transaction_id: self
                .journal_transaction_id
                .ok_or_else(|| BuildError::missing_field("journal_transaction_id"))?,
            debit: self
                .debit
                .ok_or_else(|| BuildError::missing_field("debit"))?,
            credit: self
                .credit
                .ok_or_else(|| BuildError::missing_field("credit"))?,
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
        })
    }
}
