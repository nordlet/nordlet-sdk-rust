pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionsMatchBankRequest {
    #[serde(rename = "transactionId")]
    #[serde(default)]
    pub transaction_id: String,
    #[serde(rename = "documentType")]
    pub document_type: TransactionsMatchBankRequestDocumentType,
    #[serde(rename = "documentId")]
    #[serde(default)]
    pub document_id: String,
    #[serde(rename = "invoiceAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_amount: Option<String>,
}

impl TransactionsMatchBankRequest {
    pub fn builder() -> TransactionsMatchBankRequestBuilder {
        <TransactionsMatchBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsMatchBankRequestBuilder {
    transaction_id: Option<String>,
    document_type: Option<TransactionsMatchBankRequestDocumentType>,
    document_id: Option<String>,
    invoice_amount: Option<String>,
}

impl TransactionsMatchBankRequestBuilder {
    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn document_type(mut self, value: TransactionsMatchBankRequestDocumentType) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn invoice_amount(mut self, value: impl Into<String>) -> Self {
        self.invoice_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransactionsMatchBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_id`](TransactionsMatchBankRequestBuilder::transaction_id)
    /// - [`document_type`](TransactionsMatchBankRequestBuilder::document_type)
    /// - [`document_id`](TransactionsMatchBankRequestBuilder::document_id)
    pub fn build(self) -> Result<TransactionsMatchBankRequest, BuildError> {
        Ok(TransactionsMatchBankRequest {
            transaction_id: self
                .transaction_id
                .ok_or_else(|| BuildError::missing_field("transaction_id"))?,
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            document_id: self
                .document_id
                .ok_or_else(|| BuildError::missing_field("document_id"))?,
            invoice_amount: self.invoice_amount,
        })
    }
}
