pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1BankTransactionsSuggestMatchesResponseSuggestionsItem {
    #[serde(rename = "documentType")]
    pub document_type: PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemDocumentType,
    #[serde(rename = "documentId")]
    #[serde(default)]
    pub document_id: String,
    #[serde(default)]
    pub number: String,
    #[serde(rename = "partnerName")]
    #[serde(default)]
    pub partner_name: String,
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

impl PostV1BankTransactionsSuggestMatchesResponseSuggestionsItem {
    pub fn builder() -> PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder {
        <PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder {
    document_type: Option<PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemDocumentType>,
    document_id: Option<String>,
    number: Option<String>,
    partner_name: Option<String>,
    gross_total: Option<String>,
    remaining: Option<String>,
    score: Option<i64>,
    reasons: Option<Vec<String>>,
}

impl PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder {
    pub fn document_type(
        mut self,
        value: PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemDocumentType,
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

    /// Consumes the builder and constructs a [`PostV1BankTransactionsSuggestMatchesResponseSuggestionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_type`](PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder::document_type)
    /// - [`document_id`](PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder::document_id)
    /// - [`number`](PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder::number)
    /// - [`partner_name`](PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder::partner_name)
    /// - [`gross_total`](PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder::gross_total)
    /// - [`remaining`](PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder::remaining)
    /// - [`score`](PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder::score)
    /// - [`reasons`](PostV1BankTransactionsSuggestMatchesResponseSuggestionsItemBuilder::reasons)
    pub fn build(
        self,
    ) -> Result<PostV1BankTransactionsSuggestMatchesResponseSuggestionsItem, BuildError> {
        Ok(
            PostV1BankTransactionsSuggestMatchesResponseSuggestionsItem {
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
            },
        )
    }
}
