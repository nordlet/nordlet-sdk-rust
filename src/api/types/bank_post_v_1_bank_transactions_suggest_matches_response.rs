pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankTransactionsSuggestMatchesResponse {
    #[serde(default)]
    pub suggestions: Vec<PostV1BankTransactionsSuggestMatchesResponseSuggestionsItem>,
}

impl PostV1BankTransactionsSuggestMatchesResponse {
    pub fn builder() -> PostV1BankTransactionsSuggestMatchesResponseBuilder {
        <PostV1BankTransactionsSuggestMatchesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankTransactionsSuggestMatchesResponseBuilder {
    suggestions: Option<Vec<PostV1BankTransactionsSuggestMatchesResponseSuggestionsItem>>,
}

impl PostV1BankTransactionsSuggestMatchesResponseBuilder {
    pub fn suggestions(
        mut self,
        value: Vec<PostV1BankTransactionsSuggestMatchesResponseSuggestionsItem>,
    ) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankTransactionsSuggestMatchesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`suggestions`](PostV1BankTransactionsSuggestMatchesResponseBuilder::suggestions)
    pub fn build(self) -> Result<PostV1BankTransactionsSuggestMatchesResponse, BuildError> {
        Ok(PostV1BankTransactionsSuggestMatchesResponse {
            suggestions: self
                .suggestions
                .ok_or_else(|| BuildError::missing_field("suggestions"))?,
        })
    }
}
