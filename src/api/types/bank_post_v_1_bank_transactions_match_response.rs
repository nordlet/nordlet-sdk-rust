pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankTransactionsMatchResponse {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "counterpartyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty_name: Option<String>,
    #[serde(rename = "counterpartyIban")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty_iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub status: PostV1BankTransactionsMatchResponseStatus,
    #[serde(rename = "matchedDocumentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_document_type: Option<String>,
    #[serde(rename = "matchedDocumentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_document_id: Option<String>,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1BankTransactionsMatchResponse {
    pub fn builder() -> PostV1BankTransactionsMatchResponseBuilder {
        <PostV1BankTransactionsMatchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankTransactionsMatchResponseBuilder {
    id: Option<String>,
    bank_account_id: Option<String>,
    date: Option<String>,
    amount: Option<String>,
    currency: Option<String>,
    counterparty_name: Option<String>,
    counterparty_iban: Option<String>,
    description: Option<String>,
    external_id: Option<String>,
    status: Option<PostV1BankTransactionsMatchResponseStatus>,
    matched_document_type: Option<String>,
    matched_document_id: Option<String>,
    journal_transaction_id: Option<String>,
    created_at: Option<String>,
}

impl PostV1BankTransactionsMatchResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn counterparty_name(mut self, value: impl Into<String>) -> Self {
        self.counterparty_name = Some(value.into());
        self
    }

    pub fn counterparty_iban(mut self, value: impl Into<String>) -> Self {
        self.counterparty_iban = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostV1BankTransactionsMatchResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn matched_document_type(mut self, value: impl Into<String>) -> Self {
        self.matched_document_type = Some(value.into());
        self
    }

    pub fn matched_document_id(mut self, value: impl Into<String>) -> Self {
        self.matched_document_id = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankTransactionsMatchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankTransactionsMatchResponseBuilder::id)
    /// - [`bank_account_id`](PostV1BankTransactionsMatchResponseBuilder::bank_account_id)
    /// - [`date`](PostV1BankTransactionsMatchResponseBuilder::date)
    /// - [`amount`](PostV1BankTransactionsMatchResponseBuilder::amount)
    /// - [`currency`](PostV1BankTransactionsMatchResponseBuilder::currency)
    /// - [`status`](PostV1BankTransactionsMatchResponseBuilder::status)
    /// - [`created_at`](PostV1BankTransactionsMatchResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1BankTransactionsMatchResponse, BuildError> {
        Ok(PostV1BankTransactionsMatchResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            counterparty_name: self.counterparty_name,
            counterparty_iban: self.counterparty_iban,
            description: self.description,
            external_id: self.external_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            matched_document_type: self.matched_document_type,
            matched_document_id: self.matched_document_id,
            journal_transaction_id: self.journal_transaction_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
