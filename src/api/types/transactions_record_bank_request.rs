pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionsRecordBankRequest {
    #[serde(rename = "bankAccountId")]
    #[serde(default)]
    pub bank_account_id: String,
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "documentType")]
    pub document_type: TransactionsRecordBankRequestDocumentType,
    #[serde(rename = "documentId")]
    #[serde(default)]
    pub document_id: String,
}

impl TransactionsRecordBankRequest {
    pub fn builder() -> TransactionsRecordBankRequestBuilder {
        <TransactionsRecordBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsRecordBankRequestBuilder {
    bank_account_id: Option<String>,
    date: Option<NaiveDate>,
    amount: Option<String>,
    description: Option<String>,
    document_type: Option<TransactionsRecordBankRequestDocumentType>,
    document_id: Option<String>,
}

impl TransactionsRecordBankRequestBuilder {
    pub fn bank_account_id(mut self, value: impl Into<String>) -> Self {
        self.bank_account_id = Some(value.into());
        self
    }

    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn document_type(mut self, value: TransactionsRecordBankRequestDocumentType) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransactionsRecordBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bank_account_id`](TransactionsRecordBankRequestBuilder::bank_account_id)
    /// - [`date`](TransactionsRecordBankRequestBuilder::date)
    /// - [`amount`](TransactionsRecordBankRequestBuilder::amount)
    /// - [`document_type`](TransactionsRecordBankRequestBuilder::document_type)
    /// - [`document_id`](TransactionsRecordBankRequestBuilder::document_id)
    pub fn build(self) -> Result<TransactionsRecordBankRequest, BuildError> {
        Ok(TransactionsRecordBankRequest {
            bank_account_id: self
                .bank_account_id
                .ok_or_else(|| BuildError::missing_field("bank_account_id"))?,
            date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self.description,
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            document_id: self
                .document_id
                .ok_or_else(|| BuildError::missing_field("document_id"))?,
        })
    }
}
