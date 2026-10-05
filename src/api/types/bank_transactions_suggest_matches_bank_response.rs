pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionsSuggestMatchesBankResponse {
    #[serde(default)]
    pub suggestions: Vec<TransactionsSuggestMatchesBankResponseSuggestionsItem>,
}

impl TransactionsSuggestMatchesBankResponse {
    pub fn builder() -> TransactionsSuggestMatchesBankResponseBuilder {
        <TransactionsSuggestMatchesBankResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionsSuggestMatchesBankResponseBuilder {
    suggestions: Option<Vec<TransactionsSuggestMatchesBankResponseSuggestionsItem>>,
}

impl TransactionsSuggestMatchesBankResponseBuilder {
    pub fn suggestions(
        mut self,
        value: Vec<TransactionsSuggestMatchesBankResponseSuggestionsItem>,
    ) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionsSuggestMatchesBankResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`suggestions`](TransactionsSuggestMatchesBankResponseBuilder::suggestions)
    pub fn build(self) -> Result<TransactionsSuggestMatchesBankResponse, BuildError> {
        Ok(TransactionsSuggestMatchesBankResponse {
            suggestions: self
                .suggestions
                .ok_or_else(|| BuildError::missing_field("suggestions"))?,
        })
    }
}
