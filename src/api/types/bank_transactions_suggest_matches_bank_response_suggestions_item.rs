pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionsSuggestMatchesBankResponseSuggestionsItem {
    #[serde(rename = "documentType")]
    pub document_type: TransactionsSuggestMatchesBankResponseSuggestionsItemDocumentType,
    #[serde(rename = "documentId")]
    #[serde(default)]
    pub document_id: String,
    #[serde(default)]
    pub number: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(default)]
    pub remaining: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub reasons: Vec<String>,
}

impl TransactionsSuggestMatchesBankResponseSuggestionsItem {
    pub fn builder() -> TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder {
        <TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder {
    document_type: Option<TransactionsSuggestMatchesBankResponseSuggestionsItemDocumentType>,
    document_id: Option<String>,
    number: Option<String>,
    partner_name: Option<String>,
    currency: Option<String>,
    gross_total: Option<String>,
    remaining: Option<String>,
    score: Option<i64>,
    reasons: Option<Vec<String>>,
}

impl TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder {
    pub fn document_type(
        mut self,
        value: TransactionsSuggestMatchesBankResponseSuggestionsItemDocumentType,
    ) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn partner_name(mut self, value: impl Into<String>) -> Self {
        self.partner_name = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    pub fn score(mut self, value: i64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn reasons(mut self, value: Vec<String>) -> Self {
        self.reasons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsSuggestMatchesBankResponseSuggestionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_type`](TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder::document_type)
    /// - [`document_id`](TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder::document_id)
    /// - [`number`](TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder::number)
    /// - [`partner_name`](TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder::partner_name)
    /// - [`currency`](TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder::currency)
    /// - [`gross_total`](TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder::gross_total)
    /// - [`remaining`](TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder::remaining)
    /// - [`score`](TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder::score)
    /// - [`reasons`](TransactionsSuggestMatchesBankResponseSuggestionsItemBuilder::reasons)
    pub fn build(
        self,
    ) -> Result<TransactionsSuggestMatchesBankResponseSuggestionsItem, BuildError> {
        Ok(TransactionsSuggestMatchesBankResponseSuggestionsItem {
            document_type: self
                .document_type
                .ok_or_else(|| BuildError::missing_field("document_type"))?,
            document_id: self
                .document_id
                .ok_or_else(|| BuildError::missing_field("document_id"))?,
            number: self
                .number
                .ok_or_else(|| BuildError::missing_field("number"))?,
            partner_name: self
                .partner_name
                .ok_or_else(|| BuildError::missing_field("partner_name"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
            score: self
                .score
                .ok_or_else(|| BuildError::missing_field("score"))?,
            reasons: self
                .reasons
                .ok_or_else(|| BuildError::missing_field("reasons"))?,
        })
    }
}
