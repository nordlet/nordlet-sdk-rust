pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionsMatchManyBankRequestAllocationsItem {
    #[serde(rename = "documentType")]
    pub document_type: TransactionsMatchManyBankRequestAllocationsItemDocumentType,
    #[serde(rename = "documentId")]
    #[serde(default)]
    pub document_id: String,
    #[serde(default)]
    pub amount: String,
}

impl TransactionsMatchManyBankRequestAllocationsItem {
    pub fn builder() -> TransactionsMatchManyBankRequestAllocationsItemBuilder {
        <TransactionsMatchManyBankRequestAllocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsMatchManyBankRequestAllocationsItemBuilder {
    document_type: Option<TransactionsMatchManyBankRequestAllocationsItemDocumentType>,
    document_id: Option<String>,
    amount: Option<String>,
}

impl TransactionsMatchManyBankRequestAllocationsItemBuilder {
    pub fn document_type(
        mut self,
        value: TransactionsMatchManyBankRequestAllocationsItemDocumentType,
    ) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransactionsMatchManyBankRequestAllocationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_type`](TransactionsMatchManyBankRequestAllocationsItemBuilder::document_type)
    /// - [`document_id`](TransactionsMatchManyBankRequestAllocationsItemBuilder::document_id)
    /// - [`amount`](TransactionsMatchManyBankRequestAllocationsItemBuilder::amount)
    pub fn build(self) -> Result<TransactionsMatchManyBankRequestAllocationsItem, BuildError> {
        Ok(TransactionsMatchManyBankRequestAllocationsItem {
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            document_id: self
                .document_id
                .ok_or_else(|| BuildError::missing_field("document_id"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
